fn main() {
    if let Err(e) = catr::run(catr::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
