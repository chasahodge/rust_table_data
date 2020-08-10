extern crate chrono;
use chrono::prelude::*;

mod datemod;
mod budget;
// mod client;

use budget::Budget;
use datemod::*;
//use client::Client;

#[derive(Debug)]
enum TableCell {
    Label(String),
    Float(f32),
}

fn main() {
    let today: Date<Local> = Local::now().date();
    let client_budget = Budget {
        id: 1,
        category: "Services".to_string(), 
        subcategory: "IT Concerige".to_string(),
        start_date: "01/01/2020".to_string(),
        end_date: "12/31/2024".to_string(),
        frequency: "Monthly".to_string(),
        cost: 1000.00
    };
    
    let end_date:Date<Local> = get_a_date(client_budget.get_end_date());

    let mut row = vec![];
    let mut table_data = Vec::new();
    if end_date >= today {
        let _start_date:Date<Local> = get_a_date(client_budget.get_start_date());
        println!("{}", client_budget.frequency.trim());

        match client_budget.frequency.trim() {
            "Monthly" => {
                row.push(TableCell::Label(client_budget.category));
                row.push(TableCell::Label(client_budget.subcategory));
                for _n in 1..37 {
                    row.push(TableCell::Float(client_budget.cost))
                }
                table_data.push(row);
            },
            "Quarterly" => {
                row.push(TableCell::Label(client_budget.category));
                row.push(TableCell::Label(client_budget.subcategory));
                for _y in 1..5 {
                    for m in 1..13 {
                        if m == 1 || m == 4 || m == 7 || m == 10 {
                            row.push(TableCell::Float(client_budget.cost))
                        } else {
                            row.push(TableCell::Label("--".to_string()));
                        }
                    }
                }
                table_data.push(row);
            },
            "Annually" => {
                row.push(TableCell::Label(client_budget.category));
                row.push(TableCell::Label(client_budget.subcategory));
                for _n in 1..4 {
                    row.push(TableCell::Float(client_budget.cost));
                    for _m in 2..12 {
                        row.push(TableCell::Label("--".to_string()));
                    }
                }
                table_data.push(row);
            },
            "OneTime" => {
                row.push(TableCell::Label(client_budget.category));
                row.push(TableCell::Label(client_budget.subcategory));
                row.push(TableCell::Float(client_budget.cost));
                for _n in 2..37 {
                    row.push(TableCell::Label("--".to_string()));
                }
                table_data.push(row);
            },
            _ => {
                println!("No Match");
            },
        }
    } else {
        println!("End Date has expired!");
    }

    for value in table_data.iter() {
        println!("{:?}", value);
        match value {
            TableCell::Label(label) => println!("{}", label),
            TableCell::Float(float) => println!("{}", float),
        }
    }
}
