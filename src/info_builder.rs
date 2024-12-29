use colored::Colorize;
use frequency_unit::Frequency;
use humansize::{format_size, FormatSizeOptions};
use itertools::Itertools;
use local_ip_address::local_ip;
use smbioslib::{table_load_from_device, ProcessorSpeed, SMBiosBaseboardInformation, SMBiosProcessorInformation};
use sysinfo::{System};
use wgpu::DeviceType;
use crate::cli_args::Cli;
use crate::duration_extras::DisplayableDuration;

fn get_entry_formatted(label: &str, value: &str) -> String
{
    format!("{}: {}", label.yellow(), value)
}

fn get_total_cpu_usage(system: &System) -> f32 {
    system.cpus().iter().fold(0., |freq, cpu| freq + cpu.cpu_usage()) / system.cpus().len() as f32
}

fn get_cpu_frequency() -> Option<Frequency> {
    if let Ok(data) = table_load_from_device() {
        if let Some(speed_maybe) = data.find_map(|sys_info: SMBiosProcessorInformation| sys_info.max_speed()) {
            if let ProcessorSpeed::MHz(value) = speed_maybe {
                return Some(Frequency::from_mhz(value));
            }
        }
    }

    None
}

fn get_motherboard_name() -> Option<String> {
    if let Ok(data) = table_load_from_device() {
        return data.find_map(|sys_info: SMBiosBaseboardInformation| sys_info.product().ok());
    }

    None
}

fn get_motherboard_manufacturer() -> Option<String> {
    if let Ok(data) = table_load_from_device() {
        return data.find_map(|sys_info: SMBiosBaseboardInformation| sys_info.manufacturer().ok());
    }

    None
}

pub(crate) fn build_info_lines(args: &Cli, system: &System) -> Vec<String> {
    let mut info_lines: Vec<String> = vec![];

    // Header: Username OR Username@Hostname
    // Colored and uncolored version to get the correct length. Redo this shit.
    let header = if let Ok(hostname) = whoami::fallible::hostname() { format!("{}@{}", whoami::username().green(), hostname.green()) } else { whoami::username().green().to_string() };
    let header_len = strip_ansi_escapes::strip_str(&header).len();
    info_lines.push(header);
    info_lines.push("-".repeat(header_len));

    if args.show_distro || args.show_all {
        info_lines.push(get_entry_formatted("Distro", &whoami::distro()));
    }

    if args.show_device_name || args.show_all {
        info_lines.push(get_entry_formatted("Device", &whoami::devicename()));
    }

    if args.show_architecture || args.show_all {
        info_lines.push(get_entry_formatted("Arch", &whoami::arch().to_string()));
    }

    if args.show_uptime || args.show_all {
        match uptime_lib::get() {
            Ok(duration) => {
                info_lines.push(get_entry_formatted("Uptime", &format!("{}", DisplayableDuration(duration))));
            }
            Err(..) => {
                info_lines.push(get_entry_formatted("Uptime", "Unknown"));
            }
        }
    }

    if args.show_motherboard || args.show_all {
        info_lines.push(get_entry_formatted("Motherboard Name", &get_motherboard_name().unwrap_or("Unknown".to_string())));
    }

    if args.show_motherboard_manufacturer || args.show_all {
        info_lines.push(get_entry_formatted("Motherboard Manufacturer", &get_motherboard_manufacturer().unwrap_or("Unknown".to_string())));
    }

    if args.show_cpu || args.show_all {
        info_lines.push(get_entry_formatted("CPU Name", system.cpus().first().unwrap().brand()));
    }

    if args.show_cpu_vendor || args.show_all {
        info_lines.push(get_entry_formatted("CPU Vendor", system.cpus().first().unwrap().vendor_id()));
    }

    if args.show_cpu_frequency || args.show_all {
        if let Some(max_frequency) = get_cpu_frequency() {
            info_lines.push(get_entry_formatted("CPU Frequency", &max_frequency.to_string()));
        } else {
            info_lines.push(get_entry_formatted("CPU Frequency", "Unknown"));
        }
    }

    if args.show_cpu_usage || args.show_all {
        info_lines.push(get_entry_formatted("CPU Usage", &format!("{:.2}%", get_total_cpu_usage(system))));
    }


    // GPU enumeration.
    let instance = wgpu::Instance::default();

    for (index, adapter) in instance.enumerate_adapters(wgpu::Backends::all()).iter()
        .filter(|adapter| [DeviceType::DiscreteGpu, DeviceType::IntegratedGpu].contains(&adapter.get_info().device_type))
        .sorted_by(|a, b| Ord::cmp(&a.get_info().device, &b.get_info().device))
        .dedup_by(|a, b| a.get_info().device == b.get_info().device)
        .enumerate() {
        if args.show_gpu || args.show_all {
            info_lines.push(get_entry_formatted(&format!("GPU {}", index + 1), &adapter.get_info().name));
        }
    }


    if args.show_memory || args.show_all {
        let format_size_opts = FormatSizeOptions::default().decimal_places(2);

        info_lines.push(get_entry_formatted(
            "Memory Usage",
            &format!("{} used of {} total", format_size(system.used_memory(), format_size_opts), format_size(system.total_memory(), format_size_opts)))
        );
    }

    if args.show_swap || args.show_all {
        let format_size_opts = FormatSizeOptions::default().decimal_places(2);

        info_lines.push(get_entry_formatted(
            "Swap Usage",
            &format!("{} used of {} total", format_size(system.used_swap(), format_size_opts), format_size(system.total_swap(), format_size_opts)))
        );
    }

    if args.show_ip || args.show_all {
        if let Ok(ip) = local_ip() {
            info_lines.push(get_entry_formatted("Local Address", &ip.to_string()));
        } else {
            info_lines.push(get_entry_formatted("Local Address", "Unknown"));
        }
    }

    // Color display.
    info_lines.push("".to_string());
    info_lines.push(format!(
        "{}{}{}{}{}{}{}{}",
        "███".bright_red(),
        "███".bright_green(),
        "███".bright_blue(),
        "███".bright_cyan(),
        "███".bright_yellow(),
        "███".bright_magenta(),
        "███".bright_black(),
        "███".bright_white()
    ));
    info_lines.push(format!(
        "{}{}{}{}{}{}{}{}",
        "███".red(),
        "███".green(),
        "███".blue(),
        "███".cyan(),
        "███".yellow(),
        "███".magenta(),
        "███".black(),
        "███".white()
    ));

    info_lines
}

