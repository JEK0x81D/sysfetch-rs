use std::process::Command;

#[cfg(target_os = "windows")]
fn get_package_counts_internal() -> Vec<(String, usize)> {
    let mut package_counts = Vec::new();

    let winget_output_maybe = Command::new("winget").args(["list", "--disable-interactivity", "-s", "winget"]).output();

    if let Ok(winget_output) = winget_output_maybe {
        if winget_output.status.success() {
            let line_count = winget_output.stdout.iter().filter(|c| **c as char == '\n').count();
            package_counts.push(("winget".to_string(), line_count.max(2) - 2));
        }
    }

    // TODO: Other package manager.

    package_counts
}


#[cfg(target_os = "linux")]
fn get_package_counts_internal() -> Vec<(String, usize)> {
    todo!()
}

#[cfg(target_os = "macos")]
fn get_package_counts_internal() -> Vec<(String, usize)> {
    todo!()
}

pub(crate) fn get_package_counts() -> Vec<(String, usize)> {
    get_package_counts_internal()
}