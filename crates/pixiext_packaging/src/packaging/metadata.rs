use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExtensionMetadata {
    pub uniqueName: String,
}