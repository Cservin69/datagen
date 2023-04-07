mod data;
mod myerror;

use myerror::MyError;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{self, BufWriter, prelude::*};
use chrono::NaiveDate;
use crate::data::countries::COUNTRIES;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use crate::data::ccy::CCY;
use crate::data::products::PRODUCTS_AND_SERVICES;


fn main() -> Result<(), MyError> {
    let json_file_path = "queries.json";
    let output_file_path = "clientids.csv";


    let read_clients: Vec<String> = Vec::new();
    let clients_arc = Arc::new(Mutex::new(read_clients.clone()));
    parse_client_ids_and_insert_into_vec(&json_file_path, &output_file_path, &clients_arc)?;


// Define the possible values for each column
    let clients = clients_arc.lock().unwrap();
    let country_of_origin = vec!["New Zealand", "Singapore", "Viet Nam"];
    let transactions = vec!["DEBIT", "CREDIT", "WITHDRAWAL", "DEPOSIT"];
    let iscash = vec!["CASH", "TRANSFER"];
    let partners = (1..=55).map(|n| format!("partner{}", n)).collect::<Vec<_>>();
    let currencies = CCY;
    let countries = COUNTRIES;
    let producst_services = PRODUCTS_AND_SERVICES;

// Open the file for writing using a buffered writer
    let file = File::create("transactions.csv")?;
    let mut writer = BufWriter::new(file);

// Write the header row
    writeln!(
        writer,
        "client,country_of_origin,transaction_type,date,partner,destination_country,ccy,amount,product"
    )?;

// Create a random number generator
    let mut rng = thread_rng();

// Write 100 data rows
    for _ in 0..10000 {
        let client = random_element(&clients)?;
        // rest of the code that uses the `client` variable
        let orszag = random_element(&country_of_origin)?;
        let transaction = random_element(&transactions)?;
        let date = NaiveDate::from_ymd_opt(2023, 6, 6).unwrap() -
            chrono::Duration::days(rng.gen_range(0..500));
        // let partner = random_element(&partners)?;
        let partner = if *transaction == "WITHDRAWAL" || *transaction == "DEPOSIT" {
            client.clone()
        } else {
            random_element(&partners)?.to_owned()
        };

        let country = random_element(countries)?;
        let ccy = random_element(currencies)?;
        let amount: u32 = rng.gen_range(0..50000); // Change the range of `amount`


        let product = match transaction {
            &"WITHDRAWAL" | &"DEPOSIT" => random_element(&iscash)?,
            &"DEBIT" | &"CREDIT" => random_element(&producst_services)?,
            _ => unreachable!(), // this should never happen
        };

        writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{}",
            client,
            orszag,
            transaction,
            date.format("%Y-%m-%d"), // format the date as YYYY-MM-DD
            partner,
            country,
            ccy,
            amount,
            product
        )?;
    }

    Ok(())
}


fn parse_client_ids_and_insert_into_vec(json_file_path: &str, output_file_path: &str,
                                        clients: &Arc<Mutex<Vec<String>>>) -> serde_json::Result<()> {
    println!("started the vec parsing");
    // Open the input JSON file and read its contents
    let mut file = File::open(json_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the top-level JSON object and extract the "data" array
    let json_object: serde_json::Value = serde_json::from_str(&contents)?;
    let data_array = json_object.get("data").unwrap().as_array().unwrap();
    // Pre-allocate memory for the `clients` vector
    clients.lock().unwrap().reserve(data_array.len());

    // Extract the "key" parameter from each object in the "data" array and write it to the output file
    let output_file = Arc::new(Mutex::new(File::create(output_file_path).unwrap()));
    writeln!(output_file.lock().unwrap(), "ClientId").unwrap();

    data_array.par_iter().for_each(|json_object| {
        if let Some(client_id) = json_object.get("key") {
            let client_id_str = client_id.as_str().unwrap().to_string();
            writeln!(output_file.lock().unwrap(), "{}", client_id_str).unwrap();
            clients.lock().unwrap().push(client_id_str);
        }
    });

    Ok(())
}

fn random_element<T>(vec: &[T]) -> Result<&T, MyError> {
    if vec.is_empty() {
        return Err(MyError::IoError(io::Error::new(io::ErrorKind::Other, "vector is empty")));
    }else {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..vec.len());
        Ok(&vec[index])
    }
}

