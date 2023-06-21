# carbone_cli_rs

This is a simple CLI-App to generate report with the API of Carbone.

# Usage

```bash
➜ cargo run -- -h
   Compiling carbone_cli_rs v0.1.0 (/Users/pascal/repos/carbone_cli_rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/carbone_cli_rs -h`
Simple CLI-App to communicate with the API of Carbone (http://carbone.io)

Usage: carbone_cli_rs [OPTIONS] --template <TEMPLATE>

Options:
  -c, --config-file-path <CONFIG_FILE_PATH>  path of the config file
  -j, --json-data <JSON_DATA>                json data to be rendered
  -t, --template <TEMPLATE>                  template file
  -o, --output <OUTPUT>                      output file for the generated report
  -h, --help                                 Print help
  -V, --version                              Print version
```

# Example

```bash
cargo run -- -c test_data/config.json -j test_data/report_data.json -o new_report.pdf -t test_data/template.test.odt
```

# References

(carbone_sdk_rs)[https://github.com/pascal-chenevas/carbone_sdk_rs]
