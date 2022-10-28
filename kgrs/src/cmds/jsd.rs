//! For Japanese Stable Diffusion

use crate::util::fmt::cb;
use serde::{Deserialize, Serialize};
use serenity::{
    client::Context,
    http::typing::Typing,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::component::ButtonStyle,
    },
    model::{
        channel::AttachmentType,
        prelude::interaction::message_component::MessageComponentInteraction,
    },
};
use std::{collections::HashMap, env};

#[derive(Serialize)]
pub struct JSDRequest {
    /// The subject of the image.
    pub prompts: String,
    /// Higher values keep your image closer to your prompt(0-20)
    pub scale: f64,
}

impl JSDRequest {
    /// Create a new request.
    ///
    /// # Arguments
    ///
    /// - `prompts` - The subject of the image.
    ///
    /// - `scale` - Higher values keep your image closer to your prompt(0-20)
    ///
    /// # Panics
    ///
    /// Panics if `scale` is not between 0 and 20.
    fn new(prompts: String, scale: f64) -> Self {
        if !(0. ..=20.).contains(&scale) {
            panic!("Argument `scale` must be between 0 and 20.");
        }
        Self { prompts, scale }
    }

    /// Send the request.
    async fn send(&self) -> Result<JSDResponse, JSDError> {
        let client = reqwest::Client::new()
            .post("https://api.rinna.co.jp/models/tti/v2")
            .json(self)
            .header(
                "Ocp-Apim-Subscription-Key",
                env::var("KGRS_RINNA_TTI_KEY")
                    .expect("Expected a subscription key for the TTI in the environment"),
            )
            .send();
        match client.await {
            Ok(res) => match res.json::<JSDResponse>().await {
                Ok(body) => Ok(body),
                Err(err) => Err(JSDError::DeserializeErr(err)),
            },
            Err(err) => Err(JSDError::RequestErr(err)),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct JSDResponse {
    /// The image data(base64 encoded)
    //#[serde(default)]
    pub image: String,
    /// Whether the image is inappropriate.
    #[serde(rename = "nsfwContentDetected")]
    //#[serde(default)]
    pub is_sensitive: bool,
    //statusCode: Option<u16>
}

#[derive(Debug)]
enum JSDError {
    DeserializeErr(reqwest::Error),
    RequestErr(reqwest::Error),
}

pub async fn jsd_interact_to_discord(
    ctx: &Context,
    interact: &Interaction<'_>,
    subject: String,
    dict: HashMap<String, (String, String)>,
) {
    let locale = match interact {
        Interaction::AppCmd(i) => &i.locale,
        Interaction::MsgCmp(i) => &i.locale,
    };
    let dict_lookup = |dict: &HashMap<String, (String, String)>, key: &str| {
        let s = if let Some(s) = dict.get(key) {
            s
        } else {
            panic!("Invalid dict key: {}", key);
        };
        if locale == "ja" {
            s.1.clone()
        } else {
            s.0.clone()
        }
    };

    let channel_id = match interact {
        Interaction::AppCmd(i) => i.channel_id,
        Interaction::MsgCmp(i) => i.channel_id,
    };
    let _typing = Typing::start(ctx.http.clone(), channel_id.as_u64().to_owned());

    match JSDRequest::new(subject.clone(), 10.).send().await {
        Ok(body) => {
            // Image preparation
            let img = base64::decode(body.image.replace("data:image/png;base64,", "").as_bytes())
                .expect("Failed to decode");
            let is_broken = body.image.replace("data:image/png;base64,", "").len() == 3560;

            // Delete the message "Please wait..."
            let ftdtm = "Failed to delete the message: ";
            match interact {
                Interaction::AppCmd(i) => {
                    if let Err(why) = i.delete_original_interaction_response(&ctx.http).await {
                        eprintln!("{}{}", ftdtm, why);
                    };
                }
                Interaction::MsgCmp(i) => {
                    if let Err(why) = i.message.delete(&ctx.http).await {
                        eprintln!("{}{}", ftdtm, why);
                    };
                }
            }

            // Send the image
            if let Err(why) = channel_id
                .send_files(
                    &ctx.http,
                    vec![AttachmentType::Bytes {
                        data: img.into(),
                        filename: "jsd.png".into(),
                    }],
                    |m| {
                        let username = match interact {
                            Interaction::AppCmd(i) => &i.user.name,
                            Interaction::MsgCmp(i) => &i.user.name,
                        };
                        m.content(format!(
                            "{}: {} |  {}{}{}{}{}",
                            dict_lookup(&dict, "subject"),
                            subject,
                            dict_lookup(&dict, "calledBy.before"),
                            username,
                            dict_lookup(&dict, "calledBy.after"),
                            if body.is_sensitive {
                                dict_lookup(&dict, "sensitiveFrag")
                            } else {
                                String::new()
                            },
                            if is_broken {
                                dict_lookup(&dict, "imageBrokenFrag")
                            } else {
                                String::new()
                            },
                        ));
                        if is_broken || body.is_sensitive {
                            m.components(|c| {
                                c.create_action_row(|a| {
                                    a.create_button(|b| {
                                        b.label(dict_lookup(&dict, "btn.retry"))
                                            .style(ButtonStyle::Primary)
                                            .custom_id("jsdRetry")
                                    })
                                })
                            });
                        }
                        m
                    },
                )
                .await
            {
                eprintln!("Failed to send the image: {}", why);
            }
        }
        Err(err) => match err {
            JSDError::RequestErr(why) => {
                if let Interaction::AppCmd(i) = interact {
                    if let Err(why) = i
                        .edit_original_interaction_response(&ctx.http, |m| {
                            m.content(cb(format!("Error: {}", why), ""))
                        })
                        .await
                    {
                        eprintln!("Failed to edit the message: {}", why);
                    };
                }
            }
            JSDError::DeserializeErr(_) => match interact {
                Interaction::AppCmd(i) => {
                    if let Err(why) = i
                        .edit_original_interaction_response(&ctx.http, |m| {
                            m.content(format!(
                                "{}\n{}: {} |",
                                dict_lookup(&dict, "err.plzRetry"),
                                dict_lookup(&dict, "subject"),
                                subject,
                            ))
                            .components(|c| {
                                c.create_action_row(|ar| {
                                    ar.create_button(|b| {
                                        b.label(dict_lookup(&dict, "btn.retry"))
                                            .style(ButtonStyle::Primary)
                                            .custom_id("jsdRetry")
                                    })
                                })
                            })
                        })
                        .await
                    {
                        eprintln!("Failed to edit message: {}", why);
                    };
                }
                Interaction::MsgCmp(i) => {
                    if let Err(why) = i
                        .clone()
                        .message
                        .edit(&ctx.http, |m| {
                            m.content(format!(
                                "{}\n{}: {} |",
                                dict_lookup(&dict, "err.plzRetry"),
                                dict_lookup(&dict, "subject"),
                                subject,
                            ))
                            .components(|c| {
                                c.create_action_row(|ar| {
                                    ar.create_button(|b| {
                                        b.label(dict_lookup(&dict, "btn.retry"))
                                            .style(ButtonStyle::Primary)
                                            .custom_id("jsdRetry")
                                    })
                                })
                            })
                        })
                        .await
                    {
                        eprintln!("Failed to edit message: {}", why);
                    };
                }
            },
        },
    }
}

#[allow(clippy::large_enum_variant)]
pub enum Interaction<'a> {
    AppCmd(&'a ApplicationCommandInteraction),
    MsgCmp(MessageComponentInteraction),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_request() {
        assert_eq!(
            JSDRequest::new("test".to_string(), 10.),
            JSDRequest {
                prompts: "test".to_string(),
                scale: 10.,
            }
        )
    }

    #[test]
    #[should_panic]
    fn panic_if_scale_is_under_0() {
        JSDRequest::new("test".to_string(), -0.1);
    }

    #[test]
    #[should_panic]
    fn panic_if_scale_is_over_20() {
        JSDRequest::new("test".to_string(), 20.1);
    }
}
