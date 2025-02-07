use crate::coauthor_repo::CoauthorRepo;
use clap::{arg, Parser};
use std::io;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct Coauthor {
    /// Adds co-author to co-author repository
    ///
    /// Usage example: git mob co-author --add lm "Leo Messi" leo.messi@example.com
    #[arg(short = 'a', long = "add", num_args=3, value_names=["COAUTHOR_KEY", "COAUTHOR_NAME", "COAUTHOR_EMAIL"])]
    pub(crate) add: Option<Vec<String>>,
    /// Remove co-author from co-author repository
    ///
    /// Usage example: git mob co-author --delete lm
    #[arg(short = 'd', long = "delete", value_name = "COAUTHOR_KEY")]
    pub(crate) delete: Option<String>,
    /// Lists co-author(s) with keys(s) from co-author repository
    ///
    /// Usage example: git mob co-author --list
    #[arg(short = 'l', long = "list")]
    pub(crate) list: bool,
}

impl Coauthor {
    pub(crate) fn handle(
        &self,
        coauthor_repo: &impl CoauthorRepo,
        out: &mut impl io::Write,
        err: &mut impl io::Write,
    ) {
        if let Some(key) = self.delete.as_deref() {
            match coauthor_repo.get(key) {
                Some(_) => coauthor_repo.remove(key),
                None => writeln!(err, "No co-author found with key: {key}").expect("write failed"),
            }
        }
        if self.list {
            let coauthors = coauthor_repo.list(true);
            if !coauthors.is_empty() {
                writeln!(out, "{}", coauthors.join("\n")).expect("write failed");
            }
        }
        if let Some([key, name, email]) = self.add.as_deref() {
            let coauthor = format!("{name} <{email}>");
            coauthor_repo.add(key, &coauthor);
            writeln!(out, "{coauthor}").expect("write failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coauthor_repo::MockCoauthorRepo;
    use mockall::predicate;

    #[test]
    fn test_delete_coauthor() {
        let key = "lm";
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .return_const("Leo Messi <leo.messi@example.com>".to_owned());
        mock_coauthor_repo
            .expect_remove()
            .with(predicate::eq(key))
            .once()
            .return_const(());

        let coauthor_cmd = Coauthor {
            delete: Some(key.to_owned()),
            add: None,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);
    }

    #[test]
    fn test_error_message_shown_when_trying_to_delete_coauthor_with_non_existing_coauthor_key() {
        let key = "em";
        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_get()
            .with(predicate::eq(key))
            .once()
            .return_const(None);

        let coauthor_cmd = Coauthor {
            delete: Some(key.to_owned()),
            add: None,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);

        assert_eq!(
            err,
            format!("No co-author found with key: {key}\n").as_bytes()
        );
    }

    #[test]
    fn test_add_coauthor() {
        let key = "lm";
        let name = "Leo Messi";
        let email = "leo.messi@example.com";

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_add()
            .with(
                predicate::eq(key),
                predicate::eq(format!("{name} <{email}>")),
            )
            .once()
            .return_const(());

        let coauthor_cmd = Coauthor {
            add: Some(vec![key.to_owned(), name.to_owned(), email.to_owned()]),
            delete: None,
            list: false,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);

        assert_eq!(out, format!("{name} <{email}>\n").as_bytes());
    }

    #[test]
    fn test_list_coauthors() {
        let coauthors = vec![
            "lm Leo Messi <leo.messi@example.com>".to_owned(),
            "em Emi Martinez <emi.martinez@example.com>".to_owned(),
        ];

        let mut mock_coauthor_repo = MockCoauthorRepo::new();
        mock_coauthor_repo
            .expect_list()
            .once()
            .return_const(coauthors.to_owned());

        let coauthor_cmd = Coauthor {
            list: true,
            delete: None,
            add: None,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        coauthor_cmd.handle(&mock_coauthor_repo, &mut out, &mut err);

        assert_eq!(out, format!("{}\n", coauthors.join("\n")).as_bytes());
    }
}
