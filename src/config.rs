use std::{env, fmt::Debug, str::FromStr};
pub struct Config {
    pub from: i32,
    pub to: i32,
    pub at: i32,
    pub length: usize,
    pub label: String,
    pub label_placement: LabelPlacement,
    pub style: String,
    pub no_newline: bool,
}
pub enum LabelPlacement {
    Left,
    Right,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            from: 0,
            to: 100,
            at: 0,
            length: 20,
            label: String::from(""),
            label_placement: LabelPlacement::Right,
            style: String::from("[=>.]"),
            no_newline: false,
        }
    }
}

fn expect_param<T>(desc: &str, param: Option<&String>) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let param = param.unwrap_or_else(|| panic!("{desc} needs a parameter"));
    let param = T::from_str(param);

    param.unwrap_or_else(|_| {
        panic!(
            "parameter of {} must parse into {}",
            desc,
            std::any::type_name::<T>()
        )
    })
}

pub enum ParseResult {
    Ok(Config),
    ShowHelp,
    ErrorUnknownArgument(String),
}

pub fn from_argv() -> ParseResult {
    let mut cfg = Config::default();

    let mut args = env::args();

    args.next(); // shift program name

    while let Some(next) = args.next() {
        match next.as_str() {
            "--length" => cfg.length = expect_param("--length", args.next().as_ref()),
            "--from" => cfg.from = expect_param("--from", args.next().as_ref()),
            "--to" => cfg.to = expect_param("--to", args.next().as_ref()),
            "--at" => {
                let param = args.next().expect("--at needs a parameter");
                if param.ends_with('%') {
                    cfg.from = 0;
                    cfg.to = 100;
                    cfg.at = i32::from_str(&param[..param.len() - 1]).expect("--at needs a number");
                } else {
                    cfg.at = i32::from_str(&param).expect("--at needs a number");
                }
            }
            "--label" => cfg.label = expect_param("--label", args.next().as_ref()),
            "--left" => cfg.label_placement = LabelPlacement::Left,
            "--style" => cfg.style = expect_param("--style", args.next().as_ref()),
            "--help" | "-h" | "/?" => return ParseResult::ShowHelp,
            "--no-newline" => cfg.no_newline = true,
            arg => return ParseResult::ErrorUnknownArgument(arg.to_owned()),
        }
    }

    ParseResult::Ok(cfg)
}

pub fn usage() -> String {
    "
Usage:
progredient --at X%
progredient --at Y --from X --to Z

Optional arguments:
--length L        stretch the bar to be this number of chars long
--style ABCDE     render the bar as 'ABBBBBBCDDDE' with C positioned at --at
--label LABEL     show this text after the bar
--left            show the label before the bar instead
--help            show this information and exit
--no-newline      do not print a newline after the bar
"
    .to_owned()
}
