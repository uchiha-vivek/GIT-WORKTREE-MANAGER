use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;
use colored::*;

#[derive(Parser)]
#[command(name = "worktree-manager")]
#[command(about = "A simple CLI tool to manage Git worktrees", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new branch and a worktree for it
    Create {
        /// Name of the feature/branch (e.g., feature/login)
        name: String,
    },
    /// List all existing Git worktrees
    List,
    /// Remove a worktree and delete its directory
    Remove {
        /// Name of the feature/branch to remove
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => create_worktree(&name),
        Commands::List => list_worktrees(),
        Commands::Remove { name } => remove_worktree(&name),
    }
}

/// Helper function to run terminal commands cleanly
fn run_git_cmd(args: &[&str]) -> bool {
    let status = Command::new("git")
        .args(args)
        .status();

    match status {
        Ok(s) => s.success(),
        Err(e) => {
            eprintln!("{} Failed to execute git command: {}", "error:".red().bold(), e);
            false
        }
    }
}

fn create_worktree(name: &str) {
    // Standardize folder name (e.g. "feature/login" -> "login")
    let folder_name = name.split('/').last().unwrap_or(name);
    let target_path: PathBuf = ["workspaces", folder_name].iter().collect();

    println!("{} {}", "Creating branch:".cyan().bold(), name);
    if !run_git_cmd(&["branch", name]) {
        eprintln!(
            "{} Branch might already exist, attempting to create worktree anyway...",
            "warning:".yellow().bold()
        );
    }

    println!("{} {}", "Creating worktree at:".cyan().bold(), target_path.display());
    if run_git_cmd(&["worktree", "add", target_path.to_str().unwrap(), name]) {
        println!("{}", "Worktree created successfully!".green().bold());
    } else {
        eprintln!("{}", "Failed to create worktree.".red().bold());
    }
}

fn list_worktrees() {
    println!("{}\n", "Current Git Worktrees:".cyan().bold());
    run_git_cmd(&["worktree", "list"]);
}

fn remove_worktree(name: &str) {
    let folder_name = name.split('/').last().unwrap_or(name);
    let target_path: PathBuf = ["workspaces", folder_name].iter().collect();

    println!("{} {}", "Removing worktree at:".cyan().bold(), target_path.display());
    if run_git_cmd(&["worktree", "remove", target_path.to_str().unwrap()]) {
        println!("{}", "Worktree removed successfully!".green().bold());
    } else {
        eprintln!("{}", "Failed to remove worktree.".red().bold());
    }
}