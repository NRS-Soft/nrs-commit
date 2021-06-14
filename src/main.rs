use commit::{
    get_optional_commit_body_and_footer, put_together_commit_message, put_together_first_line,
    CommitType,
};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use std::process::Command;

fn main() -> std::io::Result<()> {
    let commit_types = CommitType::default_commit_types();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select a header:")
        .items(&commit_types)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let commit_type = match selection {
        Some(index) => commit_types[index],
        None => panic!("Must select a commit type!"),
    };

    let scope: String = Input::new()
        .with_prompt("The scope of this change")
        .allow_empty(true)
        .interact_text()?;

    let subject: String = Input::new()
        .with_prompt("A short description for your commit")
        .validate_with(|input: &String| -> Result<(), &str> {
            if commit_type.text.len() + scope.len() + input.len() < 50 {
                Ok(())
            } else {
                Err("First line of commit should be less than 50")
            }
        })
        .interact_text()?;

    let first_line = put_together_first_line(commit_type, scope, subject);
    let other = get_optional_commit_body_and_footer();
    let commit_message = put_together_commit_message(first_line, other);

    Command::new("git")
        .args(&["commit", "-m", &commit_message])
        .status()
        .expect("Failed to git commit");

    Ok(())
}
