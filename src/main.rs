fn main() {
    let path = "/proc/meminfo";
    match std::fs::read_to_string(path) {
        Ok(meminfo) => {
            let ram_total = meminfo
                .lines()
                .find(|line| line.starts_with("MemTotal:"))
                .unwrap_or_default();
            let ram_available = meminfo
                .lines()
                .find(|line| line.starts_with("MemAvailable:"))
                .unwrap_or_default();
            let swap_total = meminfo
                .lines()
                .find(|line| line.starts_with("SwapTotal:"))
                .unwrap_or_default();
            let swap_free = meminfo
                .lines()
                .find(|line| line.starts_with("SwapFree:"))
                .unwrap_or_default();

            let ram_total = ram_total
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let ram_available = ram_available
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let swap_total = swap_total
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let swap_free = swap_free
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let ram_used: String = if (ram_total - ram_available) < 1024 {
                //kB
                format!("{} kB", ram_total - ram_available)
            } else if (ram_total - ram_available) / 1024 < 1024 {
                //MB
                format!("{} MB", (ram_total - ram_available) / 1024)
            } else {
                //GB
                (((ram_total - ram_available) as f32 / 1024. / 1024. * 10.).trunc() / 10.)
                    .to_string()
            };

            let ram_total = (ram_total as f32 / 1024. / 1024.).round();

            let swap_used: String = if (swap_total - swap_free) < 1024 {
                //kB
                format!("{} kB", swap_total - swap_free)
            } else if (swap_total - swap_free) / 1024 < 1024 {
                //MB
                format!("{} MB", (swap_total - swap_free) / 1024)
            } else {
                //GB
                (((swap_total - swap_free) as f32 / 1024. / 1024. * 10.).trunc() / 10.).to_string()
            };

            let swap_total = (swap_total as f32 / 1024. / 1024.).round();

            println!(
                "Ram: {}/{}GB\tSwap: {}/{}GB",
                ram_used, ram_total, swap_used, swap_total
            );
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
        }
    };
}
