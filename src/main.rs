use kintaro::application::run;
use weresocool::error::Error;

fn main() -> Result<(), Error> {
    let filename = "kintaro.socool";
    run(filename)?;
    Ok(())
}
