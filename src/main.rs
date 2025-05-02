use std::process::{Command, Stdio};
use std::str;

fn main() {
    let (curr, max) = if cfg!(target_os = "macos") {
        read_battery_macos()
    } else if cfg!(target_os = "linux") {
        ("0".to_string(), "1".to_string())
    } else {
        panic!("Unsupported OS");
    };

    let c = curr.parse::<f32>().unwrap();
    let m = max.parse::<f32>().unwrap();

    let charge = c / m;
    let threshold = charge * 10.0;

    let slots = 10;
    let filled = (threshold.round() as u32).min(slots);
    let empty = slots - filled;

    let color_out = match filled {
        0..=3 => "\x1b[;31m", // red
        4..=6 => "\x1b[;33m", // yellow
        _ => "\x1b[;32m",     // green
    };
    let color_reset = "\x1b[0m";

    let out = format!(
        "{}{}{}{}",
        color_out,
        "◼".repeat(filled as usize),
        "◻".repeat(empty as usize),
        color_reset
    );

    print!("{}", out);
}

fn read_battery_macos() -> (String, String) {
    let echo_child = Command::new("ioreg")
        .arg("-rc")
        .arg("AppleSmartBattery")
        .stdout(Stdio::piped())
        .spawn()
        .expect("[Error]: Battery Widget Failed!");

    let result = echo_child.wait_with_output().unwrap().stdout;
    let print_result = str::from_utf8(&result).unwrap();

    let mut curr = "None";
    let mut max = "None";

    for line in print_result.lines() {
        let l: Vec<&str> = line.split('=').collect();

        if !l[0].contains("AppleRaw") && l[0].contains("CurrentCapacity") {
            curr = l[1].trim();
            if max != "None" {
                break;
            }
        }

        if l[0].contains("MaxCapacity") {
            max = l[1].trim();
            if curr != "None" {
                break;
            }
        }
    }

    (curr.to_string(), max.to_string())
}
