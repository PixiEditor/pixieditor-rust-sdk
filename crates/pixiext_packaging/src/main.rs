mod packaging;
use packaging::build_pixiext;

fn main() {

    if let Err(e) = build_pixiext() {
        eprintln!("Build failed: {e}");
        std::process::exit(1);
    }

}
