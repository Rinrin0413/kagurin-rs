//! kgrs is a library for kagurin-rs.

pub mod cmd_mng;
pub mod response_interactions;
pub mod tetr;
pub mod util;
//pub mod playground;

// For Japanese Stable Diffusion
pub mod jsd {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize)]
    pub struct JSDRequest {
        /// The subject of the image.
        pub prompts: String,
        /// Higher values keep your image closer to your prompt(0-20)
        pub scale: f64,
    }

    #[derive(Deserialize, Debug)]
    pub struct JSDResponse {
        /// The image data(base64 encoded)
        pub image: String,
        /// Whether the image is inappropriate.
        #[serde(rename = "nsfwContentDetected")]
        pub is_sensitive: bool,
    }
}
