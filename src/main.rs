// src/main.rs
use std::process::Command;
use inquire::{Select, Text, Confirm};
use anyhow::Result;

fn main() -> Result<()> {
    let commit_type_options = vec![
        "feat: A new feature",
        "fix: A bug fix",
        "docs: Documentation only changes",
        "style: Changes that do not affect the meaning of the code (white-space, formatting, etc)",
        "refactor: A code change that neither fixes a bug nor adds a feature",
        "perf: A code change that improves performance",
        "test: Adding missing tests or correcting existing tests",
        "build: Changes that affect the build system or external dependencies",
        "ci: Changes to our CI configuration files and scripts",
        "chore: Other changes that don't modify src or test files",
        "revert: Reverts a previous commit",
        "security: Patches a security vulnerability",
        "release: Creates a release commit",
        "dx: Improves developer experience",
        "wip: Work in progress (not for merge)",
    ];

    let selected_type = Select::new(
        "Select the type of change you're committing:",
        commit_type_options,
    ).prompt()?;

    let scope = Text::new("What is the scope of this change? (e.g., component or file name) (optional)")
        .prompt_skippable()?;

    let short_description = Text::new("Write a short, imperative tense description of the change (max 72 chars):")
        .with_validator(|input: &str| {
            if input.len() > 72 {
                Ok(inquire::validator::Validation::Invalid("Description cannot be longer than 72 characters.".into()))
            } else if input.is_empty() {
                Ok(inquire::validator::Validation::Invalid("Description cannot be empty.".into()))
            }
            else {
                Ok(inquire::validator::Validation::Valid)
            }
        })
        .prompt()?;

    let commit_type = selected_type.split(':').next().unwrap_or(selected_type);

    let scope_formatted = scope
        .filter(|s| !s.trim().is_empty())
        .map(|s| format!("({})", s.trim()))
        .unwrap_or_default();

    let commit_message = format!("{}{}: {}", commit_type, scope_formatted, short_description);

    println!("\nGenerated commit message:\n---\n{}\n---", commit_message);

    let should_commit = Confirm::new("Do you want to proceed with the commit?").with_default(true).prompt()?;

    if should_commit {
        let status = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(&commit_message)
            .status()?;

        if status.success() {
            println!("\n✅ Commit created successfully!");
        } else {
            eprintln!("\n❌ Error: Git commit command failed.");
        }
    } else {
        println!("\nCommit aborted by user.");
    }

    Ok(())
}