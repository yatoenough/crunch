use std::io::{self, Write};

pub fn pick_device(
    label: &str,
    devices: Vec<cpal::Device>,
    default_device: Option<cpal::Device>,
) -> Result<cpal::Device, Box<dyn std::error::Error>> {
    if devices.is_empty() {
        return default_device.ok_or_else(|| format!("Could not find {} device", label).into());
    }

    let default_index = find_default_index(&devices, &default_device);

    print_devices(label, &devices, default_index);

    loop {
        match default_index {
            Some(i) => print!("Select {} device [default {}]: ", label, i + 1),
            None => print!("Select {} device: ", label),
        }

        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        // default selection
        if input.is_empty() {
            if let Some(i) = default_index {
                return Ok(devices.into_iter().nth(i).unwrap());
            } else {
                println!("Please enter a number from 1 to {}.", devices.len());
                continue;
            }
        }

        if let Some(i) = parse_selection(input, devices.len()) {
            return Ok(devices.into_iter().nth(i).unwrap());
        }

        println!("Please enter a number from 1 to {}.", devices.len());
    }
}
fn parse_selection(input: &str, len: usize) -> Option<usize> {
    input.trim().parse::<usize>().ok().and_then(|n| {
        if (1..=len).contains(&n) {
            Some(n - 1)
        } else {
            None
        }
    })
}

fn print_devices(label: &str, devices: &[cpal::Device], default_index: Option<usize>) {
    println!("\nAvailable {} devices:", label);

    for (i, device) in devices.iter().enumerate() {
        let marker = if Some(i) == default_index {
            " (default)"
        } else {
            ""
        };

        println!("  {}. {}{}", i + 1, device, marker);
    }
}

fn find_default_index(devices: &[cpal::Device], default: &Option<cpal::Device>) -> Option<usize> {
    default
        .as_ref()
        .and_then(|default| devices.iter().position(|d| d == default))
}
