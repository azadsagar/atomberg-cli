use crate::utils::fan_speed::{speed_parser, timer_parser};
use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "atomberg",
    version = "0.1.0",
    about = "Control Atomberg smart fans over local network"
)]
pub struct Cli {
    /// Path to config file
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    // Discover devices broadcasting becon on local network
    #[command(about = "Discover devices broadcasting becon on local network")]
    Discover(DiscoverArgs),

    // Send command to device or group
    #[command(about = "Send command to device or group")]
    Send(SendArgs),

    // Manage device aliases
    #[command(about = "Manage device aliases")]
    Alias(AliasArgs),

    // Manage device groups
    #[command(about = "Manage device groups")]
    Group(GroupArgs),
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum PowerState {
    On,
    Off,
}

#[derive(Args, Debug)]
pub struct DiscoverArgs {
    #[arg(
        long,
        default_value = "5",
        help = "Duration for which to listen for incoming device becons"
    )]
    pub interval: u64,

    #[arg(
        long,
        default_value = "false",
        help = "Save captured unique devices to config file"
    )]
    pub save: bool,
}

#[derive(Args, Debug)]
#[command(
    group(
        ArgGroup::new("target")
        .required(true)
        .multiple(false)
        .args(["alias", "group"])
    ),
    group(
        ArgGroup::new("action")
        .required(true)
        .args(["speed","led","power","timer","sleep"])
    )
)]
pub struct SendArgs {
    #[arg(long, help = "Friendly name of the device")]
    pub alias: Option<String>,

    #[arg(long, help = "Name of the group to send command to")]
    pub group: Option<String>,

    #[arg(long, value_parser = speed_parser, help = "Speed of the fan between 1 and 6")]
    pub speed: Option<u8>,

    #[arg(long, help = "Should the LED be on or off")]
    pub led: Option<PowerState>,

    #[arg(long, help = "Should the power be on or off")]
    pub power: Option<PowerState>,

    #[arg(long, help = "Auto reduce fan speed by 1 every 2 hours")]
    pub sleep: Option<PowerState>,

    #[arg(long, value_parser = timer_parser  ,help = "Set timer 0-Off, 1-1hr, 2-2hr, 3-3hr, 4-6hr")]
    pub timer: Option<u8>,
}

#[derive(Args, Debug)]
pub struct AliasArgs {
    #[command(subcommand)]
    pub command: AliasCommand,
}

#[derive(Args, Debug)]
pub struct GroupArgs {
    #[command(subcommand)]
    pub command: GroupCommand,
}

#[derive(Subcommand, Debug)]
pub enum AliasCommand {
    Set {
        #[arg(long, help = "Device ID")]
        device_id: String,
        #[arg(long, help = "Alias to set")]
        alias: String,
    },

    Rename {
        #[arg(long, help = "Current alias")]
        old: String,
        #[arg(long, help = "New alias")]
        new: String,
    },

    Remove {
        #[arg(long, help = "Alias to remove")]
        alias: String,
    },

    List,

    Show(AliasShowArgs),
}

#[derive(Subcommand, Debug)]
pub enum GroupCommand {
    Create {
        #[arg(long, help = "Group name")]
        name: String,
    },

    Rename {
        #[arg(long, help = "Current group name")]
        old: String,
        #[arg(long, help = "New group name")]
        new: String,
    },

    Delete {
        #[arg(long, help = "Group name")]
        name: String,
    },

    Add(GroupMembershipArgs),

    Remove(GroupMembershipArgs),

    List,

    Show {
        #[arg(long, help = "Group name")]
        name: String,
    },
}

#[derive(Args, Debug)]
#[command(
    group(
        ArgGroup::new("device_selector")
            .required(true)
            .multiple(false)
            .args(["device_id", "alias"])
    )
)]
pub struct AliasShowArgs {
    #[arg(long, help = "Device ID")]
    pub device_id: Option<String>,

    #[arg(long, help = "Alias")]
    pub alias: Option<String>,
}

#[derive(Args, Debug)]
#[command(
    group(
        ArgGroup::new("device_selector")
            .required(true)
            .multiple(false)
            .args(["device_id", "alias"])
    )
)]
pub struct GroupMembershipArgs {
    #[arg(long, help = "Group name")]
    pub name: String,

    #[arg(long, help = "Device ID")]
    pub device_id: Option<String>,

    #[arg(long, help = "Alias")]
    pub alias: Option<String>,
}
