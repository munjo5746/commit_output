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
    print!("\n\n");
    println!("Your current branch: {}", current_branch);

    let potential_ticket_number = current_branch.split("/").nth(0).unwrap();
    let ticket_number = if potential_ticket_number.contains("AV2") {
        potential_ticket_number
    } else {
        "-"
    };

    println!("Your ticket number: {}", ticket_number);
    println!(
        "Your ticket number: https://theconstellationagency.atlassian.net/browse/{}",
        ticket_number
    );
}
