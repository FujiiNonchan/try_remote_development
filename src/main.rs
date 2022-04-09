use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::io;

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
        let q0 = q.clone();
        thread::spawn({
            move || {
            let mut q0 = q0.lock().unwrap();
            *q0 += 3;
        }}).join().unwrap();
        let q = q.lock().unwrap();
        p + *q
    }
}
#[derive(Debug)]
enum Element{
    MARU,
    BATSU,
}

#[derive(Debug)]
struct Input{
    start:Element,
    middle:Element,
    end:Element
}

impl Element{
    fn to_u8(&self)->u8{
        if let Element::MARU = self {1} else {0}
    }
}

impl Input {
    fn convert_to_u8(&self) -> [u8; 3] {
        let temp0 = self.start.to_u8();
        let temp1 = self.middle.to_u8();
        let temp2 = self.end.to_u8();
        [temp0, temp1, temp2]
    }
}

fn get_element(c:Option<char>)->Element{
    // println!("c:{:?}", c);
    match c {
        Some('0')=>Element::BATSU,
        Some('1')=>Element::MARU,
        _=>panic!("Invalid Value")
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
    println!("start");
    let mut rest_vec = Vec::new();

    loop{
        let mut mem = String::new();
        match io::stdin().read_line(&mut mem) {
            Ok(_) => {
                // 改行コードを削除しておく
                let mem = mem.trim();
                match mem {
                    "111"=>{
                        break;    
                    }
                    _ =>{
                        let mut mem_chars = mem.chars();
                        rest_vec.push(
                            Input{
                                start:get_element(mem_chars.next()),
                                middle:get_element(mem_chars.next()),
                                end:get_element(mem_chars.next()),
                            }
                        );
                        // println!("{}", mem);
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }

    let _ports = serialport::available_ports().expect("No ports found!");
    // for p in ports {
    //    println!("{}", p.port_name);
    //}

    let mut port = serialport::new("/dev/ttyUSB0", 9600)
    .timeout(Duration::from_millis(1000))
    .open().expect("Failed to open port");
    // let _ = port.write(&rest_vec.convert_to_u8()).expect("Write failed!");
    // let _ = port.write("test".as_bytes()).expect("Write failed!");
    // print!("{}", t);

    for _ in 0..100{
         // let _ = port.write("test".as_bytes()).expect("Write failed!");
         // thread::sleep(Duration::from_millis(50));
        for res in &rest_vec{
            port.write(&res.convert_to_u8()).expect("Write failed!");
            thread::sleep(Duration::from_millis(10));
        }
    }

}
