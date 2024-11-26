use std::process::Command;
use std::collections::HashSet;
use std::{thread, time, io};
use walkdir::WalkDir;
use procfs::process::{all_processes, Process};

const LAUNCHER: &str = "EADesktop.exe";
const SLEEP_SECONDS: u64 = 5;
const NOT_RUNNING_LOOPS: u8 = 10;

fn main() {
    let games_root = std::env::args().nth(1);

    for process in all_processes().unwrap() {
        let p: Process = process.unwrap();
        //let path: PathBuf = p.exe().unwrap();
        //println!("path: {}", path.display());

        match p.exe() {
            Ok(path) => println!("path: {}", path.display()),
            Err(_) => (),
        }
    }
    
    if games_root.is_none() {
        println!("USAGE: {} GAMES_DIRECTORY", std::env::args().nth(0).unwrap());
        return;
    }

    let games = find_games(&games_root.unwrap()).unwrap();
    
    let mut not_running = 0;
    loop {
		let (running_launcher, running_games) = is_running(&games).unwrap();
		
		if !running_launcher {
			not_running = 0;
			println!("launcher is not running");
		} else if !running_games.is_empty() {
            not_running = 0;
            println!("a game is running:");
            for running_game in &running_games {
                println!("\t{}", running_game);

                let command: String = format!("$process = Get-Process -ErrorAction SilentlyContinue | Where-Object {{ $_.Path -eq \"{running_game}\" -and $_.MainWindowHandle -ne 0 }}; if ($process) {{ (New-Object -ComObject wscript.shell).AppActivate($process.Id) }}");
                //println!("command: {}", command);
                let output = Command::new("powershell").args(&["-command", &command]).output().unwrap().stdout;
                println!("\t\t{}", String::from_utf8(output).unwrap());
            }
        }else{
            not_running += 1;
            println!("no game is running ({}. time)", not_running);
            if not_running >= NOT_RUNNING_LOOPS {
                if kill_launcher(not_running > NOT_RUNNING_LOOPS).is_err() {
                    println!("couldn't kill launcher");
                }
            }
        }
        
        thread::sleep(time::Duration::new(SLEEP_SECONDS, 0));
    }
}



fn is_running(game_paths: &HashSet<String>) -> io::Result<(bool, HashSet::<String>)> {
	let running_paths: HashSet<String> = find_running()?;

    let mut running_launcher: bool = false;
    let mut running_games = HashSet::<String>::new();
    for path in running_paths {
        if path.ends_with(LAUNCHER) {
			println!("launcher path: {}", path);
			running_launcher = true
		}
        if game_paths.contains(&path) {
            running_games.insert(path);
        }
    }
    
    Ok((running_launcher, running_games))
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
 * Return path to games
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

fn kill_launcher(force: bool) -> io::Result<std::process::ExitStatus>{
	if force {
		println!("kill launcher (force)...");
	}else{
		println!("kill launcher...");
	}

    let mut cmd = Command::new("taskkill.exe");
    cmd.args(&["/IM", LAUNCHER]);
	if force {
		cmd.arg("/F");
	}

    cmd.stderr(std::process::Stdio::null())
        .status()
}
