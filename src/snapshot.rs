use sysinfo::System;
use colored::Colorize;
use anyhow::Result;

pub fn show_snapshot() -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    println!("{}", "=== System Snapshot ===".bright_green().bold());
    println!("{}", "─".repeat(50));
    
    // System Info
    println!("{}: {}", "System".bright_white(), 
        System::name().unwrap_or_else(|| "Unknown".to_string()));
    println!("{}: {}", "OS Version".bright_white(), 
        System::long_os_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("{}: {}", "Host Name".bright_white(), 
        System::host_name().unwrap_or_else(|| "Unknown".to_string()));
    
    println!("{}", "─".repeat(50));
    
    // CPU Info
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    println!("{}: {}%", "CPU Usage".bright_white(), 
        cpu_usage.round() as u32);
    println!("{}: {} cores", "CPU Cores".bright_white(), 
        sys.cpus().len());
    
    println!("{}", "─".repeat(50));
    
    // Memory Info
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_percent = (used_memory as f64 / total_memory as f64 * 100.0) as u32;
    
    println!("{}: {} / {} ({}%)", 
        "Memory".bright_white(),
        format_bytes(used_memory).yellow(),
        format_bytes(total_memory),
        memory_percent
    );
    
    println!("{}", "─".repeat(50));
    
    // Process count
    println!("{}: {}", "Running Processes".bright_white(), sys.processes().len());
    
    println!("{}", "─".repeat(50));
    println!("{}: {}", "Snapshot taken at".dimmed(), 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    
    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}