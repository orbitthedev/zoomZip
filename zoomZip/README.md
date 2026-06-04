zoomZip

A fast ZIP extraction tool written in Rust, focused on performance, simplicity, and real-time feedback.

Features
High-speed ZIP extraction
Live progress indicator (normal mode)
Fast silent mode for maximum performance
Benchmark mode with speed and timing output
Minimal CLI design with low overhead
Installation
From source
git clone https://github.com/orbitthedev/zoomZip.git
cd zoomZip
cargo build --release

The compiled binary will be located at:

target/release/zoomZip.exe
Usage
Basic extraction (with progress)
zoomZip file.zip
Fast mode (silent extraction)
zoomZip file.zip --mode fast
Benchmark mode
zoomZip file.zip --mode bench

Example output:

Files: 26
Time: 120ms
Speed: 40+ MB/s
Mode: bench
Output Example (normal mode)
[##########----------] 50.0% file.txt
[################----] 80.0% image.png
Modes
Mode	Description
normal	Progress display with file output
fast	Silent extraction optimized for speed
bench	Performance statistics only
Design Goals
Predictable performance
Low overhead execution
Simple command-line interface
Minimal dependencies
Technical Overview
Written in Rust
Uses buffered I/O for file extraction
Zip archive processing via standard ZIP handling library
Optimized for sequential disk throughput
Future Improvements
ETA calculation
Parallel extraction support
Improved progress smoothing
Windows context menu integration
Installer packaging
License

MIT

Version

v1.1.0 stable release