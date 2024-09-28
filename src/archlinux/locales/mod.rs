use crate::ai::gemini::assistant;
use crate::os::Os;
use inquire::Editor;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Error, ErrorKind, Write};
use std::process::Command;

pub async fn configure_locale(app: &mut Os) -> Result<(), Error> {
    assert!(File::create("/etc/locale.conf").is_ok());
    #[cfg(feature = "ai")]
    assert!(assistant(
        "Set your locale.",
        "Configure your system's language, region, and character encoding settings.",
        "Display language: Ensure your system uses your preferred language for menus, messages, and applications.\nFormatting: Get correct date, time, currency, and number formats based on your region.\nCharacter support: Handle special characters and symbols used in your language properly.",
        "User-friendly experience: Interact with your system in a familiar and comfortable way.\nAvoid errors: Prevent issues with software that relies on specific locale settings.\nImproved accessibility: Make your system more accessible to users who speak different languages or use assistive technologies.",
        "locale").await.is_ok());
    app.locale.clear();
    app.locale.push_str(
        Editor::new("Edition of /etc/locale.gen")
            .with_predefined_text(
                read_to_string("/etc/locale.gen")
                    .expect("failed to parse locale")
                    .as_str(),
            )
            .with_editor_command("vim".as_ref())
            .prompt()
            .expect("")
            .as_str(),
    );
    if let Ok(mut l) = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open("/etc/locale.gen")
    {
        l.write_all(format!("{}", app.locale).as_bytes())
            .expect("failed to write locale");
        l.sync_all().expect("failed to sync locale");
        l.flush().expect("failed to flush locale");
        if Command::new("locale-gen")
            .spawn()
            .expect("locale-gen")
            .wait()
            .expect("locale")
            .success()
        {
            app.locale.clear();
            app.locale.push_str(
                Editor::new("Edition of /etc/locale.conf")
                    .with_predefined_text(
                        read_to_string("/etc/locale.conf")
                            .expect("failed to parse locale.conf")
                            .as_str(),
                    )
                    .with_editor_command("vim".as_ref())
                    .prompt()
                    .expect("")
                    .as_str(),
            );
            if let Ok(mut l) = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .truncate(true)
                .open("/etc/locale.conf")
            {
                l.write_all(format!("{}", app.locale).as_bytes())
                    .expect("failed to write locale.conf");
                l.sync_all().expect("failed to sync locale.conf");
                l.flush().expect("failed to flush locale.conf");
            }
            return Ok(());
        }
        return Err(Error::new(ErrorKind::NotFound, "locale-gen"));
    }
    Err(Error::new(ErrorKind::Interrupted, "timezone failed"))
}
