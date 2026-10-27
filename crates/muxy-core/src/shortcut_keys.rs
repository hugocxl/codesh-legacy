//! Decoding of 1.x shortcut modifiers for profile imports.

use serde::Deserialize;

const SHIFT: u64 = 1 << 17;
const CONTROL: u64 = 1 << 18;
const OPTION: u64 = 1 << 19;
const COMMAND: u64 = 1 << 20;
const SUPPORTED_MODIFIER_MASK: u64 = SHIFT | CONTROL | OPTION | COMMAND;

#[derive(Debug, Deserialize)]
pub struct KeyCombo {
    pub key: String,
    pub modifiers: u64,
}

impl KeyCombo {
    #[cfg(test)]
    fn new(key: &str, modifiers: u64) -> Self {
        Self {
            key: key.into(),
            modifiers,
        }
    }

    fn is_assigned(&self) -> bool {
        !self.key.is_empty()
    }

    pub fn effective_modifiers(&self) -> u64 {
        let modifiers = self.modifiers & SUPPORTED_MODIFIER_MASK;
        if cfg!(target_os = "macos") || modifiers & COMMAND == 0 {
            return modifiers;
        }
        (modifiers & !COMMAND) | CONTROL
    }

    pub fn keystroke(&self) -> Option<String> {
        if !self.is_assigned() {
            return None;
        }
        let modifiers = self.effective_modifiers();
        let mut parts = Vec::new();
        if modifiers & CONTROL != 0 {
            parts.push("ctrl");
        }
        if modifiers & OPTION != 0 {
            parts.push("alt");
        }
        if modifiers & SHIFT != 0 {
            parts.push("shift");
        }
        if modifiers & COMMAND != 0 {
            parts.push("cmd");
        }
        parts.push(match self.key.as_str() {
            "leftarrow" => "left",
            "rightarrow" => "right",
            "uparrow" => "up",
            "downarrow" => "down",
            "return" => "enter",
            key => key,
        });
        Some(parts.join("-"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imported_command_modifiers_match_native_conflicts_without_rewriting_flags() {
        let native = KeyCombo::new("return", CONTROL | OPTION | SHIFT);
        for flags in [COMMAND | OPTION | SHIFT, COMMAND | CONTROL | OPTION | SHIFT] {
            let imported = KeyCombo::new("return", flags);
            if cfg!(target_os = "macos") {
                assert_ne!(imported.keystroke(), native.keystroke());
                assert_eq!(imported.effective_modifiers(), flags);
            } else {
                assert_eq!(imported.keystroke(), native.keystroke());
                assert_eq!(imported.keystroke(), native.keystroke());
                assert_eq!(imported.effective_modifiers(), native.modifiers);
            }
            assert_eq!(imported.modifiers, flags);
        }
    }
}
