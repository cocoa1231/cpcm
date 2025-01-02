use clap::Parser;
use crate::command_domain::DomainArgs;
use crate::command_server::ServerAdd;


#[derive(Debug, Parser)]
pub enum Cpcm {
    // Initialize directories
    Init(InitSubcommand),

    // Manage domains
    Domain(DomainArgs),

    #[clap(subcommand, name = "server")]
    Server(ServerSubcommand)
}



#[derive(Parser, Debug)]
pub enum ServerSubcommand {
    Add(ServerAdd)
}

#[derive(Parser, Debug)]
pub struct InitSubcommand {
    #[arg(short, long)]
    pub force: bool
}
