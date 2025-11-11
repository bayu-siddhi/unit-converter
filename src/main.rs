use unitconv::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}
