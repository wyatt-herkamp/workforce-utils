# Workforce Utils

## Features

* Convert the Workforce Schedule JSON response to a ICS file
* Convert the Workforce Schedule JSON response to a txt list

## Chrome Features

* Download the Workforce Schedule JSON response

## Installation

1. Clone the Project

### Chrome Extension

1. Go to `chrome://extensions/`
2. Enable Developer Mode
3. Click on `Load Unpacked`
4. Select the `chrome` folder under the project

### CLI Tool

1. Install Rust (https://www.rust-lang.org/tools/install)
2. Run `cargo build --release`
3. You can find the binaries in `target/release/`

## Usage

1. Download the Workforce Schedule JSON response
    * Go to https://wft.homedepot.com/
    * Then you can click the extension icon under the extensions tab
    * Then click `Download Schedule as JSON`
2. Convert the JSON to ICS
    * Run `./schedule_converter <json_file> output.ics`