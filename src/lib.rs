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

#[test]
fn test_render_default_cfg() {
    let cfg = config::Config::default();

    let output = render(&cfg);

    assert_eq!(output, "[>.................]");
}

#[test]
fn test_render_from_to_at() {
    let cfg = config::Config {
        from: 0,
        to: 100,
        at: 50,
        ..config::Config::default()
    };

    let output = render(&cfg);

    assert_eq!(output, "[=========>........]");
}

#[test]
fn test_render_length() {
    let cfg = config::Config {
        length: 10,
        ..config::Config::default()
    };

    let output = render(&cfg);

    assert_eq!(output, "[>.......]");
}

#[test]
fn test_render_label() {
    let cfg = config::Config {
        label: String::from("test"),
        ..config::Config::default()
    };

    let output = render(&cfg);

    assert_eq!(output, "[>.................] test");
}

#[test]
fn test_render_style() {
    let cfg = config::Config {
        from: 0,
        to: 100,
        at: 50,
        style: String::from("()+ }"),
        ..config::Config::default()
    };

    let output = render(&cfg);

    assert_eq!(output, "()))))))))+        }");
}
