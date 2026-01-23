---
title: >-
  Cost of storing videos as “good enough” frames
  253e579bff8980948474d140c4932bb5
updated: 2026-01-14 22:54:32Z
created: 2026-01-15 04:51:09Z
---

# Cost of storing videos as “good enough” frames

Published On: August 23, 2025
Author: dilawar
Status: Published
Type: Note
Tags: ffmpeg, lang:rust

I am going to do something terrible — store videos in a SQL database! 

I have some requirements that makes it an acceptable plan. I am building a “stream store” S. Once videos are stored in S, users should be able to fetch a video segment (in color or grayscale) between two given timestamps at a given FPS. One should also be able to annotate frame later e.g., “this frame has a face in it”. I’ll read the incoming video stream and save data as frames to a database (in addition to storing raw videos on a backup server).

The primary job of S is to provide frames for CV analysis, so serving pixel-perfect video stream is not a requirement.

- The stored frames should be “good enough” can our CV algorithms/pipeline works without a performance drop.
- The cost of storing frames should be as low as possible as long as the above requirement is met.

I am going to extract frames from video as JPEG — a lossy compression format. But what should be the quality of JPEG?

### Experiments

I downloaded a sample `mkv` file — 1080p at 30 FPS to do simple analysis.

[https://filesamples.com/samples/video/mkv/sample_1280x720.mkv](https://filesamples.com/samples/video/mkv/sample_1280x720.mkv)

| FPS | 23.976 |
| --- | --- |
| Duration | 28.237 |
| Size on disk | 16.63 MB |

I wrote a script that extract frames from the `mkv` using `ffmpeg`. The argument `-qscale:v` set the quality of frames: 2 is the best and 32 is the worst.

- `ffmpeg -i ../sample_1280x720.mkv '%04d.png'` generates PNG folder which is 1.3 GB, almost 78x of original size. PNG is a lossless format. This is as bad as its get.
- `ffmpeg -i ../sample_1280x720.mkv '%04d.jpg'` generates JPEG with default quality picked by ffmpeg. It generates 33 MB of data. Almost 1.94x more. Great!
- `ffmpeg -i ../sample_1280x720.mkv -qscale:v 2 '%04d.jpg'` generates JPEG with best possible quality. The generated size is 188 MB (almost 11x more!).

I did the same on a different file recorded at 60fps (original size . The data is below.

[Size of frames (various FPS)](../../../_resources/Size%20of%20frames%20%28various%20FPS%29%20254e579bff89806185ccd.csv)

A few things to note

- A video recorded at 60FPS takes more than twice the size compared to fps=30 case. Note video are using codec H264 (MPEG-4 AVC (part 10) (avc1)) which AFAIK compresses pretty well if two consecutive frames don’t differ too much. So this is expected.
- Our recordings have the same feature. The interesting events happens rarely. We might drop many frames from the database after doing a quick analysis to figure out if they contain something interesting or not. So in the end, we don’t even have to store so many frame.

I think `qscale:v=20` is a good default for my use case. Also I don’t have to extract frames at the same rate as they are recording. I am interested in events at the timescale of ~100ms and anything faster than 20FPS is overkill. i can just extract at 30 fps.

`ffmpeg` has a handy cli option `-filter:v "fps=30"` to fix the extraction fps to 30. Here is bonus rust code that does this. Don’t copy-paste blindly, it may not work.

```rust
/// Explode the given video into JPEG frames.
///
/// - *path*: Path of video file
/// - *fps*: Extract these many frames per seconds. The video may contain
///   more or less frames in a second.
pub fn extract_jpegs<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
    fps: u16,
    recording_start_timestamp_ms: i64
) -> anyhow::Result<usize> {
    anyhow::ensure!(path.as_ref().exists(), "{:?} does not exists", path);
    tracing::debug!(
        "Extracting frames from {:?} for fps={fps} and jpg qscale {:?}.",
        path.as_ref(),
        self.qscale
    );

    let inst = Instant::now();
    let mut cmd = std::process::Command::new(&self.ffmpeg_bin_path);
    cmd.arg("-i").arg(path.as_ref());

    // Extract at a given fps. Thanks <https://askubuntu.com/a/1019417/39035>.
    cmd.arg("-filter:v").arg(format!("fps={fps}"));

    if let Some(qscale) = self.qscale {
        tracing::debug!("Setting qscale to {qscale}");
        cmd.arg("-quality:v");
        cmd.arg(qscale.to_string());
    }

    cmd.arg(self.frame_directory.join("%05d.jpg"));
    let output = cmd.output()?;
    anyhow::ensure!(
        output.status.success(),
        "Command failed\n.{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    tracing::debug!(
        "Extraction to {:?} is complete, took {:?}.",
        &self.frame_directory,
        inst.elapsed()
    );
}
```