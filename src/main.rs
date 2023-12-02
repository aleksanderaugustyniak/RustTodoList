mod task;
mod list;
mod cli_manager;

fn main() 
{
    let mut cli = cli_manager::CliManager::new();
    cli.run();
}
