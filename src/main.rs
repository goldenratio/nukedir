use core::panic;
use std::{
  env,
  fs::{self},
  io::{self, Write},
  path::{Path, PathBuf},
};

use clap::Parser;

/// Delete directories recursively from a specified path
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Name of the directory that needs to deleted recursively
  dir_name: String,

  /// skip directories to match GLOB
  #[arg(long)]
  exclude_dir: Option<Vec<String>>,
}

fn find_folders(root: &Path, target_name: &str) -> Vec<PathBuf> {
  let mut result: Vec<PathBuf> = Vec::new();
  find_folders_recursive(root, target_name, &mut result);
  result
}

fn find_folders_recursive(dir: &Path, target_name: &str, result: &mut Vec<PathBuf>) {
  if !dir.is_dir() {
    return;
  }

  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        if entry.file_name() == target_name {
          result.push(path.clone());
        } else {
          find_folders_recursive(&path, target_name, result);
        }
      }
    }
  }
}

fn delete_folders(paths: &[PathBuf]) {
  todo!()
}

fn get_user_conifrmation(prompt: &str) -> Option<bool> {
  println!("{} [y/N]", prompt);
  if io::stdout().flush().is_err() {
    return None;
  }

  let mut input = String::new();
  if io::stdin().read_line(&mut input).is_err() {
    return None;
  }

  match input.trim().to_lowercase().as_str() {
    "y" => Some(true),
    _ => Some(false),
  }
}

fn main() {
  let args = Args::parse();
  let target_name = &args.dir_name;

  if let Ok(root) = env::current_dir() {
    let dir_list = find_folders(root.as_path(), target_name);

    if !dir_list.is_empty() {
      let dir_list_as_str = dir_list
        .iter()
        .filter_map(|x| x.to_str())
        .collect::<Vec<_>>();
      println!("{}", dir_list_as_str.join("\n"));

      match get_user_conifrmation("Are you sure, you want to delete above folders?") {
        Some(true) => delete_folders(&dir_list),
        _ => println!("Action cacelled!"),
      }
    } else {
      println!("Folder with name `{}`, not found!", target_name);
    }
  } else {
    println!("Error getting env::current_dir()");
  }
}
