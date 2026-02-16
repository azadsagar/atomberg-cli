use crate::{config::model::Device, device::DeviceResults, protocol::Payload};
use tokio::sync::mpsc::Receiver;
use std::{collections::HashMap, time::Duration};
use tokio::{time::sleep};

pub async fn discover_devices(
    rx: &mut Receiver<Payload>,
    base_devices: &HashMap<String, Device>,
    duration: Duration,
) -> anyhow::Result<DeviceResults> {

    let timeout = sleep(duration);

    tokio::pin!(timeout);

    let mut devices: HashMap<String, Device> = base_devices.clone();
    let mut changed = false;

    loop {
        tokio::select! {
            _ = &mut timeout => break,
            Some(payload) = rx.recv() => {
                if let Payload::Becon(b) = payload {
                    //println!("Found device: {:?}", b);
                    let ip = b.ip;

                    match devices.get_mut(&b.device_id) {

                        Some(device) => {
                            if device.ip != ip {
                                device.ip = ip;
                                changed = true;
                            }
                        }

                        None => {
                            devices.insert(
                                b.device_id.clone(),
                                Device {
                                    alias: Some(b.device_id.clone()),
                                    id: b.device_id.clone(),
                                    ip,
                                    groups: Vec::new(),
                                }
                            );

                            changed = true;
                        }
                    }
                }
            }
        }
    }

    Ok(DeviceResults { devices, changed })
}
