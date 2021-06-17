#[allow(dead_code)]
mod meminfo;

mod proc_status;
//mod meminfo_wh;

fn main() {
    println!("Hello, sumoworld!");
    //let a = meminfo::get_meminfo();

    let b = proc_status::get_pid(90);
    match b {
        Ok(t) => println!("{:#?}", t),
        Err(e) => println!("Error: {:#?}", e),
    }
    //assert_eq!(meminfo_wh::get_meminfo(), meminfo::get_meminfo());
}
