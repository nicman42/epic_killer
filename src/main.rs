use std::process::Command;
use std::collections::HashSet;
use std::{thread, time, io};
use walkdir::WalkDir;

const EPIC_LAUNCHER: &str = "EpicGamesLauncher.exe";
const SLEEP_SECONDS: u64 = 5;
const NOT_RUNNING_LOOPS: u8 = 5;

fn main() {
    let epic_games_root = std::env::args().nth(1);
    if epic_games_root.is_none() {
        println!("USAGE: {} EPIC_GAMES_DIRECTORY", std::env::args().nth(0).unwrap());
        return;
    }
    
    let games = find_games(&epic_games_root.unwrap()).expect("no games found");
    
    let mut not_running = 0;
    loop {
		let (running_epic, running_game) = is_running(&games).unwrap();
		
		if !running_epic {
			not_running = 0;
			println!("epic launcher is not running");
		} else if running_game {
            not_running = 0;
            println!("a game is running");
        }else{
            not_running += 1;
            println!("no game is running ({}. time)", not_running);
            if not_running >= NOT_RUNNING_LOOPS {
                if kill_epic_launcher(not_running > NOT_RUNNING_LOOPS).is_err() {
                    println!("couldn't kill epic launcher");
                }
            }
        }
        
        thread::sleep(time::Duration::new(SLEEP_SECONDS, 0));
    }
}



fn is_running(game_paths: &HashSet<String>) -> io::Result<(bool,bool)> {
	let running_paths = find_running()?;
	
    let running_game = !running_paths.is_disjoint(&game_paths);

	let mut running_epic = false;
	for path in running_paths {
		if path.ends_with(EPIC_LAUNCHER) {
			println!("epic launcher path: {}", path);
			running_epic = true
		}
	}
    
    Ok((running_epic, running_game))
}


/**
 * 
 */
fn find_running() -> io::Result<HashSet<String>> {
    let output = Command::new("wmic.exe")
        .args(&["process", "get", "executablepath"])
        .output()?
        .stdout;
        
    let mut paths = HashSet::new();
    for line in String::from_utf8(output).unwrap().lines() {
        let line = line.trim();
        if !line.is_empty() {
            paths.insert(line.to_string());
        }
    }
    
    Ok(paths)
}

/**
 * Return path to epic games
 */
fn find_games(root: &str) -> io::Result<HashSet<String>> {
    let mut paths = HashSet::new();
    
    for entry in WalkDir::new(root) {
        let path = entry?.path().display().to_string();
        if path.ends_with(".exe") {
            paths.insert(path);
        }
    }
    
    Ok(paths)
}

fn kill_epic_launcher(force: bool) -> io::Result<std::process::ExitStatus>{
	if force {
		println!("kill epic launcher (force)...");
	}else{
		println!("kill epic launcher...");
	}

    let mut cmd = Command::new("taskkill.exe");
    cmd.args(&["/IM", EPIC_LAUNCHER]);
	if force {
		cmd.arg("/F");
	}

    cmd.stderr(std::process::Stdio::null())
        .status()
}
