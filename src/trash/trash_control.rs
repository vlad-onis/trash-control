use std::fs::read_dir;
use std::path::PathBuf;

use std::io;

use super::trash_cli::Args;
use dirs::home_dir;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrashError {
    #[error("{0}")]
    IO(#[from] io::Error),
}

pub struct TrashControl {}

impl TrashControl {
    pub fn run(&self, args: Args) -> Result<(), TrashError> {
        if args.list {
            self.list_trash()?;
        } else if args.empty {
            self.empty_trash()?;
        }
        Ok(())
    }

    fn list_trash(&self) -> io::Result<Vec<PathBuf>> {
        let mut home_dir = home_dir().expect("Home directory does not exist or is empty");
        home_dir.push(".Trash");
        let trash_path = home_dir;

        // Iterate over elements of read_dir, filter only the okay ones, map them to a path and collect them
        let contents: Vec<PathBuf> = read_dir(trash_path)?
            .into_iter()
            .filter_map(|entry| entry.ok())
            .into_iter()
            .map(|dir_entry| dir_entry.path())
            .collect();
        // iterate over all paths, map them to a string ending in newline, fold all of them into 1 string, and print it
        println!(
            "{}",
            contents
                .iter()
                .map(|file_path| format!("{file_path:?}\n"))
                .fold(String::new(), |acc, arg| { acc + arg.as_str() })
        );

        Ok(contents)
    }

    fn empty_trash(&self) -> io::Result<()> {
        let files = self.list_trash()?;
        for file in files {
            match file.is_dir() {
                true => {
                    std::fs::remove_dir_all(&file)?;
                }
                false => {
                    std::fs::remove_file(&file)?;
                }
            }
        }
        Ok(())
    }
}
