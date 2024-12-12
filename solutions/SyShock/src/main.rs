use std::{env, process};

mod csv_solution {
    use csv::StringRecord;
    use std::{error::Error, io};

    #[allow(dead_code)]
    pub fn example() -> Result<(), Box<dyn Error>> {
        // Build the CSV reader and iterate over each record.
        let mut rdr = csv::Reader::from_reader(io::stdin());
        for result in rdr.records() {
            let record: StringRecord = result?;
            println!("{:?}", record);
        }
        Ok(())
    }
}

mod solution {
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs::read_to_string;

    type WeatherHashMap = HashMap<String, Vec<f64>>;

    pub fn read_lines(filename: &str) -> Result<WeatherHashMap, Box<dyn Error>> {
        let mut weather_entries: WeatherHashMap = HashMap::new();
        for line in read_to_string(filename)?.lines() {
            let line = line.to_string();
            let mut split_line: Vec<&str> = line.split(";").collect();
            let name = split_line.remove(0);
            let mut values: Vec<f64> = split_line.into_iter().map(|v| v.parse().unwrap()).collect();
            let result = weather_entries
                .entry(name.to_string())
                .or_insert_with(|| Vec::new());
            result.append(&mut values);

            // this definately could use optimization
            result.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        Ok(weather_entries)
    }

    pub fn print_hash(mut hash: WeatherHashMap) {
        print!("{{\n");
        let vec: Vec<String> = hash.keys().cloned().collect();
        let last = vec[vec.len() - 1].clone();
        let (last_k, last_v) = hash.remove_entry(last.as_str()).unwrap();
        hash.into_iter().for_each(|(k, v)| {
            let vec_to_string = {
                let vec: Vec<String> = v.iter().map(ToString::to_string).collect();
                vec.join("\\")
            };
            println!("\t{k}={vec_to_string},");
        });
        let vec_to_string = {
            let vec: Vec<String> = last_v.iter().map(ToString::to_string).collect();
            vec.join("\\")
        };
        println!("\t{last_k}={vec_to_string}");
        print!("}}\n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    match solution::read_lines(file_path.as_str()) {
        Err(err) => {
            println!("Error running example: {}", err);
            process::exit(1);
        }
        Ok(v) => {
            solution::print_hash(v);
        }
    }
}
