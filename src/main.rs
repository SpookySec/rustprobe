use std::fs::File;
use std::ffi::CString;
use nix::kmod::{finit_module, ModuleInitFlags, delete_module, DeleteModuleFlags};
use std::process::exit;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Inserts the kernel module
    Insert {
        #[arg(short, long, value_name = "PATH", required = true)]
        path: Option<String>,
    },

    Remove {
        #[arg(short, long, value_name = "PATH", required = true)]
        path: Option<String>,
    }
}

fn load_module(module_name: &str) {
    let f = match File::open(module_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e:?}");
            exit(1);
        }
    };
    
    match finit_module(&f, &CString::new("").unwrap(), ModuleInitFlags::empty()) {
        Ok(_) => println!("Successfully loaded: {}", module_name),
        Err(e) => eprintln!("Received an error: {e:?}")
    };
}


fn unload_module(module_name: &str) {
    match delete_module(&CString::new(module_name).unwrap(), DeleteModuleFlags::O_NONBLOCK) {
        Ok(_) => println!("Successfully unloaded: {}", module_name),
        Err(e) => eprintln!("{e:?}")
    }
}
    


fn main() {
    let cli = Cli::parse();

    match &cli.command {

        Some(Commands::Insert { path }) => {
            let s = path.as_deref().unwrap();
            println!("Attempting to load: {}", s);
            load_module(s);
        },

        Some(Commands::Remove { path }) => {
            let s = path.as_deref().unwrap();
            println!("Attempting to unload: {}", s);
            unload_module(s);
        },

        None => println!("Use the \"help\" command to get started.")
    };
}
