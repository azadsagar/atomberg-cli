use std::collections::HashSet;

use anyhow::Result;
use tokio::sync::mpsc::Receiver;
use tokio::time::{Duration, sleep};

use crate::config::model::Device;
use crate::device::command::{self, CommandOptions};
use crate::device::state_match::{RequestedState, matches_expected};
use crate::protocol::{self, Payload};

#[derive(Debug, Default)]
pub struct SendWorkflowResult {
    pub timed_out: bool,
}

pub async fn send_and_wait_for_expected_state(
    rx: &mut Receiver<Payload>,
    device_list: Vec<Device>,
    command_port: &u16,
    command_options: &CommandOptions,
    expected_state: &RequestedState,
    timeout_duration: Duration,
) -> Result<SendWorkflowResult> {
    let mut pending_device_ids = HashSet::new();

    for device in device_list {
        command::send(&device.ip, command_port, command_options).await?;
        pending_device_ids.insert(device.id.to_uppercase());
    }

    let timeout = sleep(timeout_duration);
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            _ = &mut timeout => {
                break;
            }
            Some(payload) = rx.recv() => {
                if let protocol::Payload::Status(state) = payload {
                    let state_device_id = state.device_id.to_uppercase();

                    if pending_device_ids.contains(&state_device_id)
                        && state.message_id != "internet_query"
                        && state.state_string.contains(',')
                    {
                        let first_field = state
                            .state_string
                            .split(',')
                            .next()
                            .ok_or_else(|| anyhow::anyhow!("Invalid State String"))?;

                        let v: u32 = first_field.parse()?;
                        let device_state = protocol::status::decode_device_state(v);

                        if expected_state.timer.is_some() {
                            println!("Timer elapsed mins is {}", &device_state.timer_elapsed_mins);
                        }

                        if matches_expected(&device_state, expected_state) {
                            pending_device_ids.remove(&state_device_id);
                        }
                    }
                }

                if pending_device_ids.is_empty() {
                    return Ok(SendWorkflowResult::default());
                }
            }
        }
    }

    Ok(SendWorkflowResult { timed_out: true })
}
