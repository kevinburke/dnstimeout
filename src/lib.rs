use std::io;
use std::io::net::addrinfo;
use std::io::net::ip;
use std::io::timer;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::thread;
use std::time;

pub fn lookup(host: String, timeout_duration: time::Duration) -> io::IoResult<Vec<ip::IpAddr>> {
    let (tx, rx):(mpsc::Sender<io::IoResult<Vec<ip::IpAddr>>>, mpsc::Receiver<io::IoResult<Vec<ip::IpAddr>>>) = channel();
    let mut timer = timer::Timer::new().unwrap();
    let timeout = timer.oneshot(timeout_duration);

    let detail = format!("Failed to resolve {} after {} milliseconds", 
                         host, timeout_duration.num_milliseconds());
    thread::Thread::spawn(move|| {
        // Reading the recverror docs, I'm not sure how this can fail.
        tx.send(addrinfo::get_host_addresses(host.as_slice()));
    });

    loop {
        select! {
            val = rx.recv() => { return val.unwrap() },
            _ = timeout.recv() => {
                let e = io::IoError{
                    kind: io::IoErrorKind::TimedOut,
                    desc: "DNS lookup timed out",
                    detail: Some(detail)
                };
                return Err(e)
            }
        }
    }
}

#[test]
fn test_lookup() {
    let res = lookup("api.twilio.com".to_string(), time::Duration::seconds(7));
    match res {
        Ok(addrs) => {
            if addrs.len() == 0 {
                panic!("expected to get addresses back, got an empty list.")
            }
            for addr in addrs.iter() {
                println!("{}", addr);
            }
        }
        Err(e) => { println!("{}", e); panic!(e) }
    }
}
