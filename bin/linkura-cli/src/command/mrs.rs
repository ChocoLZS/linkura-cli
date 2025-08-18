use anyhow::Result;
use clap::{Args as ClapArgs};
use crate::config::Global;

#[derive(Debug, ClapArgs)]
pub struct ArgsMRS {
    #[clap(short('a'), long = "address", value_name = "ADDRESS")]
    pub addr: Option<String>,
    #[clap(
        short('p'),
        long = "port",
        value_name = "PORT",
        default_value_t = 21011
    )]
    pub port: u16,
    #[clap(short('r'), long = "room-id", value_name = "ROOM_ID")]
    pub room_id: Option<u32>,
    #[clap(short('i'), long = "player-id", value_name = "PLAYER_ID")]
    pub player_id: Option<u16>,
    #[clap(short('w'), long = "watch", value_name = "WATCH", default_value_t = false)]
    pub watch: bool,
}

pub fn run(_ctx: &Global, _args: &ArgsMRS) -> Result<()> {
    todo!("Implement MRS client command handling");
}