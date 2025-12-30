use crate::args::DaemonCommands;

mod start;
mod stop;
use start::start;
use stop::stop;

pub async fn main(command: DaemonCommands) {
    match command {
        DaemonCommands::Restart => {
            stop().await;
            start().await;
        }
        DaemonCommands::Start => {
            start().await;
        }
        DaemonCommands::Stop => {
            stop().await;
        }
    }
}
