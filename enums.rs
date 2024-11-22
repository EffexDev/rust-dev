#[derive(Debug)]
struct IPv4Addr {
    address:(u8,u8,u8,u8),
    location: String
}

#[derive(Debug)]
enum IPAddr {
    V4(IPv4Addr),
}

fn main() {
    let home = IPAddr::V4 (IPv4Addr{
    address: (192,1,1,1),
    location: String::from("45 Corinda Way")
    });
    
match &home {
    IPAddr::V4(addr) => println!("{}.{}.{}.{} can be found at {}",
    addr.address.0,
    addr.address.1,
    addr.address.2,
    addr.address.3,
    addr.location
    )
}

}
