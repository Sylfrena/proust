use std::fs;
use std::collections::HashMap;




#[derive(Debug)]
pub struct Meminfo {
    pub MemTotal: u64,
    pub MemFree: u64,
    pub MemAvailable: u64,
    pub Buffers: u64,
    pub Cached: u64,
    pub SwapCached: u64,
    pub Active: u64,
    pub Inactive: u64,
    pub ActiveAnon: u64,
    pub InactiveAnon: u64,
    pub ActiveFile: u64,
    pub InactiveFile: u64,
    pub Unevictable: u64,
    pub Mlocked: u64,
    pub SwapTotal: u64,
    pub SwapFree: u64,
    pub Dirty: u64,
    pub Writeback: u64,
    pub AnonPages: u64,
    pub Mapped: u64,
    pub Shmem: u64,
    pub KReclaimable: u64,
    pub Slab: u64,
    pub SReclaimable: u64,
    pub SUnreclaim: u64,
    pub KernelStack: u64,
    pub PageTables: u64,
    pub NfsUnstable: u64,
    pub Bounce: u64,
    pub WritebackTmp: u64,
    pub CommitLimit: u64,
    pub CommittedAs: u64,
    pub VmallocTotal: u64,
    pub VmallocUsed: u64,
    pub VmallocChunk: u64,
    pub Percpu: u64,
    pub HardwareCorrupted: u64,
    pub AnonHugePages: u64,
    pub ShmemHugePages: u64,
    pub ShmemPmdMapped: u64,
    pub FileHugePages: u64,
    pub FilePmdMapped: u64,
    pub CmaTotal: u64,
    pub CmaFree: u64,
    pub HugePagesTotal: u64,
    pub HugePagesFree: u64,
    pub HugePagesRsvd: u64,
    pub HugePagesSurp: u64,
    pub Hugepagesize: u64,
    pub Hugetlb: u64,
    pub DirectMap4k: u64,
    pub DirectMap2M: u64,
    pub DirectMap1G: u64,
}

pub fn get_value(arg: &str) -> u64 {
    let content = fs::read_to_string("/proc/meminfo").expect("messed up reading the file");
    
    let a: Vec<&str> = content.split_terminator('\n').collect();    

    let mut map = HashMap::new();

    for v in a.iter() {
        let d: Vec<&str> = v.split(':').collect();
        map.insert(d[0], d[1].trim_end_matches("kB").trim().parse::<u64>().unwrap());
    }

    let value = map.get(arg);
    match value {
        Some(t) => *t,
        None => 0,
    }
}


pub fn new() -> Meminfo {
    Meminfo {
        MemTotal: get_value("MemTotal"),
        MemFree: get_value("MemFree"),
        MemAvailable: get_value("MemAvailable"),
        Buffers: get_value("Buffers"),
        Cached: get_value("Cached"),
        SwapCached: get_value("SwapCached"),
        Active: get_value("Active"),
        Inactive: get_value("Inactive"),
        ActiveAnon: get_value("ActiveAnon"),
        InactiveAnon: get_value("InactiveAnon"),
        ActiveFile: get_value("ActiveFile"),
        InactiveFile: get_value("InactiveFile"),
        Unevictable: get_value("Unevictable"),
        Mlocked: get_value("Mlocked"),
        SwapTotal: get_value("SwapTotal"),
        SwapFree: get_value("SwapFree"),
        Dirty: get_value("Dirty"),
        Writeback: get_value("Writeback"),
        AnonPages: get_value("AnonPages"),
        Mapped: get_value("Mapped"),
        Shmem: get_value("Shmem"),
        KReclaimable: get_value("KReclaimable"),
        Slab: get_value("Slab"),
        SReclaimable: get_value("SReclaimable"),
        SUnreclaim: get_value("SUnreclaim"),
        KernelStack: get_value("KernelStack"),
        PageTables: get_value("PageTables"),
        NfsUnstable: get_value("NfsUnstable"),
        Bounce: get_value("Bounce"),
        WritebackTmp: get_value("WritebackTmp"),
        CommitLimit: get_value("CommitLimit"),
        CommittedAs: get_value("CommittedAs"),
        VmallocTotal: get_value("VmallocTotal"),
        VmallocUsed: get_value("VmallocUsed"),
        VmallocChunk: get_value("VmallocChunk"),
        Percpu: get_value("Percpu"),
        HardwareCorrupted: get_value("HardwareCorrupted"),
        AnonHugePages: get_value("AnonHugePages"),
        ShmemHugePages: get_value("ShmemHugePages"),
        ShmemPmdMapped: get_value("ShmemPmdMapped"),
        FileHugePages: get_value("FileHugePages"),
        FilePmdMapped: get_value("FilePmdMapped"),
        CmaTotal: get_value("CmaTotal"),
        CmaFree: get_value("CmaFree"),
        HugePagesTotal: get_value("HugePagesTotal"),
        HugePagesFree: get_value("HugePagesFree"),
        HugePagesRsvd: get_value("HugePagesRsvd"),
        HugePagesSurp: get_value("HugePagesSurp"),
        Hugepagesize: get_value("Hugepagesize"),
        Hugetlb: get_value("Hugetlb"),
        DirectMap4k: get_value("DirectMap4k"),
        DirectMap2M: get_value("DirectMap2M"),
        DirectMap1G: get_value("DirectMap1G"),
    }
}

/* Reads the meminfo file and returns a HashMap*/
pub fn get_meminfo() {
    println!("{:?}", new());
}
