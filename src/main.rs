use std::process::Command;

fn main() {
    // get current branch name by running "grep '^\*' | cut -d' ' -f2 | tr -d '\n'"
    let output = Command::new("echo").arg("testing").output().expect("failed to get current branch name");
    match String::from_utf8(output.stdout) {
        Ok(content) => { println!("output: {}", content)}
        Err(err) => { panic!(err)}
    }
}
