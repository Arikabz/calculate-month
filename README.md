# Expense Report Generator

A simple Rust utility that generates formatted expense reports from CSV files.

## Description

This utility takes a CSV file containing expense records and generates a detailed markdown report that includes:
- Total sum of all expenses
- Breakdown of expenses by concept
- Top 5 days with the most entries (including their total costs)
- Top 5 highest individual expenses

## CSV File Format

Your CSV file must follow this specific format:

```csv
cost,concept,date
123.45,Groceries,25-12-2023
67.89,Utilities,26-12-2023
```

### Requirements:
- File must have a header row
- Columns must be in the exact order: cost,concept,date
- Cost should use decimal point (.) as separator
- Dates must be in DD-MM-YYYY format
- Fields must be separated by commas (,)

## Installation

1. Clone this repository
2. Make sure you have Rust installed
3. Build the project:
```bash
cargo build --release
```

## Usage

1. Run the program:
```bash
cargo run
```

2. When prompted, enter the path to your CSV file. Examples:
```
data.csv                                   # File in current directory
./data.csv                                 # File in current directory
C:/Users/YourName/Documents/data.csv       # Windows absolute path
/home/username/documents/data.csv          # Linux/MacOS absolute path
```

3. The program will generate a report file named `[your-csv-name]Report.md` in the same directory as the executable.

## Output Format

The generated report will include:

```markdown
# Expense Report

## Total Expenses
**1234.56**

## Expenses by Concept
- Groceries: 789.12
- Utilities: 445.44
...

## Top 5 Days by Number of Entries
- 25-12-2023: 3 entries, total: 234.56
- 26-12-2023: 2 entries, total: 123.45
...

## Top 5 Expenses
- 123.45 (Groceries) on 25-12-2023
- 67.89 (Utilities) on 26-12-2023
...
```

## Error Handling

The program will display an error message if:
- The CSV file cannot be found
- The file format is incorrect
- There are parsing errors in the date or cost fields
- The program doesn't have write permissions for the output file

## Technical Requirements

- Rust 2021 edition or later
- Dependencies (automatically managed by Cargo):
  - csv
  - chrono
  - rust_decimal
  - rust_decimal_macros

## License

[Your chosen license here]
