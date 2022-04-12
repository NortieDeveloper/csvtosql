# csvtosql

This is a simple program written in Rust that reads in a csv file and generates a sql statement to create a table from the headers in the csv. As of right now, this tool only outputs in sql server dialect.

## Getting Started


### Installing

Download binary release and add the executable to your path,
or build from source using `cargo build`

### Executing program

Simply run:
```
csvtosql --table=ExampleTable --database=ExampleDB example.csv
```
This will result in the creation of a new file called ExampleTable.sql which will contain the generated table creation code.
## Help
```
csvtosql --help
```

## License

This project is licensed under the MIT License - see the LICENSE.md file for details
