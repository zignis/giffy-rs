use gif::{Decoder, Frame};
use image::codecs::ffmpeg::Encoder as Mp4Encoder;
use image::gif::Encoder as GifEncoder;
use std::fs::{self, File};
use std::io::{self, Write};

fn convert(gif: &str, mp4: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the GIF file
    let gif_file = File::open(gif)?;
    let mut decoder = Decoder::new(gif_file);
    let mut gif_frames = decoder.into_frames();

    // Temp dir to store PNG frames
    let temp_dir = tempfile::tempdir()?;
    let temp_dir_path = temp_dir.path();

    // Split the GIF into individual PNG frames
    let mut png_frames = Vec::new();
    for (i, frame) in gif_frames.enumerate() {
        let frame = frame?;
        let mut png_path = temp_dir_path.join(format!("frame{}.png", i));
        let png_file = File::create(&png_path)?;

        // Encode the frame as PNG and write it to the file
        let mut png_encoder = GifEncoder::new(png_file);
        png_encoder.encode_frame(&frame)?;

        png_frames.push(png_path);
    }

    // Convert PNG frames to MP4 using ffmpeg
    let mut ffmpeg_cmd = std::process::Command::new("ffmpeg");
    ffmpeg_cmd.args(&["-y", "-framerate", "30", "-i"]); // Adjust the FPS

    // Append all the PNG files to the ffmpeg command
    for frame in &png_frames {
        ffmpeg_cmd.arg(frame.to_str().unwrap());
    }

    ffmpeg_cmd.args(&["-c:v", "libx264", "-pix_fmt", "yuv420p", mp4]);

    let ffmpeg_output = ffmpeg_cmd.output()?;

    if !ffmpeg_output.status.success() {
        return Err("ffmpeg command failed".into());
    }

    // Clean up temp PNG files
    for frame in png_frames {
        fs::remove_file(frame)?;
    }

    Ok(())
}

fn main() {
    let gif = "sample.gif";
    let mp4 = "out.mp4";

    match convert(gif, mp4) {
        Ok(_) => println!("Converted GIF to MP4"),
        Err(err) => println!("Error converting GIF to MP4: {}", err),
    }
}
