use std::process::Command;
use std::collections::HashSet;
use std::{thread, time};

use sysinfo::System;
use std::path::Path;

const LAUNCHER: [&str; 2] = ["EpicGamesLauncher.exe", "EADesktop.exe"];
const SLEEP_SECONDS: u64 = 5;
const NOT_RUNNING_LOOPS: u8 = 5;

fn main() {
    let games_root = std::env::args().nth(1);

    if games_root.is_none() {
        println!("USAGE: {} GAMES_DIRECTORY", std::env::args().nth(0).unwrap());
        return;
    }

    let games_root = games_root.unwrap().to_lowercase();
    
    let mut not_running = 0;
    loop {
        println!();

		let (running_launcher, running_games) = is_running(&games_root);
		
		if running_launcher.is_empty() {
			not_running = 0;
			println!("launcher is not running");
        }else {
            println!("launcher is running:");
            for path in &running_launcher {
                println!("\t{}", &path);
            }
        }

		if !running_games.is_empty() {
            not_running = 0;
            println!("a game is running:");
            for running_game in &running_games {
                println!("\t{}", running_game);

                // focus game window
                let command: String = format!("$process = Get-Process -ErrorAction SilentlyContinue | Where-Object {{ $_.Path -eq \"{running_game}\" -and $_.MainWindowHandle -ne 0 }}; if ($process) {{ (New-Object -ComObject wscript.shell).AppActivate($process.Id) }}");
                let output = Command::new("powershell").args(&["-command", &command]).output().unwrap().stdout;
                println!("\t\t{}", String::from_utf8(output).unwrap());
            }
        } else if !running_launcher.is_empty(){
            not_running += 1;
            println!("no game is running ({}. time)", not_running);
            if not_running >= NOT_RUNNING_LOOPS {
                for launcher in &running_launcher {
                    kill_launcher(&launcher)
                }
            }
        }
        thread::sleep(time::Duration::new(SLEEP_SECONDS, 0));
    }
}



fn is_running(games_root: &String) -> (HashSet::<String>, HashSet::<String>) {
    let mut running_launcher = HashSet::<String>::new();
    let mut running_games = HashSet::<String>::new();

    for (_, process) in System::new_all().processes() {
        let exe = process.exe();
        if exe.is_none() {
            continue;
        }

        let path: &str = exe.unwrap().to_str().unwrap();
        // println!("{}", path);

        for launcher in LAUNCHER {
            if path.ends_with(launcher) {
                running_launcher.insert(path.to_string());
            }
        }
        if path.to_lowercase().starts_with(games_root) {
            running_games.insert(path.to_string());
        }
    }

    return (running_launcher, running_games)
}

fn kill_launcher(launcher: &String) -> () {
    let launcher_file_name = Path::new(launcher).file_name().unwrap();

    println!("kill launcher {:?}:", launcher_file_name);

    for process in System::new_all().processes_by_exact_name(launcher_file_name) {
        let exe = process.exe();
        if exe.is_none() {
            continue;
        }

        let path: &str = exe.unwrap().to_str().unwrap();
        if path == launcher {
            println!("\t{}", path);
            process.kill();
        }
    }

}
