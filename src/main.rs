use std::str::FromStr;
use clap::Parser;
use itertools::{EitherOrBoth, Itertools};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use crate::cli_args::Cli;
use crate::icon_art::IconArt;
use crate::info_builder::build_info_lines;

mod icon_art;
mod cli_args;
mod info_builder;
mod duration_extras;
mod package_counter;

fn get_system_icon_art() -> IconArt {
    IconArt::from_str(
        System::name().unwrap_or_default().as_str()
    ).unwrap_or(IconArt::Unknown)
}

fn render(icon_art: IconArt, info_lines: &[&str]) {
    let icon_art_max_len = icon_art.get_length();

    println!();

    for pair in icon_art.to_lines().into_iter().zip_longest(info_lines) {
        match pair {
            EitherOrBoth::Both(left, right) => println!("  {}	{}", left, right),
            EitherOrBoth::Left(left) => println!("  {}", left),
            EitherOrBoth::Right(right) => println!("  {}	{}", " ".repeat(icon_art_max_len), right)
        }
    }
}

fn main() {
    let args = Cli::parse();
    let mut system = System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()).with_memory(MemoryRefreshKind::everything()));

    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again to get actual value.
    system.refresh_cpu_all();

    let info_lines = build_info_lines(&args, &system);
    render(args.icon_art.unwrap_or(get_system_icon_art()), &info_lines.iter().map(String::as_str).collect::<Vec<&str>>());
}
