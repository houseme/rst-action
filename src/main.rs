use clap::Parser;
use ipnet::Ipv4Net;
use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;
use sysinfo::{Disks, Networks, System};
use whoami;
use whoami::fallible;

#[derive(Parser, Debug)]
#[clap(
    name = "SystemPulse",
    version = "1.0",
    about = "Unleash Your Machine's Vital Signs"
)]
struct Args {
    /// 刷新间隔（秒）
    #[clap(short, long, default_value_t = 2)]
    interval: u64,
}

fn main() {
    let args = Args::parse();
    let mut sys = System::new_all();

    println!("=== Welcome to SystemPulse ===");
    println!("Running as: {}", whoami::username());
    println!("Hostname: {:?}", fallible::hostname());
    println!("=============================\n");
    loop {
        sys.refresh_all();
        display_system_info(&mut sys);
        display_network_speed(&mut sys, args.interval);
        thread::sleep(Duration::from_secs(args.interval));
        println!("--- Refreshing ---\n");
    }
}

fn display_system_info(sys: &mut System) {
    // CPU 信息
    println!("CPU Info:");
    let cpu_usage = sys.global_cpu_usage();
    println!("  Total CPU Usage: {:.2}%", cpu_usage);
    println!("  CPU Cores: {}", sys.cpus().len());
    sys.refresh_cpu_usage(); // Refreshing CPU usage.
    for cpu in sys.cpus() {
        print!("{}% ", cpu.cpu_usage());
    }

    // 内存信息
    println!("\nMemory Info:");
    let total_mem = sys.total_memory() / 1024 / 1024; // MB
    let used_mem = sys.used_memory() / 1024 / 1024; // MB
    println!("  Total: {} MB", total_mem);
    println!(
        "  Used: {} MB ({:.2}%)",
        used_mem,
        (used_mem as f64 / total_mem as f64) * 100.0
    );

    // 磁盘信息
    println!("\nDisk Info:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!(
            "  {}: {}/{} GB free",
            disk.mount_point().display(),
            disk.available_space() / 1024 / 1024 / 1024,
            disk.total_space() / 1024 / 1024 / 1024
        );
    }
}

fn display_network_speed(sys: &mut System, interval: u64) {
    println!("\nNetwork Info:");

    // 获取初始网络数据
    sys.refresh_memory();
    let networks = Networks::new_with_refreshed_list();
    let mut initial_data = 0;
    for (_, data) in &networks {
        initial_data += data.received() + data.transmitted();
    }

    // 等待一段时间后再测量
    thread::sleep(Duration::from_secs(interval));
    sys.refresh_memory();
    let mut final_data = 0;
    let networks = Networks::new_with_refreshed_list();
    for (_, data) in &networks {
        final_data += data.received() + data.transmitted();
    }

    // 计算速度 (KB/s)
    let speed = (final_data - initial_data) as f64 / 1024.0 / interval as f64;
    println!("  Network Speed: {:.2} KB/s", speed);
    let networks = Networks::new_with_refreshed_list();
    // 显示网络接口
    for (interface, data) in &networks {
        println!(
            "  {}: {}/{} KB",
            interface,
            data.received() / 1024,
            data.transmitted() / 1024
        );
    }

    // 使用 ipnet 解析示例子网（假设接口 IP 为 192.168.1.100/24）
    let example_ip: Ipv4Addr = "192.168.1.100".parse().unwrap();
    let subnet: Ipv4Net = "192.168.1.0/24".parse().unwrap();
    println!("  示例接口 IP: {}", example_ip);
    println!("  子网范围：{}", subnet);
    println!("  主机数量：{}", subnet.hosts().count());

    let networks = Networks::new_with_refreshed_list();
    // 显示网络接口流量
    for (interface, data) in &networks {
        println!(
            "  {}: {}/{} KB",
            interface,
            data.received() / 1024,
            data.transmitted() / 1024
        );
    }
}
