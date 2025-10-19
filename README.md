# SS Crusty

A multi-format URL scraper built in Rust. Supports JSON, TXT, XML, and CSV input files. Uses Rayon, Arc, Mutex, and channels for parallel processing.

## Features

- Parse URLs from JSON, TXT, XML, and CSV
- Multi-threaded scraping with Rayon
- Thread-safe data handling using Arc and Mutex
- Supports both local files and URLs as input
- Cross-platform: Windows, Linux, macOS

## Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/ss_crusty.git
cd ss_crusty
