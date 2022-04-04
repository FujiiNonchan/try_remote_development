struct Pom{
    p:u8,
    q:Qom
}

enum Qom{
    Qom1,
    Qom2
}

fn main() {
    println!("Hello, world!");
    println!("{}",pom());
    let pom = Pom{
        p:3,
        q:Qom::Qom1
    };

}

fn pom() -> &'static str {
    "pom"
}
