use config::configure_from_argv;
use progressing::{clamping::Bar as ClampingBar, Baring};

use crate::config::LabelPlacement;

mod config;

fn main() {
    let config::Config {
        from,
        to,
        at,
        length,
        label,
        label_placement,
        style,
    } = configure_from_argv();

    let mut progress_bar = ClampingBar::new();

    progress_bar.set(at as f64 / (to - from) as f64);

    progress_bar.set_len(length);
    progress_bar.set_style(style);

    let output = match label_placement {
        _ if label.is_empty() => format!("{}", progress_bar),
        LabelPlacement::Left => format!("{} {}", label, progress_bar),
        LabelPlacement::Right => format!("{} {}", progress_bar, label),
    };

    println!("{}", output);
}
