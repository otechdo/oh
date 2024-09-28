use crate::ai::gemini::assistant;
use crate::configuration::system::{KEYMAPS, KEYMAP_LAYOUTS, KEYMAP_MODELS, KEYMAP_OPTIONS};
use crate::{os::Os, select};
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

pub async fn configure_keyboard(app: &mut Os) -> Result<(), Error> {
    // Keyboard Keymap
    loop {
        #[cfg(feature = "ai")]
        assert!(assistant(
            "Set Your Keyboard Keymap",
            "Configure the way your keyboard keys are mapped to specific characters and functions.",
            "Match physical layout: Ensure your keyboard settings align with the actual keys on your keyboard.\nType in your language: Use the correct characters and symbols for your language.\nAccess special functions: Utilize keyboard shortcuts and special keys as intended.",
            "Accurate typing: Type efficiently and avoid errors due to mismatched key mappings.\nComfortable experience: Use your keyboard in a natural and intuitive way.\nImproved productivity: Access all keyboard functions and shortcuts easily.",
            "Keyboard layout configuration",
        ).await.is_ok());
        app.keyboard.keymap.clear();
        app.keyboard.keymap = select("Select a keymap :", KEYMAPS.map(String::from).to_vec());
        if app.keyboard.keymap.is_empty().eq(&false) {
            break;
        }
    }
    // Keyboard Layout
    loop {
        #[cfg(feature = "ai")]
        assert!(assistant(
            "Set Your Keyboard Layout",
            "Configure the way your keyboard keys are mapped to specific characters and functions.",
            "Match physical layout: Ensure your keyboard settings align with the actual keys on your keyboard.\nType in your language: Use the correct characters and symbols for your language.\nAccess special functions: Utilize keyboard shortcuts and special keys as intended.",
            "Accurate typing: Type efficiently and avoid errors due to mismatched key mappings.\nComfortable experience: Use your keyboard in a natural and intuitive way.\nImproved productivity: Access all keyboard functions and shortcuts easily.",
            "Keyboard layout configuration",
        ).await.is_ok());
        app.keyboard.layout.clear();
        app.keyboard.layout = select(
            "Select a layout :",
            KEYMAP_LAYOUTS.map(String::from).to_vec(),
        );
        if app.keyboard.layout.is_empty().eq(&false) {
            break;
        }
    }
    // Keyboard Model
    loop {
        #[cfg(feature = "ai")]
        assert!(assistant(
            "Configure Your Keyboard Model",
            "Ensure your system recognizes your specific keyboard model for optimal functionality.",
            "Correct key mapping: Accurate mapping of special keys (e.g., multimedia keys, function keys) specific to your keyboard model.\nDriver compatibility: Install appropriate drivers for advanced features or customizations.\nTroubleshooting: Resolve potential issues related to unrecognized keys or functions.",
            "Full keyboard utilization: Access all features and functions your keyboard offers.\nEnhanced user experience: Seamless interaction with your system using your specific keyboard model.\nImproved troubleshooting: Efficiently address any keyboard-related problems.",
            "Configure Your Keyboard Model",
        ).await.is_ok());
        app.keyboard.model.clear();
        app.keyboard.model = select(
            "Select a keyboard model :",
            KEYMAP_MODELS.map(String::from).to_vec(),
        );
        if app.keyboard.model.is_empty().eq(&false) {
            break;
        }
    }

    // Keyboard Options
    loop {
        #[cfg(feature = "ai")]
        assert!(assistant(
            "Configure Your Keyboard Options",
            "Customize various keyboard behaviors to match your preferences and needs.",
            "Adjust key repeat: Control how quickly a key repeats when held down.\nSet cursor blink rate: Change how fast the cursor blinks.\nEnable/disable sticky keys: Make modifier keys (e.g., Shift, Ctrl, Alt) stick after being pressed once, allowing you to press other keys in combination without holding the modifier key down continuously.\nConfigure other accessibility options: Adjust settings for accessibility features like slow keys or bounce keys.",
            "Personalized experience: Tailor your keyboard behavior to your typing style and workflow.\nImproved accessibility: Make your keyboard easier to use for individuals with specific needs.\nEnhanced comfort: Reduce strain and fatigue during extended keyboard use.",
            "Keyboard Options",
        ).await.is_ok());
        app.keyboard.options.clear();
        app.keyboard.options = select(
            "Select a keyboard options :",
            KEYMAP_OPTIONS.map(String::from).to_vec(),
        );
        if app.keyboard.options.is_empty().eq(&false) {
            break;
        }
    }

    // Save configuration
    // KEYMAP=fr
    // XKBLAYOUT=fr
    // XKBMODEL=pc105
    // XKBOPTIONS=terminate:ctrl_alt_bksp
    assert!(File::create("/etc/vconsole.conf").is_ok());
    if let Ok(mut console) = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/etc/vconsole.conf")
    {
        assert!(console
            .write_all(
                format!(
                    "KEYMAP={}\nXKBLAYOUT={}\nXKBMODEL={}\nXKBOPTIONS={}",
                    app.keyboard.keymap,
                    app.keyboard.layout,
                    app.keyboard.model,
                    app.keyboard.options
                )
                .as_bytes()
            )
            .is_ok());
        assert!(console.sync_all().is_ok());
        assert!(console.flush().is_ok());
        return Ok(());
    }
    Ok(())
}
