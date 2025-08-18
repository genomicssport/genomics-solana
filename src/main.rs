mod args;
mod generatannot;
mod map;
mod readstruct;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::generatannot::getrsid;
use crate::map::mapid;
use async_std::task;
use clap::Parser;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-7
*/

fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::GenerateSol { pathfile, rsid } => {
            let command = task::block_on(mapid(pathfile, rsid.to_string())).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::GenerateInfo { rsid } => {
            let command = task::block_on(getrsid(rsid)).unwrap();
            println!("The command has finished:{}", command);
        }
    }
}
