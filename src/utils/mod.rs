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
/// Example: <bug|task>/AV2-<numbere>/<description>
pub fn get_ticket_number(branch: &String) -> String {
    let ticket_parts: Vec<String> = branch.split("/").map(str::to_string).collect();
    if ticket_parts.len() != 3 {
        return String::from("-");
    }

    return ticket_parts[1].to_string();
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

#[cfg(test)]
mod tests {
    use crate::utils::get_ticket_number;

    #[test]
    fn test_get_ticket_number() {
        let empty_ticket = get_ticket_number(&String::from(""));
        assert!(empty_ticket == String::from("-"));
    }
}
