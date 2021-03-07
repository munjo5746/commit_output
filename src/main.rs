use colored::*;
use std::process::Command;
fn main() {
    // get current branch name by running "grep '^\*' | cut -d' ' -f2 | tr -d '\n'"

    let current_branch = get_current_branch();

    let ticket_number = get_ticket_number(current_branch.as_str());

    let url_prefix = "https://theconstellationagency.atlassian.net/browse";
    let jira_ticket_url = get_ticket_url(url_prefix, ticket_number.as_str());

    println!();
    println!("{}", get_info("branch", current_branch));
    println!("{}", get_info("ticket", ticket_number));
    println!("{}", get_info("ticket_url", jira_ticket_url));
}

fn get_current_branch() -> String {
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

fn get_ticket_number(branch: &str) -> String {
    let potential_ticket_number = branch.clone().split("/").nth(0).unwrap();

    if potential_ticket_number.contains("AV2") {
        potential_ticket_number.to_string()
    } else {
        "-".to_string()
    }
}

fn get_ticket_url(url_prefix: &str, ticket_number: &str) -> String {
    if ticket_number != "-" {
        format!("{0}/{1}", url_prefix, ticket_number)
    } else {
        "-".to_string()
    }
}

fn get_info(kind: &str, arg: String) -> String {
    let result = match kind {
        "branch" => format("Your current branch".to_string(), arg),
        "ticket" => format("Your ticket number".to_string(), arg),
        "ticket_url" => format("Your ticket url".to_string(), arg),
        _ => "Unknown".to_string(),
    };

    result
}

fn format(title: String, arg: String) -> String {
    format!("{0:<20}: {1}", title.yellow(), arg).to_string()
}
