mod git_engine;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let repository = git_engine::repository::get_folder().unwrap();

    let mut commit_message = git_engine::status::build_commited_message(&repository).expect("build message faild");
    let message = &args[1];

    commit_message = message.to_owned() + "\n\n" + &commit_message;

    println!("{}", commit_message);

    let commit_id = git_engine::repository::commit(&repository, &commit_message).expect("error durring commit");
    println!("{}", commit_id);
}
