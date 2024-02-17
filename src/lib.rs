use std::{fs, process::Command};

use dirs::home_dir;

#[derive(PartialEq, Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
}

pub fn get_current_theme() -> Result<Theme, &'static str> {
    let output = Command::new("defaults")
        .args(["read", "-g", "AppleInterfaceStyle"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(Theme::Dark)
            } else {
                // Assuming light mode if the command fails, as it does not exist in light mode
                Ok(Theme::Light)
            }
        }
        Err(_) => Err("Failed to execute command"),
    }
}

pub fn update_alacritty_config(theme: Theme) -> Result<(), std::io::Error> {
    let mut path = home_dir().unwrap();
    path.push(".config/alacritty/alacritty.toml");

    let content = fs::read_to_string(path.clone())?;

    let new_content = content
        .lines()
        .map(|line| {
            if line.starts_with("import = [\"~/.config/alacritty/themes/themes/catppuccin") {
                match theme {
                    Theme::Dark => {
                        "import = [\"~/.config/alacritty/themes/themes/catppuccin_macchiato.toml\"]"
                    }
                    Theme::Light => {
                        "import = [\"~/.config/alacritty/themes/themes/catppuccin_latte.toml\"]"
                    }
                }
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(path, new_content)?;
    Ok(())
}
