use std::net::IpAddr;
use std::str::FromStr;
use trippy_core::Builder;
use trippy_core::SocketReadinessMode;

fn main() {
    let _fds = make_a_bunch_of_fds();

    // copied nearly verbatim from docs.rs example
    let addr = IpAddr::from_str("1.1.1.1").unwrap();
    Builder::new(addr)
        .socket_readiness_mode(SocketReadinessMode::Poll)
        .build()
        .unwrap()
        .run_with(|round| println!("{:?}", round))
        .unwrap();
}

fn make_a_bunch_of_fds() -> Vec<std::fs::File> {
    (0..1024)
        .map(|_| std::fs::File::open("/dev/null").unwrap())
        .collect()
}
