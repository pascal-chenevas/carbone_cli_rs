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

### Create a report

```bash
cargo run --  -j test_data/report_data.json -t test_data/template.test.odt -o /tmp/report.pdf

{
  "bytes": 92989,
  "file": "/tmp/report.pdf",
  "state": {
    "Created": true
  }
}
```
### Generate a TemplateId 

```bash
cargo run -- -g -t test_data/template.test.odt
{
  "file": "test_data/template.test.odt",
  "templateId": "844318fe97904fb0897d4b0a47fbe9bbd1ce5c9624ae694545cbc1877f581d86"
}
```

### Upload a Template

```bash
cargo run -- -u -t test_data/template.test.odt

{
  "file": "test_data/template.test.odt",
  "state": {
    "Uploaded": true
  },
  "templateId": "844318fe97904fb0897d4b0a47fbe9bbd1ce5c9624ae694545cbc1877f581d86"
}
```


### Delete a Template

```bash
cargo run -- -r test_data/template.test.odt

{
  "state": {
    "Deleted": true
  },
  "templateId": "844318fe97904fb0897d4b0a47fbe9bbd1ce5c9624ae694545cbc1877f581d86"
}

```

