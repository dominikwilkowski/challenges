PROCESS_CSV
===========

A parser for CSV files with postfix notations which can include spreadsheet coordinates.

## Run

To run the processor run the below command.
```sh
cargo run --release postfix.csv -o postfix-out.csv
```

## Help

```
Usage: $ process_csv [path/to/csv] [OPTIONS]

Options:
  -V, --version        Print version info and exit
  -h, --help           Print help and exit
  -s, --separator ","  Set the separator for the CSV
  -x, --overwrite      Allow to overwrite the output file if it exists
```
