use std::fs;
use std::thread;
use std::time;

static PATH_STAT : &str = "/proc/stat";
static PATH_CPU_INFO : &str = "/proc/cpuinfo";
static PATH_MEM_INFO : &str = "/proc/meminfo";
static PATH_UPTIME : &str = "/proc/uptime";
static PATH_KERNEL : &str = "/proc/version";
static PATH_DISTRO : &str = "/etc/os-release";

static READ_ERROR : &str = "Error while reading file";
static PARSE_ERROR : &str = "Error while parsing string";

struct FilesContent {
    stat: String,
    cpu_info: String,
    mem_info: String,
    uptime: String,
    kernel: String,
    distro: String
}

pub struct CoreTime {
    pub time: [u64; 10]
}

pub struct CpuTime {
    pub times: Vec<CoreTime>
}

pub struct CoreLoad {
    pub idle: f64,
}

pub struct CpuLoad {
    pub loads: Vec<CoreLoad>
}

pub struct CpuFreq {
    pub freqs: Vec<u64>
}

pub struct CpuInfo {
    pub load: CpuLoad,
    pub freq: CpuFreq,
    pub core_count: u16
}

pub struct MemInfo {
    pub total: u64,
    pub taken: u64
}

pub struct MiscInfo {
    pub uptime: u64,
    pub kernel: String,
    pub distro: String
}

pub struct SysInfo {
    pub cpu: CpuInfo,
    pub mem: MemInfo,
    pub misc: MiscInfo
}

pub fn get_sys_info(wait_time : u64) -> SysInfo{
    let cpu_time1 : CpuTime = get_cpu_times(get_stat());
    thread::sleep(time::Duration::from_millis(wait_time));
    let files = get_files_content();

    return SysInfo {
        cpu : CpuInfo {
            load: get_cpu_loads(cpu_time1, get_cpu_times(files.stat)),
            freq: get_cpu_freqs(&files.cpu_info),
            core_count: get_core_count(&files.cpu_info)
        },
        mem: get_mem_info(files.mem_info),
        misc: get_misc_info(files.uptime, files.kernel, files.distro)
    }
}

fn get_core_load(time1 : &CoreTime, time2 : &CoreTime) -> CoreLoad {
    let idle1 = time1.time[1] + time1.time[4];
    let idle2 = time2.time[1] + time2.time[4];

    let busy1 = time1.time[0] + time1.time[1] + time1.time[2] + time1.time[5] + time1.time[6] + time1.time[7] + time1.time[8] + time1.time[9];
    let busy2 = time2.time[0] + time2.time[1] + time2.time[2] + time2.time[5] + time2.time[6] + time2.time[7] + time2.time[8] + time2.time[9];

    let delta_idle = idle2 - idle1;
    let delta_total = busy2 - busy1 + delta_idle;

    return CoreLoad {
        idle: (delta_total as f64 - delta_idle as f64) / delta_total as f64,
    };
}

fn get_cpu_loads(time1 : CpuTime, time2 : CpuTime) -> CpuLoad {
    let mut loads : Vec<CoreLoad> = Vec::new();

    for i in 0..time1.times.len() {
        loads.push(get_core_load(&time1.times[i], &time2.times[i]));
    }
    
    return CpuLoad {
        loads: loads
    };
}

fn get_cpu_times(stat : String) -> CpuTime {
    let mut times : Vec<CoreTime> = Vec::new();

    for line in stat.lines().skip(1) {
        if line.starts_with("cpu"){
            times.push(CoreTime {
                time: time_string_to_array(line.to_string())
            });
        }
    }

    return CpuTime {
        times: times
    };
}

fn time_string_to_array(time_string : String) -> [u64; 10] {
    let mut times : [u64; 10] = [0; 10];

    for (i, time) in time_string.split_whitespace().skip(1).enumerate() {
        times[i] = time.parse().expect(PARSE_ERROR);
    }

    return times;
}

fn get_cpu_freqs(cpu_info : &String) -> CpuFreq {
    let mut freqs : Vec<u64> = Vec::new();

    for line in cpu_info.lines() {
        if line.starts_with("cpu MHz"){
            freqs.push(line.split_whitespace().nth(3).expect(READ_ERROR).parse::<f64>().expect(PARSE_ERROR).floor() as u64);
        }
    }

    return CpuFreq {
        freqs: freqs
    };
}

fn get_misc_info(uptime : String, kernel : String, distro : String) -> MiscInfo {
    return MiscInfo {
        uptime: get_uptime(uptime),
        kernel: get_kernel_version(kernel),
        distro: get_distro_name(distro)
    };
}

fn get_mem_info(mem_info : String) -> MemInfo {
    let splitted = mem_info.lines();

    let total : u64 = splitted.clone().nth(0).expect(READ_ERROR).split_whitespace().nth(1).expect(READ_ERROR).parse().expect(PARSE_ERROR);
    let available : u64 = splitted.clone().nth(2).expect(READ_ERROR).split_whitespace().nth(1).expect(READ_ERROR).parse().expect(PARSE_ERROR);

    return MemInfo {
        total: total,
        taken: total - available,
    };
}

fn get_kernel_version(kernel : String) -> String {
    return kernel.split_whitespace().nth(2).expect(READ_ERROR).to_string();
}

fn get_uptime(uptime : String) -> u64 {
    return uptime.split_whitespace().nth(0).expect(READ_ERROR).parse::<f64>().expect(PARSE_ERROR).floor() as u64;
}

fn get_distro_name(distro : String) -> String {
    for line in distro.lines(){
        if line.starts_with("PRETTY_NAME"){
            return line.split("\"").nth(1).expect(READ_ERROR).to_string();
        }
    }

    panic!("{}", READ_ERROR);
}

fn get_core_count(cpu_info : &String) -> u16 {
    let mut count = 0;

    for line in cpu_info.lines(){
        if line.starts_with("processor"){
            count += 1;
        }
    }

    return count;
}

fn get_files_content() -> FilesContent {
    let stat : String = fs::read_to_string(PATH_STAT).expect(READ_ERROR);
    let cpu_info : String = fs::read_to_string(PATH_CPU_INFO).expect(READ_ERROR);
    let mem_info : String = fs::read_to_string(PATH_MEM_INFO).expect(READ_ERROR);
    let uptime : String = fs::read_to_string(PATH_UPTIME).expect(READ_ERROR);
    let kernel : String = fs::read_to_string(PATH_KERNEL).expect(READ_ERROR);
    let distro : String = fs::read_to_string(PATH_DISTRO).expect(READ_ERROR);

    return FilesContent {
        stat: stat,
        cpu_info: cpu_info,
        mem_info: mem_info,
        uptime: uptime,
        kernel: kernel,
        distro: distro
    };
}

fn get_stat() -> String {
    return fs::read_to_string(PATH_STAT).expect(READ_ERROR);
}
