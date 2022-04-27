use git2::{Repository, StatusOptions};
use std::string::String as OtherString;

pub fn build_commited_message(repository: &Repository) -> Result<OtherString, git2::Error> {
    let mut opts = StatusOptions::new();
    let statues = repository.statuses(Some(&mut opts))?;
    let mut commit_message = OtherString::from("");


    for entry in statues.iter().filter(|e| e.status() != git2::Status::CURRENT) {
        let istatus = match entry.status() {
            s if s.contains(git2::Status::INDEX_NEW) => "new file: ",
            s if s.contains(git2::Status::INDEX_MODIFIED) => "modified: ",
            s if s.contains(git2::Status::INDEX_DELETED) => "deleted: ",
            s if s.contains(git2::Status::INDEX_RENAMED) => "renamed: ",
            s if s.contains(git2::Status::INDEX_TYPECHANGE) => "typechange:",
            _ => continue,
        };
        let message = format!("{} : {} \r\n", istatus, entry.path().unwrap());
        commit_message = commit_message + &message;
    }

    Ok(commit_message)
}
