mod todo;
mod cli;

fn main() 
{
    let mut cli = crate::cli::cli_manager::CliManager::new();
    cli.run();
}
