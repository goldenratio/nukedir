use glob::Pattern;
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
  /// Name of the directory that needs to deleted recursively (GLOB)
  dir_name: Vec<String>,

  /// skip directories to match GLOB
  #[arg(long)]
  exclude_dir: Option<Vec<String>>,

  /// Maximum directory depth to recurse into (inclusive)
  #[arg(long, default_value_t = 5)]
  max_depth: u8,

  /// skip directory deletion confirmation prompt
  #[arg(long, default_value_t = false)]
  yes: bool,
}

fn find_folders(
  root: &Path,
  target_pattern_list: &[Pattern],
  exclude_dir_pattern_list: &[Pattern],
  max_depth: u8,
) -> Vec<PathBuf> {
  let mut result: Vec<PathBuf> = Vec::new();
  find_folders_recursive(
    root,
    target_pattern_list,
    exclude_dir_pattern_list,
    max_depth,
    0,
    &mut result,
  );
  result
}

fn find_folders_recursive(
  dir: &Path,
  target_pattern_list: &[Pattern],
  exclude_dir_pattern_list: &[Pattern],
  max_depth: u8,
  current_depth: u8,
  result: &mut Vec<PathBuf>,
) {
  if current_depth > max_depth {
    return;
  }

  if !dir.is_dir() {
    return;
  }

  if exclude_dir_pattern_list.iter().any(|p| p.matches_path(dir)) {
    return;
  }

  if let Some(dir_name_str) = dir.file_name().and_then(|name| name.to_str()) {
    if target_pattern_list.iter().any(|p| p.matches(dir_name_str)) {
      // check if dir already exist in result
      if !result
        .iter()
        .any(|existing| dir.starts_with(existing) && existing != dir)
      {
        result.push(dir.to_path_buf());
      }
      // dir will be deleted, no need recurse further
      return;
    }
  }

  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        find_folders_recursive(
          &path,
          target_pattern_list,
          exclude_dir_pattern_list,
          max_depth,
          current_depth + 1,
          result,
        );
      }
    }
  }
}

fn delete_folders(paths: &[PathBuf]) {
  for path in paths {
    let _ = fs::remove_dir_all(path);
  }
}

fn get_user_confirmation(prompt: &str) -> Option<bool> {
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
  let target_pattern_list = &args
    .dir_name
    .iter()
    .filter_map(|p| Pattern::new(p).ok())
    .collect::<Vec<Pattern>>();

  let exclude_dir_glob = &args.exclude_dir.unwrap_or_default();
  let max_depth = args.max_depth;
  let skip_confirmation = args.yes;

  let exclude_dir_pattern = exclude_dir_glob
    .iter()
    .filter_map(|p| Pattern::new(p).ok())
    .collect::<Vec<Pattern>>();

  if let Ok(root) = env::current_dir() {
    let dir_list = find_folders(
      root.as_path(),
      target_pattern_list,
      &exclude_dir_pattern,
      max_depth,
    );

    if !dir_list.is_empty() {
      let dir_list_as_str = dir_list
        .iter()
        .filter_map(|x| x.to_str())
        .collect::<Vec<_>>();
      println!("{}", dir_list_as_str.join("\n"));

      if skip_confirmation
        || matches!(
          get_user_confirmation("Are you sure, you want to delete above folders?"),
          Some(true)
        )
      {
        delete_folders(&dir_list);
      } else {
        println!("Action cancelled!")
      }
    } else {
      eprintln!("No matching folder names found!");
    }
  } else {
    eprintln!("Error getting env::current_dir()");
  }
}
