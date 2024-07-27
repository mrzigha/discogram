#![allow(unused)] // silence unused while developing
use config::Config;

mod config;
fn main()
{
    let config = Config::load();
}
