use std::fs::{read_to_string, File, OpenOptions};
use crate::os::Os;
use std::io::{Error, ErrorKind, Write};
use std::process::Command;

#[cfg(feature = "ai")]
use crate::ai::gemini::assistant;
#[cfg(feature = "ask")]
use inquire::Editor;


pub async fn configure_locale(app: &mut Os, expert: bool) -> Result<(), Error> {
    if let Ok(mut f) = File::create("/etc/locale.conf") {
        f.write_all("LANG=\nLANGUAGE=\nLC_TIME=\nLC_COLLATE=\nLC_ALL=".as_bytes())
            .expect("Unable to write to /etc/locale.conf");
        f.sync_all().expect("Unable to sync /etc/locale.conf");
        f.flush().expect("Unable to flush /etc/locale.conf");
    }
    #[cfg(feature = "ai")]
    assert!(assistant(
        "Set your locale.",
        "Configure your system's language, region, and character encoding settings.",
        "Display language: Ensure your system uses your preferred language for menus, messages, and applications.\nFormatting: Get correct date, time, currency, and number formats based on your region.\nCharacter support: Handle special characters and symbols used in your language properly.",
        "User-friendly experience: Interact with your system in a familiar and comfortable way.\nAvoid errors: Prevent issues with software that relies on specific locale settings.\nImproved accessibility: Make your system more accessible to users who speak different languages or use assistive technologies.",
        "locale", expert).await.is_ok());
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
            if let Ok(mut l) = OpenOptions::new().write(true).open("/etc/locale.conf") {
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
