//! Native console styling; the focus slot stays fixed even when its artwork grows.
use super::*;

pub(super) fn tile(out: &mut Scene, state: &State, assets: &Assets, app: &App, slot: Rect) {
    let focused = state.focus == Target::App(app.id.clone());
    out.control(Target::App(app.id.clone()), slot, true);
    let viewport = state.favorites_viewport();
    if slot.x + slot.w < viewport.x || slot.x > viewport.x + viewport.w {
        return;
    }
    let size = if focused { slot.w } else { 184. };
    let rect = Rect::new(slot.x, slot.y + slot.h - size, size, size);
    let radius = state.preferences.console.radius();
    let start = out.draw.len();
    out.shadow(rect, radius, 28., 8., 0.32);
    app_face(out, assets, app, rect, state.preferences.language, radius);
    if focused {
        let accent = c(
            crate::console::ACCENTS[state.preferences.console.accent].1,
            1.,
        );
        out.border(rect.inset(-5.), radius + 5., 3., accent);
        let caption = Rect::new(rect.x, rect.y + rect.h - 65., rect.w, 65.);
        out.shape(caption, radius, c(0x071009, 0.9));
        let style = assets.fit_text(&app.title, text(21., 550, -0.02), rect.w - 28.);
        out.text(
            &app.title,
            Rect::new(rect.x + 14., caption.y + 20., rect.w - 28., 31.),
            style,
            WHITE,
            false,
        );
    }
    if state.moving.is_some() {
        if state.moving.as_ref() == Some(&app.id) {
            out.icon(
                assets,
                "left",
                Rect::new(rect.x + 12., rect.y + 12., 24., 24.),
                WHITE,
            );
            out.icon(
                assets,
                "arrow",
                Rect::new(rect.x + rect.w - 36., rect.y + 12., 24., 24.),
                WHITE,
            );
        } else {
            out.fade(start, 0.48, 1.);
        }
    }
    if state.pressed.as_ref() == Some(&Target::App(app.id.clone())) {
        out.fade(start, 1., 0.85);
    }
}

pub(super) fn hero(out: &mut Scene, state: &State, assets: &Assets, now: OffsetDateTime) {
    let language = state.preferences.language;
    let top = 184. - state.scroll;
    out.text(
        "LUMA CONSOLE",
        Rect::new(84., top, 1000., 28.),
        text(18., 600, 0.14),
        c(
            crate::console::ACCENTS[state.preferences.console.accent].1,
            1.,
        ),
        false,
    );
    if state.preferences.show_greeting {
        let greeting = language.greeting(now.hour(), &state.preferences.greeting_name);
        out.text(
            &greeting,
            Rect::new(84., top + 45., 1140., 65.),
            assets.fit_text(&greeting, text(48., 500, -0.04), 1140.),
            WHITE,
            false,
        );
    }
    if state.preferences.show_date {
        out.text(
            state.preferences.date_format.format(language, now),
            Rect::new(84., top + 122., 1100., 35.),
            text(21., 450, 0.),
            c(0xffffff, 0.75),
            false,
        );
    }
    if state.preferences.show_weather {
        weather_card(out, state, assets, top);
    }
}

fn weather_card(out: &mut Scene, state: &State, assets: &Assets, top: f32) {
    let language = state.preferences.language;
    let rect = Rect::new(1376., top, 460., 172.);
    out.shape(rect, state.preferences.console.radius(), c(0x10151c, 0.8));
    out.border(
        rect,
        state.preferences.console.radius(),
        1.,
        c(0xffffff, 0.12),
    );
    let (place, detail, icon) = if let Some(place) = &state.preferences.weather_location {
        if let Some(reading) = state.weather.reading(place) {
            let condition = weather::condition(reading);
            (
                place.name.clone(),
                format!(
                    "{}  {}{}",
                    weather::format_temperature(
                        reading.temperature,
                        state.preferences.temperature_unit
                    ),
                    language.t(condition.key),
                    if state.weather.failed {
                        format!(" · {}", language.t("lastReading"))
                    } else {
                        String::new()
                    }
                ),
                format!("weather-{}", condition.icon),
            )
        } else {
            (
                place.name.clone(),
                language
                    .t(if state.weather.loading {
                        "loadingWeather"
                    } else {
                        "weatherUnavailable"
                    })
                    .into(),
                "weather-cloud".into(),
            )
        }
    } else {
        (
            language.t("weather").into(),
            language.t("setWeatherLocation").into(),
            "location".into(),
        )
    };
    out.icon(
        assets,
        &icon,
        Rect::new(rect.x + 24., top + 24., 38., 38.),
        WHITE,
    );
    out.text(
        &place,
        Rect::new(rect.x + 78., top + 30., 358., 35.),
        assets.fit_text(&place, text(23., 550, 0.), 358.),
        WHITE,
        false,
    );
    out.text(
        &detail,
        Rect::new(rect.x + 24., top + 91., 412., 55.),
        assets.fit_text(&detail, text(23., 450, 0.), 412.),
        c(0xffffff, 0.78),
        false,
    );
    if state.preferences.weather_location.is_none() {
        if state.focus == Target::WeatherConfigure {
            out.border(
                rect.inset(-4.),
                state.preferences.console.radius() + 4.,
                2.,
                WHITE,
            );
        }
        out.control(Target::WeatherConfigure, rect, true);
    }
}

pub(super) fn settings(out: &mut Scene, state: &State, assets: &Assets) {
    let appearance = &state.preferences.console;
    let on_off = |enabled| if enabled { "On" } else { "Off" };
    let dim = format!("{}%", appearance.dim);
    for (position, (index, title, value)) in [
        (10, "Console layout", on_off(appearance.enabled)),
        (
            11,
            "Accent color",
            crate::console::ACCENTS[appearance.accent].0,
        ),
        (
            12,
            "Tile corners",
            if appearance.square_corners {
                "Square"
            } else {
                "Rounded"
            },
        ),
        (13, "Background dimming", dim.as_str()),
        (
            14,
            "Per-app backgrounds",
            on_off(appearance.app_backgrounds),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        row(
            out,
            state,
            assets,
            Row {
                target: Target::Setting(index),
                rect: Rect::new(
                    640.,
                    164.8 + (position + 4) as f32 * 84. - state.scroll,
                    1120.,
                    72.,
                ),
                title,
                icon: None,
                value: Some(value),
                description: None,
                selected: false,
                chevron: false,
            },
        );
    }
}
