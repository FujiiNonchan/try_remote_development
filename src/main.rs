use std::thread;
use std::sync::{Arc, Mutex};

trait Pom{
    fn get_dummy(&self, p:String)->String;
    fn try_thread(&self, p:i32)->i32;
}

#[allow(dead_code)]
struct Qom{
    p:u8,
    r:Rom,
    s:String
}

#[allow(dead_code)]
enum Rom{
    R1,
    R2
}

impl Pom for Qom{
    fn get_dummy(&self, p:String)->String{
        p +"qom"
    }

    fn try_thread(&self, p:i32)->i32{
        let q = Arc::new(Mutex::new(0));
        thread::spawn({
            let q = q.clone();
            move || {
            let mut q = q.lock().unwrap();
            *q += 3;
        }}).join().unwrap();
        let q = q.lock().unwrap();
        p + *q
    }
}

fn main() {
    let qom = Qom{
        p:3,
        r:Rom::R2,
        s:"dummy".to_string()
    };
    println!("get_dummy={}", qom.get_dummy("ダミー".to_string()));
    println!("try_thread={}", qom.try_thread(5));

}
