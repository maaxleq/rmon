mod monitor;
mod display;

use crate::monitor as m;
use crate::display as d;

fn main() {
    //let sys_info = m::get_sys_info(1000);
    d::init();
    //loop {}
}
