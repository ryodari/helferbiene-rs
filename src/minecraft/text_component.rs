use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::packet::slp::SlpServerDescription;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TextComponentType {
    Text,
    Translatable,
    Keybind,
    Score,
    Selector,
    Nbt,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TextComponentClickAction {
    OpenUrl,
    OpenFile, // cannot be used within json chat, only exists internally to link screenshots in chat
    RunCommand,
    TwitchUserInfo, // removed in 1.9, only internally on the client and not usable with json chat
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextComponentClick {
    pub action: TextComponentClickAction,
    pub value: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextComponent {
    #[serde(rename = "type")]
    pub type_: Option<TextComponentType>,
    pub extra: Option<Vec<SlpServerDescription>>,

    // Type: text
    pub text: String,

    // Styling fields
    pub color: Option<String>, // color name or #hex rgb spec
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub font: Option<String>, // only valid on 1.16+
    pub insertion: Option<String>, // to insert text when clicked in chat
                              // pub click_event: Option<TextComponentClick>,
}

impl TextComponent {
    pub fn format_string(&self) -> String {
        match self.type_ {
            Some(TextComponentType::Text) => {}
            None => {}
            _ => return "".into(),
        };

        let mut res = self.text.clone();

        let mut is_wrapped = false;
        let mut wrap_str = |chars| {
            res.insert_str(0, chars);
            res.push_str(chars);
            is_wrapped = true;
        };

        let is_true = |arg: &Option<bool>| arg.is_some_and(|x| x == true);

        if is_true(&self.bold) && is_true(&self.italic) {
            wrap_str("***");
        } else if is_true(&self.bold) {
            wrap_str("**");
        } else if is_true(&self.italic) {
            wrap_str("*");
        }

        if is_true(&self.underlined) {
            wrap_str("__");
        }

        if is_true(&self.strikethrough) {
            wrap_str("~~");
        }

        if is_wrapped {
            res.push('\u{200B}'); // zero width to seperate markdown
        }

        if let Some(childs) = &self.extra {
            for child in childs {
                let child_string = match child {
                    SlpServerDescription::Simple(x) => x.clone(),
                    SlpServerDescription::Complex(x) => x.format_string(),
                };
                res.push_str(child_string.as_str());
            }
        }

        res
    }
}
