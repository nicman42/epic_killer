use std::process::Command;
use std::collections::HashSet;
use std::io;
use walkdir::WalkDir;

const DEBUG: bool = false;
const EPIC_LAUNCHER: &str = "";
const EPIC_GAMES: &str = "C:\\opt";

fn main() {
    let games = find_games(EPIC_GAMES).expect("no games found");
    if DEBUG {
        for game in &games {
            println!("game: {}", game);
        }
    }

    println!("something running: {}", is_game_running(games).unwrap());
}



fn is_game_running(game_paths: HashSet<String>) -> io::Result<bool> {
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