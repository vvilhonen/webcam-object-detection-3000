use std::process::ChildStdin;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;

pub fn get_pipe_in() -> anyhow::Result<ChildStdout> {
    let mut cmd = "ffmpeg -hide_banner -f v4l2 -vcodec mjpeg -r 30 -video_size 1920x1080 -i /dev/video0 -vf format=rgb24 -f rawvideo -".split_whitespace();
    let mut input = Command::new(cmd.next().unwrap())
        .args(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(input.stdout.take().unwrap())
}

pub fn get_pipe_out() -> anyhow::Result<ChildStdin> {
    let mut cmd = "ffmpeg -hide_banner -pix_fmt rgb24 -f rawvideo -r 30 -video_size 1920x1080 -i - -f v4l2 /dev/video10".split_whitespace();
    let mut output = Command::new(cmd.next().unwrap())
        .args(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    Ok(output.stdin.take().unwrap())
}
