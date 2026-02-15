use crate::utils::fan_speed::speed_parser;
use clap::{Args, Parser, Subcommand, ValueEnum};
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

#[derive(ValueEnum, Clone, Debug)]
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
    Set { device_id: String, alias: String },

    Remove { alias: String },

    List,
}

#[derive(Subcommand, Debug)]
pub enum GroupCommand {
    Create { name: String, devices: Vec<String> },

    Delete { name: String },

    Add { name: String, alias: String },

    Remove { name: String, alias: String },

    List,
}
