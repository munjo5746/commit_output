

mod utils;
fn main() {
    // get current branch name by running "grep '^\*' | cut -d' ' -f2 | tr -d '\n'"

    let current_branch = utils::get_current_branch();

    let ticket_number = utils::get_ticket_number(&current_branch);

    let url_prefix = "https://theconstellationagency.atlassian.net/browse";
    let jira_ticket_url = utils::get_ticket_url(&url_prefix.to_string(), &ticket_number);

    println!();
    println!("{}", utils::get_info("branch", current_branch));
    println!("{}", utils::get_info("ticket", ticket_number));
    println!("{}", utils::get_info("ticket_url", jira_ticket_url));
    println!();
}

