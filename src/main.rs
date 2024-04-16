use std::{
    io::Write,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

mod context;
mod interface;

fn main() -> std::io::Result<()> {
    let interface = interface::Interface::new();

    let mut cxt = context::Context::new(interface);
    let start = Instant::now();

    cxt.writer.write_all(
        format!(
            "{}\n\n",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )
        .as_bytes(),
    )?;

    while cxt.process_next_block().is_some() {}
    let elapsed = start.elapsed();
    println!(
        "Got rid of {} illegals in {:.3} seconds",
        cxt.cnt,
        elapsed.as_secs_f64()
    );
    Ok(())
}
