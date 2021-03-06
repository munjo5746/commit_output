use colored::*;
use std::process::Command;
fn main() {
    // get current branch name by running "grep '^\*' | cut -d' ' -f2 | tr -d '\n'"
    let git_branch = Command::new("git").arg("branch").output().expect("fail");
    let output_branches = String::from_utf8(git_branch.stdout).unwrap();
    let current_branch_with_star = output_branches
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .find(|line| line.contains("*"))
        .unwrap();

    let current_branch = current_branch_with_star.replace("*", "").trim().to_string();

    let potential_ticket_number = current_branch.split("/").nth(0).unwrap();
    let ticket_number = if potential_ticket_number.contains("AV2") {
        potential_ticket_number.to_string()
    } else {
        "-".to_string()
    };

    let jira_url_prefix = "https://theconstellationagency.atlassian.net/browse";
    let jira_ticket_url = if ticket_number != "-" {
        format!("{0}/{1}", jira_url_prefix, ticket_number)
    } else {
        "-".to_string()
    };

    println!("{}", get_line("branch", current_branch));
    println!("{}", get_line("ticket", ticket_number));
    println!("{}", get_line("ticket_url", jira_ticket_url));
}

fn get_line(kind: &str, arg: String) -> String {
    let result = match kind {
        "branch" => format("Your current branch".to_string(), arg),
        "ticket" => format("Your ticket number".to_string(), arg),
        "ticket_url" => format("Your ticket url".to_string(), arg),
        _ => "Unknown".to_string(),
    };

    result
}

fn format(title: String, arg: String) -> String {
    format!("{0:<20}: {1}", title.green(), arg).to_string()
}
