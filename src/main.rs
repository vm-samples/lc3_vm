extern crate signal_hook;

use lc3_vm;
use lc3_vm::hardware::instructions::*;
use lc3_vm::sys::terminal::*;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<Error>> {
    //setup for interrupt handling.
    spawn_thread_for_signal_processing()?;
    //handle command line arguments and process instructions
    match lc3_vm::handle_args(env::args()) {
        Ok(mem) => {
            process_instructions(mem);
            //restore terminal settings
            restore_terminal_settings();
            //return
            Ok(())
        }
        Err(_) => {
            //restore terminal settings
            restore_terminal_settings();
            //exit
            process::exit(1)
        }
    }
}
