use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug)]
struct Expense {
    cost: Decimal,
    concept: String,
    date: NaiveDate,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get file path from user
    println!("Please enter the path to your CSV file:");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;
    let file_path = file_path.trim();

    // Read CSV
    let expenses = read_csv(file_path)?;

    // Generate report
    let report = generate_report(&expenses)?;

    // Create output filename
    let input_path = Path::new(file_path);
    let stem = input_path.file_stem().unwrap().to_str().unwrap();
    let output_filename = format!("{}Report.md", stem);

    // Write report to file
    let mut output_file = File::create(&output_filename)?;
    output_file.write_all(report.as_bytes())?;

    println!("Report generated: {}", output_filename);
    Ok(())
}

fn read_csv(file_path: &str) -> Result<Vec<Expense>, Box<dyn Error>> {
    let mut expenses = Vec::new();
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let cost: Decimal = record[0].parse()?;
        let concept = record[1].to_string();
        let date = NaiveDate::parse_from_str(&record[2], "%d-%m-%Y")?;

        expenses.push(Expense {
            cost,
            concept,
            date,
        });
    }

    Ok(expenses)
}

fn generate_report(expenses: &[Expense]) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();

    // Total sum
    let total: Decimal = expenses.iter().map(|e| e.cost).sum();
    report.push_str("# Expense Report\n\n");
    report.push_str(&format!("## Total Expenses\n**{:.2}**\n\n", total));

    // Sum by concept
    report.push_str("## Expenses by Concept\n");
    let mut concept_sums: HashMap<String, Decimal> = HashMap::new();
    for expense in expenses {
        *concept_sums.entry(expense.concept.clone()).or_insert(dec!(0.0)) += expense.cost;
    }

    let mut concept_sums: Vec<_> = concept_sums.into_iter().collect();
    concept_sums.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (concept, sum) in concept_sums {
        report.push_str(&format!("- {}: {:.2}\n", concept, sum));
    }
    report.push_str("\n");

    // Days with most entries
    report.push_str("## Top 5 Days by Number of Entries\n");
    let mut date_entries: HashMap<NaiveDate, (i32, Decimal)> = HashMap::new();
    for expense in expenses {
        let entry = date_entries.entry(expense.date).or_insert((0, dec!(0.0)));
        entry.0 += 1;
        entry.1 += expense.cost;
    }

    let mut date_entries: Vec<_> = date_entries.into_iter().collect();
    date_entries.sort_by(|a, b| b.1.0.cmp(&a.1.0));

    for (date, (count, sum)) in date_entries.iter().take(5) {
        report.push_str(&format!("- {}: {} entries, total: {:.2}\n",
            date.format("%d-%m-%Y"), count, sum));
    }
    report.push_str("\n");

    // Top 5 expenses
    report.push_str("## Top 5 Expenses\n");
    let mut top_expenses: Vec<_> = expenses.iter().collect();
    top_expenses.sort_by(|a, b| b.cost.partial_cmp(&a.cost).unwrap());

    for expense in top_expenses.iter().take(5) {
        report.push_str(&format!("- {:.2} ({}) on {}\n",
            expense.cost,
            expense.concept,
            expense.date.format("%d-%m-%Y")));
    }

    Ok(report)
}
