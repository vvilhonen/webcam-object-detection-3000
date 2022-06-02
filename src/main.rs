use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

use crate::classify::Classify;

mod annotate;
mod classify;
mod ffmpeg;
mod preprocess;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    tracing::info!("Starting");

    check_device()?;

    run()?;

    Ok(())
}

fn check_device() -> anyhow::Result<()> {
    let md = std::fs::metadata("/dev/video10");
    match md {
        Ok(md) => {
            tracing::debug!("{md:#?}");
        }
        Err(e) => {
            tracing::info!("No video device: {e}, creating /dev/video10");
            let res = Command::new("sudo")
                .args(["modprobe", "v4l2loopback", "video_nr=10"])
                .spawn()?
                .wait()?;
            tracing::info!("Result {:?}", res.code());
            assert!(res.success());
        }
    };
    Ok(())
}

fn run() -> anyhow::Result<()> {
    let classifier = Classify::new()?;
    let (tx, is_black_gold) = classifier.start();

    let mut pipe_in = ffmpeg::get_pipe_in()?;
    let mut pipe_out = ffmpeg::get_pipe_out()?;

    loop {
        let mut buf = [0u8; 1920 * 1080 * 3];
        pipe_in.read_exact(&mut buf)?;
        let buf = process(&buf, &tx, &is_black_gold)?;
        pipe_out.write_all(&buf)?;
    }
}

fn process(
    buf: &[u8],
    tx: &SyncSender<Vec<u8>>,
    is_black_gold: &Arc<AtomicBool>,
) -> anyhow::Result<Vec<u8>> {
    let mut buf = buf.to_owned();
    let _ = tx.try_send(buf.clone());
    if is_black_gold.load(Ordering::Relaxed) {
        buf = annotate::annotate(buf)?;
    }
    Ok(buf)
}
