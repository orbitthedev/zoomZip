use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;
use std::time::Instant;
use zip::ZipArchive;

fn main() -> io::Result<()> {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: zoomZip <file.zip> [--mode fast|normal|bench]");
        return Ok(());
    }

    let zip_path = &args[1];

    // Mode handling
    let mut mode = "normal".to_string();

    if let Some(pos) = args.iter().position(|x| x == "--mode") {
        if let Some(m) = args.get(pos + 1) {
            mode = m.clone();
        }
    }

    let bench = mode == "bench";

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    let out_dir = Path::new(zip_path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    fs::create_dir_all(&out_dir)?;

    let total = archive.len() as f64;
    let mut processed: usize = 0;

    let mut buffer = vec![0u8; 1024 * 1024];

    let mut bytes_done: u64 = 0;

    for i in 0..archive.len() {
        let mut zipped_file = archive.by_index(i)?;

        let outpath = Path::new(&out_dir).join(zipped_file.name());

        if zipped_file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let outfile = File::create(&outpath)?;
            let mut writer = BufWriter::new(outfile);

            loop {
                let bytes = zipped_file.read(&mut buffer)?;
                if bytes == 0 {
                    break;
                }

                writer.write_all(&buffer[..bytes])?;
                bytes_done += bytes as u64;
            }
        }

        processed += 1;

        let progress = (processed as f64 / total) * 100.0;

        if mode == "normal" {
            let bar_width = 20;
            let filled = ((progress / 100.0) * bar_width as f64) as usize;

            let bar = format!(
                "[{}{}] {:.1}%",
                "#".repeat(filled),
                "-".repeat(bar_width - filled),
                progress
            );

            println!("{} {}", bar, zipped_file.name());
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let mb = bytes_done as f64 / (1024.0 * 1024.0);
    let speed = mb / elapsed;

    println!("\nDone extracting: {}", zip_path);

    if bench || mode == "fast" {
        println!("Files: {}", processed);
        println!("Time: {:.2?}", start.elapsed());
        println!("Speed: {:.2} MB/s", speed);
        println!("Mode: {}", mode);
    }

    Ok(())
}