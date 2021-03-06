use std::process::Command;

fn main() {
    // get current branch name by running "grep '^\*' | cut -d' ' -f2 | tr -d '\n'"
    let git_branch = Command::new("git").arg("branch").output().expect("fail");
    let branches = String::from_utf8(git_branch.stdout).unwrap();
    println!("{}", branches)

    // match String::from_utf8(output.stdout) {
    //     Ok(content) => { println!("output: {}", content)}
    //     Err(err) => { panic!(err)}
    // }
}
