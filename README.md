# SS_Crusty 🕸🦀🕸 (Simple Scraper, Crusty Rust Edition)

A fast, **multithreaded and concurrent** web scraping utility built in **Rust** for fetching raw HTML content and extracting all absolute links from a list of URLs. It's designed for simple, high-speed data collection and outputs clean JSON.

--Made out of boredom--

## 🛣️ Development Roadmap
Future features might include a full distributed crawling framework, dynamic JavaScript rendering, and a personalized pizza delivery bot.

Seriously though...

### Immediate Plans (Next Releases)

* Configuration Files: Support for managing inputs/settings via Json, yaml or [Something Completely Different](https://www.imdb.com/title/tt0066765/).
* `robots.txt` Respect: Respect: Automatic adherence to website exclusion rules.
* Targeted Data Extraction: Allow extraction of specific data using CSS selectors.

### Later Plans

* Structured Data Output: Output results in defined, structured formats (schemas).
* Anti-Bot Defenses: Implement rate limiting and proxy support for OpSec tooling.
* Session Management: Add cookie and header control for authenticated scraping.

---

## ✨ Features

* **High-Speed Concurrency:** Uses the **`rayon`** crate for **multithreaded, parallel** processing of multiple URLs, significantly improving scraping speed.
* **Versatile Input:** Supports loading URLs from four different file formats: **JSON, CSV, XML, and TXT**.
* **Simple Output:** Results are saved to a **pretty-printed JSON file** containing the full HTML and a list of all extracted absolute links for each URL.
* **Flexible Input:** Provide target URLs directly via command-line arguments (`--urls`) or load them from one or **multiple files** (`--file`).
* **Custom User Agent:** Easily set a standard User-Agent string (`Mozilla`, `Webkit`, or `Chrome`) to manage requests politely.
* **Rust-Native Performance:** Leveraging Rust for safety and execution speed.

---

## ⚠️ Disclaimer

**SS_Crusty** is a command-line tool designed for technical demonstration, learning, and analysis.

**You use this tool at your own risk.**

The user of this software is entirely responsible for adhering to all applicable local, national, and international laws, including but not limited to, the **Terms of Service (ToS)** and **robots.txt** files of any websites they scrape.

* **Respect Website Policies:** Always review a website's `robots.txt` file and its Terms of Service before scraping.
* **Rate Limiting:** This tool is concurrent; excessive use or rapid requests may overload a target site or result in your IP address being blocked. The user is responsible for implementing any necessary rate-limiting or delay mechanisms not built into the tool.
* **Liability:** The author of SS_Crusty is not responsible for any direct, indirect, or consequential damages resulting from the use of this software, including any legal action or bans resulting from misuse.

## 🛠️ Installation

You can either **download a pre-built binary** for Windows and Linux from the [Releases page](https://github.com/Fairdose/ss_crusty/releases).
For MacOS see below.

### Building from Source

This project requires the **Rust toolchain**. If you don't have it, you can install it via [rustup].

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/Fairdose/ss_crusty.git
    cd rust-web-scraper
    ```
2.  **Build the project:**
    ```bash
    cargo build --release
    ```
3.  The executable will be located at `./target/release/ss_crusty`.

---

## 🚀 Usage

The scraper accepts URLs from either command-line arguments or a file path.

### Command-Line Arguments

Use the built-in help for a quick reference: `ss_crusty --help`

| Argument                  | Description                                                                                                                 | Default                            |
|:--------------------------|:----------------------------------------------------------------------------------------------------------------------------|:-----------------------------------|
| **`--urls <URL>`**        | URLs to fetch. You can specify this argument multiple times to scrape several pages.                                        | (Required if `--file` is not used) |
| **`--file <PATH>`**       | Path to a file containing a list of URLs. The current implementation accepts; CSV, xml, JSON, txt (With proper formatting). |                                    |
| **`--output <PATH>`**     | The path where the output JSON results will be saved.                                                                       | `results.json`                     |
| **`--user-agent <NAME>`** | Optional user-agent for HTTP requests. Supported values: `Mozilla`, `Webkit`, `Chrome`.                                     | Default composite user agent       |
| **`--v, --vv, --vvv`**    | Verbose `--v = INFO` `--vv = WARN` `--vvv = DEBUG`                                                                          |                        |

### Examples

#### 1. Scraping a Single URL

Fetch the HTML and links from a single page, saving the result to the default `results.json`.

```bash
    ss_crusty --urls "https://example.com"
```

#### 2. Scraping Multiple URLs with a Custom User-Agent

Scrape two different pages and set the User-Agent to mimic Chrome.

```bash
    ss_crusty \
        --urls "https://example1.com") \
        --urls "https://example2.com" \
        --user-agent Chrome
```

#### 3. Reading URLs from Multiple Files

Process URLs found in both `batch1.json` and `batch2.json`, and save the combined results.

`batch1.json`
```json
  { "urls": ["https://example.com/page1", "https://example.com/page2"] 
```

`batch2.json`
```json
  { "urls": ["https://example.com/a", "https://example.com/b"] }
```

You can process them using the multiple `--file` flag:

```bash
    ss_crusty \
        --file batch1.json \
        --file batch2.json \
        --output combined_results.json
```

### 📝 Output Format

The output is a single JSON object containing a pages array, where each element represents a scraped URL and its associated data.

Example results.json:

```json
{
  "pages": [
    {
      "url": "[https://example.com](https://example.com)",
      "links": [
        "[https://www.iana.org/domains/example](https://www.iana.org/domains/example)",
        "[https://another-absolute-link.com/page](https://another-absolute-link.com/page)" 
        "...more links"
      ],
      "html": "<!doctype html>\n<html>\n<head>...</head></html> <----HTML Content"
    },
    {
      "... more Page Results Content"
    }
  ]
}
```

### 📂 Input File Formats

The scraper automatically attempts to parse the file content based on common conventions for the following four supported formats. Do not worry about duplications 😉

#### 1. JSON Format

Files must contain a single JSON object with a key named **`urls`** that holds an array of URL strings.

**File Content Example (`input.json`):**
```json
{ 
  "urls": [
    "https://example1.com", 
    "https://example2.com"
  ] 
}
```

#### 2. CSV Format

Files must have a column named url containing the target URLs.

**File Content Example (`input.csv`):**
```csv
url
https://example1.com
https://example2.com
```

#### 3. Plain Text (TXT) Format

Files must contain one URL per line.

**File Content Example (`input.txt`):**
```
url
https://example1.com
https://example2.com
```

#### 4. XML Format

Files must use an <urls> root element with URLs contained within nested <url> tags.

**File Content Example (`input.xml`):**
```xml
<urls>
  <url>https://example1.com</url>
  <url>https://example2.com</url>
</urls>
```


### ⚙️ How it Works (Under the Hood)

The core logic is handled by the scrape module:

* The tool reads all URLs from the command-line arguments and all specified files, inferring the format to correctly parse URLs from each source.

* All URLs are merged, sorted, and deduplicated before processing.

* The [`reqwest::blocking`](https://docs.rs/reqwest/latest/reqwest/blocking/index.html) client is used for fetching, configured with the specified user agent.

* Parallelism is achieved using rayon, allowing multiple requests to happen simultaneously.

* The scraper crate is used to parse the HTML and extract all href attributes from `<a>` tags, filtering to keep only absolute links (starting with 'http://' or 'https://').

### ⚖️ License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE)