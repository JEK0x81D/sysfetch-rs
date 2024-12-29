use clap::Parser;
use crate::icon_art::IconArt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Whether to show distro info.
    #[arg(long)]
    pub(crate) show_distro: bool,

    /// Whether to show desktop environment.
    #[arg(long)]
    pub(crate) show_desktop_env: bool,

    /// Whether to show device name.
    #[arg(long)]
    pub(crate) show_device_name: bool,

    /// Whether to show architecture.
    #[arg(long)]
    pub(crate) show_architecture: bool,

    /// Whether to show uptime.
    #[arg(long)]
    pub(crate) show_uptime: bool,

    /// Whether to show number of packages installed.
    #[arg(long)]
    pub(crate) show_packages: bool,

    /// Whether to show motherboard name.
    #[arg(long)]
    pub(crate) show_motherboard: bool,

    /// Whether to show motherboard manufacturer name.
    #[arg(long)]
    pub(crate) show_motherboard_manufacturer: bool,

    /// Whether to show CPU name.
    #[arg(long)]
    pub(crate) show_cpu: bool,

    /// Whether to show CPU vendor.
    #[arg(long)]
    pub(crate) show_cpu_vendor: bool,

    /// Whether to show CPU frequency.
    #[arg(long)]
    pub(crate) show_cpu_frequency: bool,

    /// Whether to show CPU usage.
    #[arg(long)]
    pub(crate) show_cpu_usage: bool,

    /// Whether to show GPU info.
    #[arg(long)]
    pub(crate) show_gpu: bool,

    /// Whether to show memory usage.
    #[arg(long)]
    pub(crate) show_memory: bool,

    /// Whether to show swap usage.
    #[arg(long)]
    pub(crate) show_swap: bool,

    /// Whether to show local IP.
    #[arg(long)]
    pub(crate) show_ip: bool,

    /// Flag to show all entries.
    #[arg(short='a', long)]
    pub(crate) show_all: bool,

    /// Override for icon art.
    #[arg(long, value_enum, ignore_case=true)]
    pub(crate) icon_art: Option<IconArt>
}
