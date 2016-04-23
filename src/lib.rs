//#![feature(lookup_host)]

use std::io;
use std::net;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::time;

pub fn lookup(host: String, timeout_duration: time::Duration) -> io::Result<net::LookupHost> {
    let (tx, rx):(mpsc::Sender<io::Result<net::LookupHost>>, mpsc::Receiver<io::Result<net::LookupHost>>) = channel();
    let (tx2, rx2) = mpsc::channel();
    let nanos = timeout_duration.subsec_nanos() as u64;
    let ms = (1000*1000*1000 * timeout_duration.as_secs() + nanos)/(1000 * 1000);
    let detail = format!("Failed to resolve {} after {} milliseconds", host, ms);
    thread::spawn(move|| {
        // Reading the recverror docs, I'm not sure how this can fail.
        tx.send(net::lookup_host(host.as_str())).unwrap();
    });
    thread::spawn(move|| {
        thread::sleep(timeout_duration);
        tx2.send(()).unwrap();
    });

    loop {
        select! {
            val = rx.recv() => { return val.unwrap() },
            _ = rx2.recv() => {
                let e = io::Error::new(io::ErrorKind::TimedOut, detail);
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
