use std::process::Command;

fn main() {
    // get current branch name by running "grep '^\*' | cut -d' ' -f2 | tr -d '\n'"
    let git_branch = Command::new("git").arg("branch").output().expect("fail");
    let output_branches = String::from_utf8(git_branch.stdout).unwrap();
    let current_branch = output_branches
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .find(|line| line.contains("*"))
        .unwrap();
    print!("\n\n");
    println!(
        "Your current branch: {}",
        current_branch.replace("*", "").trim()
    );
    println!(
        "Your ticket number: {}",
        current_branch.replace("*", "").trim()
    );
    println!(
        "Your ticket number: https://theconstellationagency.atlassian.net/browse/{}",
        current_branch.replace("*", "").trim()
    );
    // match String::from_utf8(output.stdout) {
    //     Ok(content) => { println!("output: {}", content)}
    //     Err(err) => { panic!(err)}
    // }
}
