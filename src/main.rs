extern crate launchpad;
extern crate clap;
extern crate portmidi as pm;

use launchpad::*;

use std::thread;
use std::process;
use std::time::Duration;

mod cli;

fn main() {
    // initialize the PortMidi context.

    let args = cli::build_args();
    let inpt = args.get_matches();
    if inpt.occurrences_of("list") > 0 {
        list();
    }

    run().unwrap_or_else(|e| eprintln!("{}", e));
}

fn list() -> ! {
    let context = pm::PortMidi::new().unwrap();
    let devs = context.devices().unwrap();

    for d in devs {
        println!("{:?}", d);
    }

    process::exit(0);
}

fn run() -> Result<(), Error> {
    println!("Please enjoy!");
    let timeout = Duration::from_millis(1);
    let mut lpad = LaunchpadMini::guess()?;

    println!("Clear screen...");
    lpad.light_all(rg(0,0));
    Ok(())
}
