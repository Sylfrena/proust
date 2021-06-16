use std::fs;

#[derive(Debug)]
pub struct Meminfo {
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_cached: u64,
    pub active: u64,
    pub inactive: u64,
    pub active_anon: u64,
    pub inactive_anon: u64,
    pub active_file: u64,
    pub inactive_file: u64,
    pub unevictable: u64,
    pub mlocked: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub dirty: u64,
    pub writeback: u64,
    pub anon_pages: u64,
    pub mapped: u64,
    pub shmem: u64,
    pub k_reclaimable: u64,
    pub slab: u64,
    pub s_reclaimable: u64,
    pub s_unreclaim: u64,
    pub kernel_stack: u64,
    pub page_tables: u64,
    pub nfs_unstable: u64,
    pub bounce: u64,
    pub writeback_tmp: u64,
    pub commit_limit: u64,
    pub committed_as: u64,
    pub vmalloc_total: u64,
    pub vmalloc_used: u64,
    pub vmalloc_chunk: u64,
    pub percpu: u64,
    pub hardware_corrupted: u64,
    pub anon_huge_pages: u64,
    pub shmem_huge_pages: u64,
    pub shmem_pmd_mapped: u64,
    pub file_huge_pages: u64,
    pub file_pmd_mapped: u64,
    pub cma_total: u64,
    pub cma_free: u64,
    pub huge_pages_total: u64,
    pub huge_pages_free: u64,
    pub huge_pages_rsvd: u64,
    pub huge_pages_surp: u64,
    pub hugepagesize: u64,
    pub hugetlb: u64,
    pub direct_map4k: u64,
    pub direct_map2_m: u64,
    pub direct_map1_g: u64,
}

impl Meminfo {
    fn new() -> Self {
        Meminfo {
            mem_total: 0,
            mem_free: 0,
            mem_available: 0,
            buffers: 0,
            cached: 0,
            swap_cached: 0,
            active: 0,
            inactive: 0,
            active_anon: 0,
            inactive_anon: 0,
            active_file: 0,
            inactive_file: 0,
            unevictable: 0,
            mlocked: 0,
            swap_total: 0,
            swap_free: 0,
            dirty: 0,
            writeback: 0,
            anon_pages: 0,
            mapped: 0,
            shmem: 0,
            k_reclaimable: 0,
            slab: 0,
            s_reclaimable: 0,
            s_unreclaim: 0,
            kernel_stack: 0,
            page_tables: 0,
            nfs_unstable: 0,
            bounce: 0,
            writeback_tmp: 0,
            commit_limit: 0,
            committed_as: 0,
            vmalloc_total: 0,
            vmalloc_used: 0,
            vmalloc_chunk: 0,
            percpu: 0,
            hardware_corrupted: 0,
            anon_huge_pages: 0,
            shmem_huge_pages: 0,
            shmem_pmd_mapped: 0,
            file_huge_pages: 0,
            file_pmd_mapped: 0,
            cma_total: 0,
            cma_free: 0,
            huge_pages_total: 0,
            huge_pages_free: 0,
            huge_pages_rsvd: 0,
            huge_pages_surp: 0,
            hugepagesize: 0,
            hugetlb: 0,
            direct_map4k: 0,
            direct_map2_m: 0,
            direct_map1_g: 0,
        } 
    }

    fn get_meminfo_values(mut self, file_content: &str) -> Meminfo {
        
        let content: Vec<&str> = file_content.split_terminator('\n').collect();
        for x in content.iter() {
            let arr: Vec<&str> = x.split(':').collect();
            let value = arr[1].trim_end_matches("kB").trim().parse::<u64>().unwrap();
    
            match arr[0].trim() {
                "MemTotal" => self.mem_total = value,
                "MemFree" => self.mem_free = value,
                "MemAvailable" => self.mem_available = value,
                "Buffers" => self.buffers = value,
                "Cached" => self.cached = value,
                "SwapCached" => self.swap_cached = value,
                "Active" => self.active = value,
                "Inactive" => self.inactive = value,
                "Active(anon)" => self.active_anon = value,
                "Inactive(anon)" => self.inactive_anon = value,
                "Active(file)" => self.active_file = value,
                "Inactive(file)" => self.inactive_file = value,
                "Unevictable" => self.unevictable = value,
                "Mlocked" => self.mlocked = value,
                "SwapTotal" => self.swap_total = value,
                "SwapFree" => self.swap_free = value,
                "Dirty" => self.dirty = value,
                "Writeback" => self.writeback = value,
                "AnonPages" => self.anon_pages = value,
                "Mapped" => self.mapped = value,
                "Shmem" => self.shmem = value,
                "KReclaimable" => self.k_reclaimable = value,
                "Slab" => self.slab = value,
                "SReclaimable" => self.s_reclaimable = value,
                "SUnreclaim" => self.s_unreclaim = value,
                "KernelStack" => self.kernel_stack = value,
                "PageTables" => self.page_tables = value,
                "NFS_Unstable" => self.nfs_unstable = value,
                "Bounce" => self.bounce = value,
                "WritebackTmp" => self.writeback_tmp = value,
                "CommitLimit" => self.commit_limit = value,
                "Committed_AS" => self.committed_as = value,
                "VmallocTotal" => self.vmalloc_total = value,
                "VmallocUsed" => self.vmalloc_used = value,
                "VmallocChunk" => self.vmalloc_chunk = value,
                "Percpu" => self.percpu = value,
                "HardwareCorrupted" => self.hardware_corrupted = value,
                "AnonHugePages" => self.anon_huge_pages = value,
                "ShmemHugePages" => self.shmem_huge_pages = value,
                "ShmemPmdMapped" => self.shmem_pmd_mapped = value,
                "FileHugePages" => self.file_huge_pages = value,
                "FilePmdMapped" => self.file_pmd_mapped = value,
                "CmaTotal" => self.cma_total = value,
                "CmaFree" => self.cma_free = value,
                "HugePages_Total" => self.huge_pages_total = value,
                "HugePages_Free" => self.huge_pages_free = value,
                "HugePages_Rsvd" => self.huge_pages_rsvd = value,
                "HugePages_Surp" => self.huge_pages_surp = value,
                "Hugepagesize" => self.hugepagesize = value,
                "Hugetlb" => self.hugetlb = value,
                "DirectMap4k" => self.direct_map4k = value,
                "DirectMap2M" => self.direct_map2_m = value,
                "DirectMap1G" => self.direct_map1_g = value,
                _ => println!("{:?} field is not available in meminfo", arr[0]),
            }
        }
        self
    }
}

pub fn get_meminfo() -> Meminfo {
    let file_content = fs::read_to_string("/proc/meminfo").expect("messed up reading the file");
    Meminfo::get_meminfo_values(Meminfo::new(), &file_content)
}


#[cfg(test)]
mod tests {
    use super::*; //without this all the above stuff goes outta scope in tests
    #[test]
    fn meminfo_values_test() {
        let file_content = String::from("Hugepagesize: 478
                                         Hugetlb: 47800
                                         DirectMap4k: 000");
        let meminfo = Meminfo::get_meminfo_values(Meminfo::new(), &file_content);
        assert_eq!(meminfo.hugetlb, 47800);
        assert_eq!(meminfo.hugepagesize, 478);
        assert_eq!(meminfo.direct_map4k, 0);
    }
}