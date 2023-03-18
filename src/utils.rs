use std::{process::{Output, Command}, ffi::OsStr};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CommandExecutor {
    fn execute<I, S>(&mut self, program: S, args: I) -> Output
    where
        I: IntoIterator<Item = S> + 'static,
        S: AsRef<OsStr> + 'static
    {
        Command::new(program)
            .args(args)
            .output()
            .expect("failed to execute process")
    }
}

pub struct DefaultCommandExecutor {}
impl CommandExecutor for DefaultCommandExecutor {}
