fn main() {
    if let Err(e) = wcr::run(wcr::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
