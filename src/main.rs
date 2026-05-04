use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oasis")]
#[command(about = "숭실대학교 도서관 CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
}

#[derive(Subcommand)]
enum AuthCommand {
    Login {
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        force: bool,
    },
    Status,
    Logout,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Auth { command } => match command {
            AuthCommand::Login { id, force } => {
                println!("login: id={id:?}, force={force}");
            }
            AuthCommand::Status => {
                println!("status")
            }
            AuthCommand::Logout => {
                println!("logout")
            }
        },
    }
}
