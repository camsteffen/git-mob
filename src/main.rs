use git_mob_tool::{
    cli,
    coauthor_repo::GitConfigCoauthorRepo,
    utils::{CommandExecutor, DefaultCommandExecutor},
};
use std::io;

fn main() {
    let mut executor = DefaultCommandExecutor {};
    let coauthor_repo = GitConfigCoauthorRepo::new(&mut executor);
    let out = &mut io::stdout();
    let err = &mut io::stderr();

    cli::run(&coauthor_repo, out, err);
}
