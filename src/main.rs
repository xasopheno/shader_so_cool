use kintaro::{application::run, Config};
use weresocool::error::Error;

fn main() -> Result<(), Error> {
    let filename = "kintaro.socool";
    let config = Config::default();
    run(filename, config)
}
