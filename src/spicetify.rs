use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process;
pub struct Spicetify {
    config_path: PathBuf,
    theme: String,
}
impl Spicetify {
    pub fn new(home: PathBuf, theme: &str) -> Self {
        let mut config_path = home;
        config_path.push(format!(".config/spicetify/Themes/{theme}/color.ini"));
        if let Err(e) = fs::metadata(&config_path) {
            panic!("Error reading file {} {}", config_path.display(), e);
        }
        Self {
            config_path,
            theme: String::from(theme),
        }
    }

    pub fn reload(&self) {
        process::Command::new("spicetify")
            .arg("config")
            .arg("current_theme")
            .arg(&self.theme)
            .status()
            .unwrap();
        process::Command::new("spicetify")
            .arg("config")
            .arg("color_scheme")
            .arg("pywal")
            .status()
            .unwrap();
        match process::Command::new("spicetify").arg("apply").output() {
            Ok(stdout) => {
                println!("Running spicetify...");
                if let Ok(output) = String::from_utf8(stdout.stdout) {
                    println!("{output}");
                }
            }
            Err(e) => panic!("Error running spicetify apply {}", e),
        }
    }

    pub fn write_config(&self, wal_config: Option<String>) {
        let file = File::open(&self.config_path).expect("Invalid path");

        let reader = BufReader::new(file);

        let lines: Vec<String> = reader
            .lines()
            .collect::<Result<_, _>>()
            .expect("Error reading lines!");

        let mut buf: Vec<String> = Vec::new();
        let mut i = 0;
        while i < lines.len() {
            //Remove exisitng config
            if lines[i].contains("pywal") {
                buf.pop(); //pop the last \n
                i += 14;
                continue;
            }
            buf.push(lines[i].clone());
            i += 1;
        }
        let mut writer = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.config_path)
            .expect("Error opening file!");

        let mut content = buf.join("\n");
        if let Some(wal_config) = wal_config {
            content.push_str("\n\n");
            content.push_str("[pywal]");
            content.push('\n');
            content.push_str(&wal_config);
        }

        writer
            .write_all(content.as_bytes())
            .expect("Error writing to file!");
    }
}
