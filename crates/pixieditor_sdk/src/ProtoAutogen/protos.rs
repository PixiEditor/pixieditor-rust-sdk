use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct Shortcut {
    #[prost(int32, tag = "1")]
    pub key: i32,

    #[prost(int32, tag = "2")]
    pub modifiers: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct ActionDisplayConfig {
    #[prost(string, tag = "1")]
    pub text: String,

    #[prost(int32, tag = "2")]
    pub modifiers: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct CustomToolConfig {
    #[prost(string, tag = "1")]
    pub name: String,

    #[prost(string, tag = "2")]
    pub icon: String,

    #[prost(string, tag = "3")]
    pub tool_tip: String,

    #[prost(message, optional, tag = "4")]
    pub default_shortcut: Option<Shortcut>,

    #[prost(bool, tag = "5")]
    pub supports_secondary_action_on_right_click: bool,

    #[prost(message, repeated, tag = "6")]
    pub actions_display_configs: Vec<ActionDisplayConfig>,

    #[prost(string, tag = "7")]
    pub common_tool_type: String,
}