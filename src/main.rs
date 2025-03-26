use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let stream_file = args.get(1).expect("please provide the stream file");

    if let Ok(lines) = read_lines(stream_file) {
        println!("Reading from {stream_file}");

        let mut current_h5 = 0;
	let mut current_sn = 0;
	let mut histogram: HashMap<i32, i32> = HashMap::new();

        for line in lines.map_while(Result::ok) {
	    if line.starts_with("Image filename: ") {
		current_h5 = line[line.len()-9..line.len()-3].parse::<i32>().unwrap();
		// println!("h5 number {current_h5}");
	    } else if line.starts_with("Image serial number: ") {
		current_sn = line[21..line.len()].parse::<i32>().unwrap();
	    } else if line.starts_with("indexed_by = x") {
		let current_number = (current_h5 - 1) * 1000 + current_sn;
		let current_hist_index = current_number / 1000;
		// println!("{current_number}: {current_hist_index}");

		match histogram.get(&current_hist_index) {
		    Some(count) => histogram.insert(current_hist_index, count+1),
	            None =>  histogram.insert(current_hist_index, 1)
		};
	    }
        }

	for (id, count) in &histogram {
	    println!("{id}: {count}");
	}
    }
}
