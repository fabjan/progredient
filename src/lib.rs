pub mod config;

use progressing::{clamping::Bar as ClampingBar, Baring};

pub fn render(cfg: &config::Config) -> String {
    let config::Config {
        from,
        to,
        at,
        length,
        label,
        label_placement,
        style,
        ..
    } = cfg;

    let mut progress_bar = ClampingBar::new();

    progress_bar.set((at - from) as f64 / (to - from) as f64);

    progress_bar.set_len(*length);
    progress_bar.set_style(style);

    match label_placement {
        _ if label.is_empty() => format!("{progress_bar}"),
        config::LabelPlacement::Left => format!("{label} {progress_bar}"),
        config::LabelPlacement::Right => format!("{progress_bar} {label}"),
    }
}
