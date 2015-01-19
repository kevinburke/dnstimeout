use std::io;
use std::io::net::addrinfo;
use std::io::net::ip;
use std::time;

pub fn lookup(host: &str, timeout_duration: time::Duration) -> io::IoResult<Vec<ip::IpAddr>> {
    return addrinfo::get_host_addresses(host)
}

#[test]
fn test_lookup() {
    let res = lookup("api.twilio.com", time::Duration::seconds(3));
    match res {
        Ok(addrs) => {
            for addr in addrs.iter() {
                println!("{}", addr);
            }
            panic!("foobar")
        }
        Err(e) => { panic!(e) }
    }
}
