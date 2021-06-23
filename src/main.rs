mod cpuinfo;
#[allow(dead_code)]
mod meminfo;
mod proc_status;
//mod meminfo_wh;

fn main() {
    println!("Hello, sumoworld!");

    let a = cpuinfo::get_cpuinfo();

    //println!("{:#?}", a[3].apicid);

    //let a = meminfo::get_meminfo();

    /*let b = proc_status::get_pid(68789);
    match b {
        Ok(t) => println!("{:#?}", t),
        Err(e) => println!("Error: {:#?}", e),
    }*/
    //assert_eq!(meminfo_wh::get_meminfo(), meminfo::get_meminfo());
}
