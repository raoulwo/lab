fn main() {
    if let Err(e) = headr::run(headr::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
