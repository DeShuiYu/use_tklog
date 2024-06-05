use std::fmt::format;
use tklog::{debug, error, info, LEVEL, LOG, warn};

fn main() {
    // https://github.com/donnie4w/tklog/
    LOG.set_console(true)
        .set_level(LEVEL::Debug)
        .set_formatter("{time} {level} {file} >>> {message}\n")
        .set_cutmode_by_size("tklog.log",100*1024*1024,10,true);
    debug!(format!("debug"));
    info!(format!("info"));
    warn!(format!("warn"));
    error!(format!("error"));
}
