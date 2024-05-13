use std::{
    env, fs,
    io::ErrorKind,
    process::{Command, Stdio},
    thread::sleep,
    time::{Duration, SystemTime},
};

const HELP: &str = r#"
crs - command line file watcher

Usage: crs <file> <command> [args...]

crs is a command line filewatcher which runs the specified
<command> with the provided [args...] whenever a modification
of the file is detected.

Example:
    crs src/main.rs cargo test
"#;

struct WatchedFile {
    path: String,
    modified: SystemTime,
}

impl WatchedFile {
    fn new(path: String) -> std::io::Result<Self> {
        let meta = fs::metadata(path.clone())?;
        Ok(WatchedFile {
            path: path.clone(),
            modified: meta.modified()?,
        })
    }

    fn check_modification(&mut self) -> std::io::Result<bool> {
        let modified = std::fs::metadata(self.path.clone())?.modified()?;
        if modified > self.modified {
            self.modified = modified;
            return Ok(true);
        }
        Ok(false)
    }
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 4 {
        println!("{}", HELP);
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            format!("Not enough args. Expected 4. Found {}", argv.len()),
        ));
    };
    let fname = &argv[1];
    let mut f = WatchedFile::new(fname.to_string())?;
    let cmd_string = &argv[2].clone();
    let args: Vec<String> = argv
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i > 2)
        .map(|(_, a)| a)
        .collect();
    loop {
        match f.check_modification() {
            Ok(modified) => {
                if modified {
                    println!("Modified!");
                    let mut cmd = Command::new(cmd_string)
                        .args(args.clone())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?;
                    cmd.wait()?;
                }
            }
            Err(_) => break,
        }
        sleep(Duration::new(2, 0));
    }
    Ok(())
}
