# carbone_cli_rs

this is a CLI-App to communicate with the API of Carbone

# Usage

```bash
➜ cargo run -- -h
Simple CLI-App to generate a report using the API of Carbone (http://carbone.io)

Usage: carbone_cli_rs [OPTIONS] --config <FILE>

Options:
  -c, --config <FILE>
          a configuration which contains the api url, timeout and api version
  -j, --json <FILE>
          json data to be rendered
  -t, --template <FILE>
          template file
  -o, --output <FILE>
          output file for the generated report
  -r, --remove-template <TEMPLATE_ID>
          remove a template with the given template_id
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

```bash
cargo run -- -c test_data/config.json -j test_data/report_data.json -o new_report.pdf -t test_data/template.test.odt
```