mod meminfo;
//mod meminfo_wh;

fn main() {
    println!("Hello, sumoworld!");
    let a = meminfo::get_meminfo();
    println!("{:#?}", a);

    //assert_eq!(meminfo_wh::get_meminfo(), meminfo::get_meminfo());
        
}
