use anyhow::Error;
use fehler::throws;

mod clap_app;
mod request;
mod utils;

#[throws(Error)]
fn main() {
    let matches = clap_app::app().get_matches();
    let request = request::Request::new(&matches);
    request.handle()?;
}
