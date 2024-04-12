use std::{io::Write, sync::mpsc::Receiver};

use anyhow::Context;
use tokio_serial::SerialPortBuilderExt;

pub fn start(rx: Receiver<String>) -> anyhow::Result<()> {
    // let mut port = tokio_serial::new("", 115200)
    //     .open_native_async()
    //     .context("could not open port!")?;

    // port.set_exclusive(false)
    //     .context("unable to set port to exclusive!")?;

    while let Ok(message) = rx.recv() {
        println!("{}", &message);
        // port.write_all(message.as_bytes())
        //     .context("couldn't write message!")?;
    }

    Ok(())
}
