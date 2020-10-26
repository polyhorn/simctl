//! Supporting types for the `simctl push` subcommand.

use serde::Serialize;
use std::process::Stdio;

use super::{Device, Result, Validate};

/// Represents a push notification that can be sent to a device.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Push {
    /// Contains the payload of this push notification.
    pub aps: PushPayload,
}

/// Alert that is presented to the user.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PushAlert {
    /// Title that is shown to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Subtitle that is shown to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,

    /// Body that is shown to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Path to a launch image contained in the app bundle that will be shown to
    /// the user when the user opens the notification and has to wait for the
    /// application to launch.
    #[serde(rename = "launch-image", skip_serializing_if = "Option::is_none")]
    pub launch_image: Option<String>,

    /// Key of a localized string that will be used as a title in lieu of
    /// [`PushAlert::title`].
    #[serde(rename = "title-loc-key", skip_serializing_if = "Option::is_none")]
    pub title_loc_key: Option<String>,

    /// Arguments that are passed to the localized title that will be shown to
    /// the user. The number of arguments should equal the number of `%@`
    /// formatters in the localized string.
    #[serde(rename = "title-loc-args", skip_serializing_if = "Option::is_none")]
    pub title_loc_args: Option<Vec<String>>,

    /// Key of a localized string that will be used as a subtitle in lieu of
    /// [`PushAlert::subtitle`].
    #[serde(rename = "subtitle-loc-key", skip_serializing_if = "Option::is_none")]
    pub subtitle_loc_key: Option<String>,

    /// Arguments that are passed to the localized subtitle that will be shown
    /// to the user. The number of arguments should equal the number of `%@`
    /// formatters in the localized string.
    #[serde(rename = "subtitle-loc-args", skip_serializing_if = "Option::is_none")]
    pub subtitle_loc_args: Option<Vec<String>>,

    /// Key of a localized string that will be used as body in lieu of
    /// [`PushAlert::body`].
    #[serde(rename = "loc-key", skip_serializing_if = "Option::is_none")]
    pub loc_key: Option<String>,

    /// Arguments that are passed to the localized body that will be shown to
    /// the user. The number of arguments should equal the number of `%@`
    /// formatters in the localized string.
    #[serde(rename = "loc-args", skip_serializing_if = "Option::is_none")]
    pub loc_args: Option<Vec<String>>,
}

/// Sound that is played through the device's speakers when a push notification
/// arrives.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PushSound {
    /// Enables "critical" push sound.
    pub critical: usize,

    /// Name of the sound file in the app's bundle that will be played.
    pub name: String,

    /// Volume that will be used to play the sound.
    pub volume: f32,
}

/// Payload of a push notification that is sent to a device.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PushPayload {
    /// Optional alert that will be presented to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<PushAlert>,

    /// Optional number that will update the badge on the springboard. Set this
    /// to `Some(0)` to remove an existing badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<usize>,

    /// Optional sound that will be played when the notification arrives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<PushSound>,

    /// Optional thread id that is used by the OS to group multiple messages
    /// that are related to the same "thread" (e.g. conversation or topic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,

    /// Category that matches with one of the categories registered in the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Flag that indicates if content is available (should be either 0 or 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_available: Option<usize>,

    /// Flag that indicates if this payload should be run through the push
    /// notification extension of this app to update its content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<usize>,

    /// Content ID that is passed to the app when this notification is opened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_content_id: Option<String>,
}

impl Device {
    /// Sends the given push message to this device for an app with the given
    /// bundle ID.
    pub fn push(&self, bundle_id: &str, push: &Push) -> Result<()> {
        let mut process = self
            .simctl()
            .command("push")
            .arg(&self.udid)
            .arg(bundle_id)
            .arg("-")
            .stdin(Stdio::piped())
            // .stdout(Stdio::inherit())
            .spawn()?;

        if let Some(stdin) = process.stdin.as_mut() {
            serde_json::to_writer(stdin, push)?;
        }

        process.wait_with_output()?.validate()
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::mock;

    #[test]
    #[serial]
    fn test_push() -> Result<()> {
        mock::device()?.boot()?;
        mock::device()?.push(
            "com.apple.mobilecal",
            &Push {
                aps: PushPayload {
                    alert: Some(PushAlert {
                        body: Some("Hello World!".to_owned()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
        )?;
        mock::device()?.shutdown()?;

        Ok(())
    }
}
