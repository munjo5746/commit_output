use colored::*;
use std::process::Command;

pub fn get_current_branch() -> String {
    let git_branch = Command::new("git").arg("branch").output().expect("fail");
    let output_branches = String::from_utf8(git_branch.stdout).unwrap();
    let current_branch_with_star = output_branches
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .find(|line| line.contains("*"))
        .unwrap();

    current_branch_with_star.replace("*", "").trim().to_string()
}

pub fn get_ticket_number(branch: &String) -> String {
    let potential_ticket_number = branch.split("/").nth(0).unwrap();

    if potential_ticket_number.contains("AV2") {
        potential_ticket_number.to_string()
    } else {
        "-".to_string()
    }
}

pub fn get_ticket_url(url_prefix: &String, ticket_number: &String) -> String {
    if ticket_number != "-" {
        format!("{0}/{1}", url_prefix, ticket_number)
    } else {
        "-".to_string()
    }
}

pub fn get_info(kind: &str, arg: String) -> String {
    let result = match kind {
        "branch" => format("Your current branch".to_string(), arg),
        "ticket" => format("Your ticket number".to_string(), arg),
        "ticket_url" => format("Your ticket url".to_string(), arg),
        _ => "Unknown".to_string(),
    };

    result
}

pub fn format(title: String, arg: String) -> String {
    format!("{0:<20}: {1}", title.yellow(), arg).to_string()
}