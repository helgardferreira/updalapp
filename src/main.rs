use std::{thread, time::Duration};

use updalapp::{get_current_theme, update_alacritty_config, Theme::*};

fn main() {
    let mut initial_theme = get_current_theme().unwrap_or_else(|e| {
        eprintln!("Error getting initial theme: {}", e);
        Dark
    });

    update_alacritty_config(initial_theme).unwrap_or_else(|e| {
        eprintln!("Error updating initial alacritty config: {}", e);
    });

    loop {
        thread::sleep(Duration::from_secs(1)); // Check every 1 second

        match get_current_theme() {
            Ok(current_theme) => {
                if current_theme != initial_theme {
                    if let Err(e) = update_alacritty_config(current_theme) {
                        panic!("Failed to update alacritty config: {}", e);
                    }

                    initial_theme = current_theme;
                }
            }
            Err(e) => eprintln!("Error getting theme: {}", e),
        }
    }
}
