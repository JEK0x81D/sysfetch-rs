use std::fmt::{Display, Formatter};
use colored::Colorize;
use lazy_static::lazy_static;
use strum_macros::EnumString;

lazy_static! {
    static ref WINDOWS_TEXT: Vec<String> = vec![
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "                                  ".to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
        "################  ################".blue().to_string(),
    ];

    static ref ARCH_TEXT: Vec<String> = vec![
        "                   -`                 ".blue().to_string(),
        "                  .o+`                ".blue().to_string(),
        "                 `ooo/                ".blue().to_string(),
        "                `+oooo:               ".blue().to_string(),
        "               `+oooooo:              ".blue().to_string(),
        "               -+oooooo+:             ".blue().to_string(),
        "             `/:-:++oooo+:            ".blue().to_string(),
        "            `/++++/+++++++:           ".blue().to_string(),
        "           `/++++++++++++++:          ".blue().to_string(),
        "          `/+++ooooooooooooo/`        ".blue().to_string(),
        "         ./ooosssso++osssssso+`       ".blue().to_string(),
        "        .oossssso-````/ossssss+`      ".blue().to_string(),
        "       -osssssso.      :ssssssso.     ".blue().to_string(),
        "      :osssssss/        osssso+++.    ".blue().to_string(),
        "     /ossssssss/        +ssssooo/-    ".blue().to_string(),
        "   `/ossssso+/:-        -:/+osssso+-  ".blue().to_string(),
        "  `+sso+:-`                 `.-/+oso: ".blue().to_string(),
        " `++:.                           `-/+/".blue().to_string(),
        ".`                                 `  ".blue().to_string()
    ];

    static ref UNKNOWN_TEXT: Vec<String> = vec![
        "       ________       ".to_string(),
        "   _jgN########Ngg_    ".to_string(),
        " _N##N@@\"\"  \"\"9NN##Np_ ".to_string(),
        "d###P            N####p ".to_string(),
        "\"^^\"              T####".to_string(),
        "                  d###P ".to_string(),
        "               _g###@F  ".to_string(),
        "            _gN##@P     ".to_string(),
        "          gN###F\"       ".to_string(),
        "         d###F          ".to_string(),
        "        0###F           ".to_string(),
        "        0###F           ".to_string(),
        "        0###F           ".to_string(),
        "        \"NN@\'           ".to_string(),
        "                        ".to_string(),
        "         ___            ".to_string(),
        "        q###            ".to_string(),
        "         \"\"             ".to_string(),
    ];
}

#[derive(EnumString)]
pub(crate) enum IconArt {
    Windows,
    Arch,
    Unknown
}

impl IconArt {
    pub(crate) fn to_lines(&self) -> &Vec<String> {
        match self {
            IconArt::Windows => &WINDOWS_TEXT,
            IconArt::Arch => &ARCH_TEXT,
            _ => &UNKNOWN_TEXT
        }
    }

    pub(crate) fn get_length(&self) -> usize {
        if let Some(max_element) = self.to_lines().iter().max_by(|a, b| a.len().cmp(&b.len())) {
            strip_ansi_escapes::strip_str(max_element).len()
        } else {
            0
        }
    }
}

impl Display for IconArt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_lines().join("\n"))
    }
}