use std::{env, fmt::Debug, str::FromStr};

pub struct Config {
    pub from: i32,
    pub to: i32,
    pub at: i32,
    pub length: usize,
    pub label: String,
    pub label_placement: LabelPlacement,
    pub style: String,
}

pub enum LabelPlacement {
    Left,
    Right,
}

fn expect_param<T>(desc: &str, param: Option<&String>) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let param = param.unwrap_or_else(|| panic!("{} needs a parameter", desc));
    let param = T::from_str(param);

    param.unwrap_or_else(|_| {
        panic!(
            "parameter of {} must parse into {}",
            desc,
            std::any::type_name::<T>()
        )
    })
}

pub fn configure_from_argv() -> Config {
    let mut cfg = Config {
        length: 20,
        from: 0,
        to: 20,
        at: 0,
        label: String::from(""),
        label_placement: LabelPlacement::Right,
        style: String::from("[=>.]"),
    };

    let mut args = env::args();

    args.next(); // shift program name

    while let Some(next) = args.next() {
        match next.as_str() {
            "--length" => cfg.length = expect_param("--length", args.next().as_ref()),
            "--from" => cfg.from = expect_param("--from", args.next().as_ref()),
            "--to" => cfg.to = expect_param("--to", args.next().as_ref()),
            "--at" => cfg.at = expect_param("--at", args.next().as_ref()),
            "--label" => cfg.label = expect_param("--label", args.next().as_ref()),
            "--left" => cfg.label_placement = LabelPlacement::Left,
            "--style" => cfg.style = expect_param("--style", args.next().as_ref()),
            _ => (),
        }
    }

    cfg
}
