use clap::Parser;

///RUSTIC CHAT
///
/// This cli uses iroh-gossip to facilitates chat over a p2p network
///
#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// Disable relay completely
    #[clap(long)]
    pub(crate) no_relay: bool,
    /// Setting the nickname
    #[clap(short, long)]
    pub(crate) name: Option<String>,
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Parser, Debug)]
pub(crate) enum Command {
    /// Opens a chat room for a topic and print ticket for other people
    Open,
    /// Joining a chat room using the ticket
    Join {
        /// The ticket to join a chat room
        ticket: String,
    },
}
