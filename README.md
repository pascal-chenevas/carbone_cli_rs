[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)

# carbone_cli_rs

this is a CLI-App to communicate with the API of Carbone

# Usage

## Run from the sources

```bash
➜ cargo run -- -h
Simple CLI-App to generate a report and to manage templates using the API of Carbone (http://carbone.io)

Usage: carbone_cli_rs [OPTIONS]

Options:
  -c, --config <FILE>
          a configuration file which contains the api url, timeout and api version
  -j, --json <FILE>
          json data to be rendered
  -t, --template <FILE>
          template file
  -g, --generate-template-id
          template id
  -o, --output <FILE>
          output file for the generated report
  -r, --remove-template <TEMPLATE_ID | FILE>
          remove a template with the given template_id or from a file
  -u, --update
          update a template
  -d, --download-template <TEMPLATE_ID>
          download a template
  -h, --help
          Print help
  -V, --version
          Print version
```

# Example

(run from the sources)

### create a report

```bash
cargo run --  -j test_data/report_data.json -t test_data/template.test.odt -o /tmp/report.pdf

{
  "bytes": 92989,
  "created": true,
  "error": null,
  "output": "/tmp/report.pdf"
}
```
