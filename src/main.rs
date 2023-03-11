mod data;
mod myerror;

use myerror::MyError;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{self, prelude::*};
use data::clients::CLIENTS;
use chrono::{NaiveDate, Datelike};
use crate::data::countries::COUNTRIES;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use serde_json::Value;
use crate::data::products::PRODUCTS_AND_SERVICES;


fn main() -> std::result::Result<(), MyError> {
    let json_file_path = "queries.json";
    let output_file_path = "clientids.csv";

    //dead code @todo
    parse_client_ids_from_json(&json_file_path, &output_file_path).unwrap();
    let mut read_clients: Vec<String> = Vec::new();
    let clients_arc = Arc::new(Mutex::new(read_clients.clone()));
    parse_client_ids_and_insert_into_vec(&json_file_path, &output_file_path, &clients_arc)?;


    // Define the possible values for each column
    let clients= CLIENTS;
    let country_of_origin = vec!["New-Zealand", "Singapore", "Vietnam"];
    let transactions = vec!["DEBIT", "CREDIT"];
    let partners = (1..=55).map(|n| format!("partner{}", n)).collect::<Vec<_>>();
    let countries = COUNTRIES;
    let producst_services = PRODUCTS_AND_SERVICES;


    // Open the file for writing
    let mut file = File::create("transactions.csv")?;

    // Write the header row
    writeln!(file, "client,country_of_origin,transaction_type,date,partner,destination_country,product")?;

    // Create a random number generator
    let mut rng = thread_rng();

    // Write 100 data rows
    for _ in 0..100 {
            let client = random_element(&clients)?;
            // rest of the code that uses the `client` variable

        let transaction = random_element(&transactions)?;
        let date = NaiveDate::from_ymd(2022, 1, 1) - chrono::Duration::days(rng.gen_range(0..365));
        let partner = random_element(&partners)?;
        let country = random_element(&countries)?;
        let amount: u32 =  rng.gen_range(0..500)*100;
        let product = random_element(&producst_services)?;

        writeln!(
            file,
            "{},{},{},{},{},{},{},{}",
            client,
            random_element(&country_of_origin)?,
            transaction,
            date.format("%Y-%m-%d"), // format the date as YYYY-MM-DD
            partner,
            country,
            amount,
            product
        )?;

    }

    Ok(())
}
// fn parse_client_ids_from_json(json_file_path: &str, output_file_path: &str) -> std::io::Result<()> {
//     // Open the input JSON file and read its contents
//     let mut file = File::open(json_file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//
//     // Parse the JSON contents into a vector of JSON objects
//     let json_objects: Vec<serde_json::Value> = serde_json::from_str(&contents)?;
//
//     // Extract the "ClientID" parameter from each JSON object and write it to the output file
//     let mut output_file = File::create(output_file_path)?;
//     writeln!(output_file, "ClientID")?;
//     for json_object in json_objects {
//         if let Some(client_id) = json_object.get("clientId") {
//             let client_id_str = client_id.as_str().unwrap();
//             writeln!(output_file, "{}", client_id_str)?;
//         }
//     }
//
//     Ok(())
// }
fn parse_client_ids_from_json(json_file_path: &str, output_file_path: &str) -> std::io::Result<()> {
    // Open the input JSON file and read its contents
    let mut file = File::open(json_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON contents into a vector of JSON objects
    let json_objects: Vec<serde_json::Value> = serde_json::from_str(&contents)?;

    // Extract the "ClientID" parameter from each JSON object and write it to the output file
    let mut output_file = File::create(output_file_path)?;
    writeln!(output_file, "ClientId")?;
    for json_object in json_objects {
        if let Some(client_id) = json_object.get("clientId") {
            let client_id_str = client_id.as_str().unwrap();
            writeln!(output_file, "{}", client_id_str)?;
        }
    }

    Ok(())
}


// fn parse_client_ids_and_insert_into_vec(json_file_path: &str, output_file_path: &str,
//                                         clients: &Arc<Mutex<Vec<String>>>) -> serde_json::Result<()> {
//     // Open the input JSON file and read its contents
//     let mut file = File::open(json_file_path).unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//
//     // Parse the JSON contents into a vector of JSON objects
//     let json_objects: Vec<Value> = serde_json::from_str(&contents)?;
//
//     // Extract the "ClientID" parameter from each JSON object and write it to the output file
//     let output_file = Arc::new(Mutex::new(File::create(output_file_path).unwrap()));
//     writeln!(output_file.lock().unwrap(), "ClientID").unwrap();
//
//     json_objects.par_iter().for_each(|json_object| {
//         if let Some(client_id) = json_object.get("clientId") {
//             let client_id_str = client_id.as_str().unwrap().to_string();
//             writeln!(output_file.lock().unwrap(), "{}", client_id_str).unwrap();
//             clients.lock().unwrap().push(client_id_str);
//         }
//     });
//
//     Ok(())
// }

fn parse_client_ids_and_insert_into_vec(json_file_path: &str, output_file_path: &str,
                                        clients: &Arc<Mutex<Vec<String>>>) -> serde_json::Result<()> {
    // Open the input JSON file and read its contents
    let mut file = File::open(json_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the JSON contents into a vector of JSON objects
    let json_objects: Vec<Value> = serde_json::from_str(&contents)?;

    // Extract the "ClientID" parameter from each JSON object and write it to the output file
    let output_file = Arc::new(Mutex::new(File::create(output_file_path).unwrap()));
    writeln!(output_file.lock().unwrap(), "ClientID").unwrap();

    json_objects.par_iter().for_each(|json_object| {
        if let Some(client_id) = json_object.get("clientId") {
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

