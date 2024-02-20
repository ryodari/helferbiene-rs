use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TextComponentType {
    Text,
    Translatable,
    Keybind,
    Score,
    Selector,
    Nbt,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextComponentClick {
    pub action: TextComponentClickAction,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextComponent {
    #[serde(rename = "type")]
    pub type_: Option<TextComponentType>,
    pub extra: Option<Vec<Self>>,

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
