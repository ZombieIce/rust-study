use perf_monitor::cpu::{processor_numbers, ProcessStat, ThreadStat};
use perf_monitor::fd::fd_count_cur;
use perf_monitor::io::get_process_io_stats;
use perf_monitor::mem::get_process_memory_info;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: Vec<String>,
    pub timestamp: i64,
}

pub fn monitor() -> Vec<String> {
    // cpu
    let core_num = processor_numbers().unwrap();
    let mut stat_p = ProcessStat::cur().unwrap();
    let mut stat_t = ThreadStat::cur().unwrap();

    let usage_p = stat_p.cpu().unwrap() * 100f64;
    let usage_t = stat_t.cpu().unwrap() * 100f64;

    let mut monitor_message: Vec<String> = Vec::with_capacity(3);
    monitor_message.push(format!(
        "[CPU] core Number: {}, process usage: {:.2}%, current thread usage: {:.2}%",
        core_num, usage_p, usage_t
    ));

    // mem
    let mem_info = get_process_memory_info().unwrap();
    monitor_message.push(format!(
        "[Memeory] memory used: {} bytes, virtural memory used: {} bytes ",
        mem_info.resident_set_size, mem_info.virtual_memory_size,
    ));

    // fd
    let fd_num = fd_count_cur().unwrap();
    monitor_message.push(format!("[FD] fd number: {}", fd_num));

    // io
    let io_stat = get_process_io_stats().unwrap();
    monitor_message.push(format!(
        "[IO] read bytes: {}, write bytes: {}",
        io_stat.read_bytes, io_stat.write_bytes
    ));

    monitor_message
}

pub fn timestamp() -> i64 {
    // get now time milliseconds timestamp
    let now = std::time::SystemTime::now();
    let timestamp = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    timestamp.as_millis() as i64
}
