extern crate git2;

use std::env;
use git2::{Branch, Repository};

fn main() {
    let current_path_buffer = match env::current_dir() {
        Ok(path_buffer) => path_buffer,
        Err(e) => panic!("Failed to get current working directory: {}", e),
    };
    let repo = match Repository::open(current_path_buffer) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open repository: {}", e),
    };

    match repo.is_empty() {
        Ok(is_empty) => {
            if is_empty {
                println!("Nothing to see here");
                return;
            }
        },
        Err(e) => panic!("Failed to check if repository is empty: {}", e),
    }

    let head = match repo.head() {
        Ok(reference) => reference,
        Err(e) => panic!("Failed to retrieve HEAD: {}", e),
    };

    if head.is_branch() {
        let branch = Branch::wrap(head);

        let branch_name = match branch.name() {
            Ok(Some(name)) => name,
            Ok(None) => "",
            Err(e) => panic!("Failed to get branch name: {}", e),
        };

        println!("We're in {}", branch_name);
    } else {
        if let Some(oid) = head.target() {
            let hash = oid.to_string();
            println!("Not a branch, we're in {}", &hash[..7]);
        } else {
            println!("Current branch has no name or reference.");
        }
    }
}
