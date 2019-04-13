use std::process::Command;
use std::collections::HashSet;
use std::{thread, time, io};
use walkdir::WalkDir;

const EPIC_LAUNCHER: &str = "EpicGamesLauncher.exe";
const SLEEP_SECONDS: u64 = 15;
const NOT_RUNNING_LOOPS: u8 = 2;

fn main() {
    let epic_games_root = std::env::args().nth(1);
    if epic_games_root.is_none() {
        println!("USAGE: {} EPIC_GAMES_DIRECTORY", std::env::args().nth(0).unwrap());
        return;
    }
    
    let games = find_games(&epic_games_root.unwrap()).expect("no games found");
    
    let mut not_running = 0;
    while true {
        if is_game_running(&games).unwrap() {
            not_running = 0;
            println!("a game is running");
        }else{
            not_running += 1;
            println!("no game is running ({}. time)", not_running);
            if not_running >= NOT_RUNNING_LOOPS {
                println!("kill epic launcher...");
                if kill_epic_launcher().is_err() {
                    println!("couldn't kill epic launcher");
                }
            }
        }
        
        thread::sleep(time::Duration::new(SLEEP_SECONDS, 0));
    }
}



fn is_game_running(game_paths: &HashSet<String>) -> io::Result<bool> {
    let is_running = !find_running()?.is_disjoint(&game_paths);
    
    Ok(is_running)
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

fn kill_epic_launcher() -> io::Result<std::process::ExitStatus>{
    Command::new("taskkill.exe")
        .args(&["/IM", EPIC_LAUNCHER])
        .stderr(std::process::Stdio::null())
        .status()
}
