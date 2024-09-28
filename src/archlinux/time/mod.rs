use crate::ai::gemini::assistant;
use crate::configuration::system::TIMEZONES;
use crate::os::Os;
use inquire::Select;
use std::io::{Error, ErrorKind};
use std::process::Command;

pub async fn configure_timezone(app: &mut Os) -> Result<(), Error> {
    #[cfg(feature = "ai")]
    assert!(assistant(
        "Configure your time zone.",
        "Set your system's time zone to match your physical location.",
        "Accurate time display: Ensure your system clock shows the correct time based on your location.\nSchedule coordination: Synchronize your calendar, appointments, and events with others in your time zone.\nNetwork communication: Maintain proper timestamps for emails, file transfers, and other online activities.",
        "Avoid confusion: Prevent misunderstandings due to incorrect time displays.\nImprove productivity: Stay organized and on schedule with accurate timekeeping.\nFacilitate collaboration: Seamlessly interact with others across different time zones.\n",
        "time zone").await.is_ok());
    loop {
        app.time_zone.clear();
        app.time_zone.push_str(
            Select::new("Select your timezone :", TIMEZONES.to_vec())
                .prompt()
                .expect("bad timezone"),
        );
        if app.time_zone.is_empty().eq(&false) {
            break;
        }
    }
    if Command::new("ln")
        .arg("-sfv")
        .arg(app.time_zone.as_str())
        .arg("/etc/localtime")
        .spawn()
        .expect("linux")
        .wait()
        .expect("")
        .success()
    {
        return Ok(());
    }
    Err(Error::new(ErrorKind::Interrupted, "timezone failed"))
}
