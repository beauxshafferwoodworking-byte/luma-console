//! Console appearance preferences. Backgrounds are catalog IDs, never arbitrary paths or URLs.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const ACCENTS: [(&str, u32); 6] = [
    ("Xbox green", 0x83d744),
    ("Electric blue", 0x54b9ff),
    ("Violet", 0xb38bff),
    ("Coral", 0xff876f),
    ("Gold", 0xffcf66),
    ("White", 0xffffff),
];

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Appearance {
    pub enabled: bool,
    pub accent: usize,
    pub square_corners: bool,
    pub dim: u8,
    pub app_backgrounds: bool,
    pub backgrounds: BTreeMap<String, String>,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            // Existing preferences and upstream smoke fixtures retain their layout.
            enabled: false,
            accent: 0,
            square_corners: true,
            dim: 20,
            app_backgrounds: true,
            backgrounds: BTreeMap::new(),
        }
    }
}

impl Appearance {
    pub fn sanitize(&mut self) {
        self.accent = self.accent.min(ACCENTS.len() - 1);
        self.dim = self.dim.min(60);
        self.backgrounds.retain(|app, background| {
            crate::model::valid_app_id(app)
                && (crate::model::BACKGROUNDS
                    .iter()
                    .any(|(id, _)| *id == background.as_str())
                    || crate::wallpapers::valid_id(background))
        });
        // Same bound as saved favorite IDs; settings files cannot create an unbounded catalog.
        while self.backgrounds.len() > 1000 {
            self.backgrounds.pop_last();
        }
    }

    pub fn radius(&self) -> f32 {
        if self.square_corners { 4. } else { 20. }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_paths_and_bounds_render_values() {
        let mut appearance = Appearance {
            accent: usize::MAX,
            dim: u8::MAX,
            ..Appearance::default()
        };
        appearance
            .backgrounds
            .insert("com.plex.app".into(), "forest".into());
        appearance
            .backgrounds
            .insert("com.other.app".into(), "../../etc/passwd".into());
        appearance
            .backgrounds
            .insert("invalid id".into(), "dunes".into());
        appearance.sanitize();
        assert_eq!(appearance.backgrounds.len(), 1);
        assert_eq!(appearance.accent, ACCENTS.len() - 1);
        assert_eq!(appearance.dim, 60);
    }

    #[test]
    fn old_preferences_keep_classic_layout() {
        let preferences: crate::model::Preferences = serde_json::from_str("{}").unwrap();
        assert!(!preferences.console.enabled);
        assert!(preferences.console.app_backgrounds);
    }

    #[test]
    fn console_defaults_use_only_installed_preferred_services_and_preserve_saved_order() {
        use crate::model::{App, Preferences};
        let mut preferences = Preferences::default();
        preferences.console.enabled = true;
        let apps = vec![
            App::new("com.apple.appletv", "Apple TV"),
            App::new("com.crunchyroll.webos", "Crunchyroll"),
            App::new("plex", "Plex"),
            App::new("com.disney.disneyplus-prod", "Disney+"),
            App::new("netflix", "Netflix"),
        ];
        assert_eq!(
            preferences.favorites(&apps),
            vec![
                "plex",
                "com.disney.disneyplus-prod",
                "com.crunchyroll.webos",
                "com.apple.appletv"
            ]
        );
        preferences.favorites = Some(vec!["com.apple.appletv".into(), "plex".into()]);
        assert_eq!(
            preferences.favorites(&apps),
            vec!["com.apple.appletv", "plex"]
        );
    }

    #[test]
    fn app_background_is_transient_and_missing_upload_falls_back() {
        use crate::state::{Page, State, Target};
        let mut state = State::new(crate::model::Preferences::default());
        state.preferences.console.enabled = true;
        state
            .preferences
            .console
            .backgrounds
            .insert("com.plex.app".into(), "forest".into());
        state.focus = Target::App("com.plex.app".into());
        state.tick();
        assert_eq!(state.console_background(), "forest");
        assert_eq!(state.preferences.background, "dunes");
        assert_eq!(state.preferences.wallpaper_changed, 0);
        state.page = Page::Settings;
        assert_eq!(state.console_background(), "dunes");
        state.page = Page::Home;
        state
            .preferences
            .console
            .backgrounds
            .insert("com.plex.app".into(), "uploaded-0123456789abcdef".into());
        assert_eq!(state.console_background(), "dunes");
    }

    #[test]
    fn assignment_and_clear_save_without_launching() {
        use crate::state::{Effect, Overlay, State, Target};
        let mut state = State::new(crate::model::Preferences::default());
        let id = "com.plex.app".to_owned();
        state.overlay = Some(Overlay::Options {
            id: id.clone(),
            restore: Target::App(id.clone()),
        });
        state.focus = Target::AppBackground;
        assert_eq!(state.activate(), vec![Effect::Save]);
        assert_eq!(
            state
                .preferences
                .console
                .backgrounds
                .get(&id)
                .map(String::as_str),
            Some("dunes")
        );
        state.overlay = Some(Overlay::Options {
            id: id.clone(),
            restore: Target::App(id.clone()),
        });
        state.focus = Target::ClearAppBackground;
        assert_eq!(state.activate(), vec![Effect::Save]);
        assert!(!state.preferences.console.backgrounds.contains_key(&id));
    }
}
