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
        println!("Usage: zoomxtract <zipfile>");
        return Ok(());
    }

    let zip_path = &args[1];
    let extract_path = "output";

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(extract_path)?;

    let total_files = archive.len();
    let mut buffer = vec![0u8; 1024 * 1024];

    for i in 0..total_files {
        let mut zipped_file = archive.by_index(i)?;

        let outpath = Path::new(extract_path).join(zipped_file.name());

        if zipped_file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let outfile = File::create(&outpath)?;
            let mut writer = BufWriter::with_capacity(1024 * 1024, outfile);

            loop {
                let bytes_read = zipped_file.read(&mut buffer)?;

                if bytes_read == 0 {
                    break;
                }

                writer.write_all(&buffer[..bytes_read])?;
            }
        }

        println!("[{}/{}]", i + 1, total_files);
    }

    let elapsed = start.elapsed();

    println!();
    println!("Extraction complete!");
    println!("Files processed: {}", total_files);
    println!("Time elapsed: {:.2?}", elapsed);

    Ok(())
}