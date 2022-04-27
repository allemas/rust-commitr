use git2::{Error, Commit, ErrorCode, ObjectType, Repository, Signature, StatusOptions, SubmoduleIgnore, Oid};
use std::env;

pub fn get_folder() -> Result<Repository, Error> {
    let repo_root = env::current_dir().expect("Could not determine current dir");
    let dir = repo_root.display().to_string();

    println!("Commit from : {}", dir);
    let repo = Repository::open(dir)?;

    if repo.is_bare() {
        return Err(Error::from_str("cannot report status on bare repository"));
    }
    Ok(repo)
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldnt find commit"))
}


pub fn commit(repo: &Repository, message: &str) -> Result<Oid, Error> {
    let mut index = repo.index()?;
    let oid = index.write_tree()?;

    let signature = Signature::now("Commitr", "commitr@commitr.com")?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;


    repo.commit(Some("HEAD"), //  point HEAD to our new commit
                &signature, // author
                &signature, // committer
                message, // commit message
                &tree, // tree
                &[&parent_commit]) // parents
}