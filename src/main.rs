use std::{
    fs,
    thread::sleep,
    time::{Duration, SystemTime},
};

struct WatchedFile {
    path: String,
    accessed: SystemTime,
}

impl WatchedFile {
    fn new(path: String) -> std::io::Result<Self> {
        let meta = fs::metadata(path.clone())?;
        Ok(WatchedFile {
            path: path.clone(),
            accessed: meta.modified()?,
        })
    }

    fn was_modified(&mut self) -> std::io::Result<bool> {
        Ok(std::fs::metadata(self.path.clone())?.modified()? > self.accessed)
    }
}

fn main() -> std::io::Result<()> {
    let fname = "test.txt";
    let mut counter = 0;
    let mut f = WatchedFile::new(fname.to_string())?;
    sleep(Duration::new(2, 0));
    loop {
        counter += 1;
        if f.was_modified()? {
            println!("Was modified !");
            f = WatchedFile::new(fname.to_string())?;
        }
        sleep(Duration::new(2, 0));
        if counter == 100 {
            break;
        };
    }
    Ok(())
}
