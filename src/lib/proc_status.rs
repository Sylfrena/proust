use std::fs;

#[derive(Debug)]
pub struct ProcStatus {
    name: String,                        //	ksoftirqd/7
    umask: String,                       //	0000
    state: String,                       //	S (sleeping)
    tgid: String,                        //	66
    ngid: String,                        //	0
    pid: String,                         //	66
    p_pid: String,                       //	2
    tracer_pid: String,                  //	0
    uid: String,                         //	0	0	0	0
    gid: String,                         //	0	0	0	0
    fd_size: u64,                        //	64
    groups: u64,                         //
    n_stgid: String,                     //	66
    n_spid: String,                      //	66
    n_spgid: String,                     //	0
    n_ssid: String,                      //	0
    threads: u64,                        //	1
    sig_q: String,                       //	0/31192
    sig_pnd: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	0000000000000000
    shd_pnd: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	0000000000000000
    sig_blk: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	0000000000000000
    sig_ign: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	ffffffffffffffff
    sig_cgt: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	0000000000000000
    cap_inh: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	0000000000000000
    cap_prm: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	000001ffffffffff
    cap_eff: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	000001ffffffffff
    cap_bnd: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	000001ffffffffff
    cap_amb: String, //actuly can do, a:u32 = 0xffffff but how to add ox? hence//	0000000000000000
    no_new_privs: u64, //	0
    seccomp: u64,    //	0
    seccomp_filters: u64, //	0
    speculation_store_bypass: String, //	thread vulnerable
    speculation_indirect_branch: String, //	conditional enabled
    cpus_allowed: String, //	080
    cpus_allowed_list: u64, //	7
    mems_allowed: String, //	00000001
    mems_allowed_list: String, //	0
    voluntary_ctxt_switches: String, //	6612
    nonvoluntary_ctxt_switches: u64, //	2
}

impl ProcStatus {
    fn new() -> Self {
        ProcStatus {
            //maybe change to &str
            name: String::from(""),
            umask: String::from(""),
            state: String::from(""),
            tgid: String::from(""),
            ngid: String::from(""),
            pid: String::from(""),
            p_pid: String::from(""),
            tracer_pid: String::from(""),
            uid: String::from(""),
            gid: String::from(""),
            fd_size: 0,
            groups: 0,
            n_stgid: String::from(""),
            n_spid: String::from(""),
            n_spgid: String::from(""),
            n_ssid: String::from(""),
            threads: 0,
            sig_q: String::from(""),
            sig_pnd: String::from(""),
            shd_pnd: String::from(""),
            sig_blk: String::from(""),
            sig_ign: String::from(""),
            sig_cgt: String::from(""),
            cap_inh: String::from(""),
            cap_prm: String::from(""),
            cap_eff: String::from(""),
            cap_bnd: String::from(""),
            cap_amb: String::from(""),
            no_new_privs: 0,
            seccomp: 0,
            seccomp_filters: 0,
            speculation_store_bypass: String::from(""),
            speculation_indirect_branch: String::from(""),
            cpus_allowed: String::from(""),
            cpus_allowed_list: 0,
            mems_allowed: String::from(""),
            mems_allowed_list: String::from(""),
            voluntary_ctxt_switches: String::from(""),
            nonvoluntary_ctxt_switches: 0,
        }
    }

    fn pid_status(mut self, file_content: String) -> ProcStatus {
        let content: Vec<&str> = file_content.split_terminator('\n').collect();
        for x in content.iter() {
            let arr: Vec<&str> = x.split(':').collect();
            let value = format!("{}", arr[1].trim());
            let val;
            match arr[1].trim().parse::<u64>() {
                Ok(t) => val = t,
                Err(_e) => val = 0,
            };

            match arr[0].trim() {
                "Name" => self.name = value,
                "Umask" => self.umask = value,
                "State" => self.state = value,
                "Tgid" => self.tgid = value,
                "Ngid" => self.ngid = value,
                "Pid" => self.pid = value,
                "PPid" => self.p_pid = value,
                "TracerPid" => self.tracer_pid = value,
                "Uid" => self.uid = value.replace("\t", ""),
                "Gid" => self.gid = value.replace("\t", ""),
                "FDSize" => self.fd_size = val,
                "Groups" => self.groups = val,
                "NStgid" => self.n_stgid = value,
                "NSpid" => self.n_spid = value,
                "NSpgid" => self.n_spgid = value,
                "NSsid" => self.n_ssid = value,
                "Threads" => self.threads = val,
                "SigQ" => self.sig_q = value,
                "SigPnd" => self.sig_pnd = value,
                "ShdPnd" => self.shd_pnd = value,
                "SigBlk" => self.sig_blk = value,
                "SigIgn" => self.sig_ign = value,
                "SigCgt" => self.sig_cgt = value,
                "CapInh" => self.cap_inh = value,
                "CapPrm" => self.cap_prm = value,
                "CapEff" => self.cap_eff = value,
                "CapBnd" => self.cap_bnd = value,
                "CapAmb" => self.cap_amb = value,
                "NoNewPrivs" => self.no_new_privs = val,
                "Seccomp" => self.seccomp = val,
                "Seccomp_filters" => self.seccomp_filters = val,
                "Speculation_Store_Bypass" => self.speculation_store_bypass = value,
                "SpeculationIndirectBranch" => self.speculation_indirect_branch = value,
                "Cpus_allowed" => self.cpus_allowed = value,
                "Cpus_allowed_list" => self.cpus_allowed_list = val,
                "Mems_allowed" => self.mems_allowed = value,
                "Mems_allowed_list" => self.mems_allowed_list = value,
                "voluntary_ctxt_switches" => self.voluntary_ctxt_switches = value,
                "nonvoluntary_ctxt_switches" => self.nonvoluntary_ctxt_switches = val,
                _ => println!("no such field: {:?}", arr[0]),
            }
        }
        self
    }
}

pub fn get_pid(pid: u32) -> Result<ProcStatus, &'static str> {
    let file_path = format!("/proc/{}/status", pid);
    let file_content: String;
    match fs::metadata(&file_path) {
        Ok(_) => {
            file_content = fs::read_to_string(file_path).expect("Messed up reading the file");
            Ok(ProcStatus::pid_status(ProcStatus::new(), file_content))
        }
        Err(_) => Err("Invalid file path, maybe process does not exist."),
    }
}
