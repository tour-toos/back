use std::net::SocketAddr;


#[derive(clap::Parser)]
pub struct Cli {

    /// Database url 'sqlite:path/to/database'
    /// 
    /// Modes:
    /// 
    ///     1. SQLite with file: 
    ///         * '?mode=rwc' - сreate file if not exists;
    ///         * '?mode=ro'  - read only.
    /// 
    ///     2. SQLite in memory: 'sqlite::memory:'
    /// 
    #[clap(long, default_value="sqlite:/database?mode=rwc")]
    pub database: String,

    /// 
    #[clap(subcommand)]
    pub mode: Option<Mode>
}


#[derive(clap::Subcommand)]
pub enum Mode {
    Operate {
        #[clap(subcommand)]
        command: Option<Operate>
    },
    
}

#[derive(clap::Subcommand)]
pub enum Operate {
    /// Start in standalone mode
    Standalone {
        #[clap(default_value = "127.0.0.1:30100")]
        addr: SocketAddr,
    },
    /// Start in api mode
    Api {
        #[clap(default_value = "127.0.0.1:30100")]
        addr: SocketAddr,
    }
}