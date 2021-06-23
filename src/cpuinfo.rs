use std::fs;

/*
 * TODO : change type of elements, all string now
 */

#[derive(Debug)]
pub struct ProcessorInfo {
    pub processor : String,
    pub vendor_id : String,
    pub cpu_family : String,
    pub model : String,
    pub model_name : String,
    pub stepping : String,
    pub microcode : String,
    pub cpu_m_hz : String,
    pub cache_size : String,
    pub physical_id : String,
    pub siblings : String,
    pub core_id : String,
    pub cpu_cores : String,
    pub apicid : String,
    pub initial_apicid : String,
    pub fpu : String,
    pub fpu_exception : String,
    pub cpuid_level : String,
    pub wp : String,
    pub flags : String,
    pub vmx_flags : String,
    pub bugs : String,
    pub bogomips : String,
    pub clflush_size : String,
    pub cache_alignment : String,
    pub address_sizes : String,
    pub power_management : String,
}

impl ProcessorInfo {
    fn new() -> Self {
        ProcessorInfo {
            processor : String::from(""),
            vendor_id : String::from(""),
            cpu_family : String::from(""),
            model : String::from(""),
            model_name : String::from(""),
            stepping : String::from(""),
            microcode : String::from(""),
            cpu_m_hz : String::from(""),
            cache_size : String::from(""),
            physical_id : String::from(""),
            siblings : String::from(""),
            core_id : String::from(""),
            cpu_cores : String::from(""),
            apicid : String::from(""),
            initial_apicid : String::from(""),
            fpu : String::from(""),
            fpu_exception : String::from(""),
            cpuid_level : String::from(""),
            wp : String::from(""),
            flags : String::from(""),
            vmx_flags : String::from(""),
            bugs : String::from(""),
            bogomips : String::from(""),
            clflush_size : String::from(""),
            cache_alignment : String::from(""),
            address_sizes : String::from(""),
            power_management : String::from(""),
        }
    }

    fn get_processor_info(mut self, processor_content: &Vec<&str>) -> Self {
        for x in processor_content.iter() {
            let arr: Vec<&str> = x.split(':').collect();
            let value = format!("{}", arr[1].trim());
            match arr[0].trim() {
                "processor"	 => self.processor = value,
                "vendor_id"	 => self.vendor_id = value,
                "cpu family"	 => self.cpu_family = value,
                "model"		 => self.model = value,
                "model name"	 => self.model_name = value,
                "stepping"	 => self.stepping = value,
                "microcode"	 => self.microcode = value,
                "cpu MHz"	 => self.cpu_m_hz = value,
                "cache size" => self.cache_size = value,
                "physical id"	 => self.physical_id = value,
                "siblings"	 => self.siblings = value,
                "core id"		 => self.core_id = value,
                "cpu cores"	 => self.cpu_cores = value,
                "apicid"		 => self.apicid = value,
                "initial apicid" => self.initial_apicid = value,
                "fpu"	 => self.fpu = value,
                "fpu_exception"	 => self.fpu_exception = value,
                "cpuid level" => self.cpuid_level = value,
                "wp"		 => self.wp = value,
                "flags"		 => self.flags = value,
                "vmx flags"	 => self.vmx_flags = value,
                "bugs"		 => self.bugs = value,
                "bogomips"	 => self.bogomips = value,
                "clflush size"	 => self.clflush_size = value,
                "cache_alignment"	 => self.cache_alignment = value,
                "address sizes" => self.address_sizes = value,
                "power management" => self.power_management = value,
                _ => println!("no such value"),
            }
        }
    self
    }
}



pub fn for_each_processor(file_content:&str) -> Vec<ProcessorInfo> {
    let content_cpu: Vec<&str> = file_content.split_terminator("\n\n").collect(); //splits each processor
    let mut content: Vec<Vec<&str>> = vec![vec!["blah"]];
    for x in content_cpu.iter() {
        content.push(x.split_terminator("\n").collect()); //goes through each processor and splits into member components
    }//pushes a vector 
    content.remove(0);
    let mut result: Vec<ProcessorInfo> = vec![ProcessorInfo::new()];
    result.remove(0);
    for x in content.iter() { //for each processor(vector)
        let a = ProcessorInfo::new();
        result.push(ProcessorInfo::get_processor_info(a,x));

        //println!("{:#?}", ProcessorInfo::get_processor_info(a,x));
    }
    result
    //println!("{:#?}", content);
}


pub fn get_cpuinfo() -> Vec<ProcessorInfo> {
    let file_content = fs::read_to_string("/proc/cpuinfo")
    .expect("oops messed up reading file");

    for_each_processor(&file_content)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn cpuinfo_values_test() {
        let file_content = String::from("siblings	: 12
                                         core id		: 5
                                         cpu cores	: 6
                                         apicid		: 11
                                         initial apicid	: 11
                                         fpu		: yes
                                         fpu_exception	: yes
                                         cpuid level	: 22
                                         wp		: yes
                                         flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid mpx rdseed adx smap clflushopt intel_pt xsaveopt xsavec xgetbv1 xsaves dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp md_clear flush_l1d"
                                        );
        let a = for_each_processor(&file_content);
        assert_eq!(a[0].apicid, "11");
        assert_eq!(a[0].fpu, "yes");
        assert_eq!(a[0].core_id, "5");
    
        }
}