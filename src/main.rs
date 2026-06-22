mod fzf;
use std::collections::hash_map::HashMap;

#[derive(Debug, Clone)]
struct Code {
    country_name: String,
    iso_codes: Vec<String>,
    num_codes: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut codes: Vec<Code> = Vec::new();
    let mut codes_fzf: Vec<fzf::Item<Code>> = Vec::new();

    let mut reader = csv::Reader::from_path("codes.csv")?;
    for result in reader.records() {
        let record = result?;
        let country_name = record.get(0).unwrap();
        let iso_codes = record.get(1).unwrap();
        let num_codes = record.get(2).unwrap();

        let iso_codes = iso_codes
            .split('/')
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| str.to_string())
            .collect::<Vec<String>>();

        let code = Code {
            country_name: country_name.to_string(),
            iso_codes: iso_codes.clone(),
            num_codes: num_codes
                .split('|')
                .collect::<Vec<&str>>()
                .iter()
                .map(|str| str.to_string())
                .collect::<Vec<String>>(),
        };

        codes_fzf.push(fzf::Item::new(
            code.country_name.clone(),
            iso_codes,
            code.clone(),
        ));
        // for iso in &code.iso_codes {
        //     codes_fzf.push(Item::new(iso.to_string(), code.clone()));
        // }
        codes.push(code);
    }

    // maps a country's name to its code struct
    let name_map: HashMap<&str, &Code> = codes
        .iter()
        .map(|code| (code.country_name.as_str(), code))
        .collect();

    // maps each of a country's iso codes to its code struct
    let iso_map: HashMap<&str, &Code> = codes
        .iter()
        .flat_map(|code| {
            code.iso_codes
                .iter()
                .map(|iso| (iso.as_str(), code))
                .collect::<Vec<(&str, &Code)>>()
        })
        .collect();

    // maps each of a country's numerical codes to its code struct
    let code_map: HashMap<&str, &Code> = codes
        .iter()
        .flat_map(|code| {
            code.num_codes
                .iter()
                .map(|num| (num.as_str(), code))
                .collect::<Vec<(&str, &Code)>>()
        })
        .collect();

    println!("name_map: {:?}", code_map);

    let fzf = fzf::find(codes_fzf, 10)?;
    let answer = match fzf {
        Some(answer) => answer,
        None => {
            println!("answer please");
            return Ok(());
        }
    };

    println!("answer: {:?}", answer);

    Ok(())
}
