use std::fs::{ self, File };
use std::io::prelude::*;
use std::path::Path;

use colored::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct SubTask {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub intro: Option<String>,
    pub subs: Vec<SubTask>,
}

// The session.
// Handles the things.
#[derive(Serialize, Deserialize)]
pub struct ZubSession {
    pub name: String,
    pub author: String,
    pub content: Vec<Task>
}

impl ZubSession {
    pub fn new(name: String, author: String, content: Vec<Task>) -> Self {
        ZubSession {
            name,
            author,
            content,
        }
    }

    pub fn push_task(&mut self, name: String) {
        self.content.push(
            Task {
                name,
                subs: Vec::new(),
                intro: None,
            }
        )
    }

    pub fn push_sub_task(&mut self, name: String) {
        match self.content.last_mut() {
            Some(mut task) => {
                task.subs.push(
                    SubTask {
                        name,
                        content: String::new(),
                    }
                )
            },
            None => unimplemented!()
        }
    }

    pub fn current_task_mut(&mut self) -> &mut Task {
        match self.content.last_mut() {
            Some(task) => task,
            None => {
                panic!("Not supposed to happen. Call the 12.")
            }
        }
    }

    pub fn append(&mut self, content: String) {
        self.current_task_mut().subs.last_mut().unwrap().content.push_str(
            &format!(
                "\n{}",
                content
            )
        )
    }

    pub fn set_current_intro(&mut self, intro: String) {
        self.current_task_mut().intro = Some(intro)
    }
}

// Acts as an API for ZubTime sessions.
// This is where you interface with the session.
// Also used for saving things.
pub struct ZubFile {
    pub path: String,
    pub session: ZubSession,
}

impl ZubFile {
    pub fn new(name: String, path: String, author: String) -> Self {
        Self {
            path: format!("{}.zub", path),
            session:  ZubSession::new(name, author, Vec::new()),
        }
    }

    pub fn from(path: &str) -> Option<Self> {
        let path = Path::new(path);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(_) => {
                println!("{}", "This file is broken.".red());
                return None
            },
            Ok(file) => file,
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Err(_) => {
                println!("{}", "This file's content is broken.".red());
                None
            },
            Ok(_) => Some(
                ZubFile {
                    path: path.display().to_string(),
                    session: match serde_json::from_str(&s) {
                        Ok(result) => result,
                        Err(_) => {
                            println!("{}", "This file's JSON content is broken.".red());
                            return None
                        },
                    }
                }
            )
        }
    }

    pub fn save(&mut self) -> Result<()> {
        let json = serde_json::to_string(&self.session)?;

        let path = Path::new(&self.path);

        fs::write(path, json).expect("Unable to save. This is crazy.");

        Ok(())
    }
}