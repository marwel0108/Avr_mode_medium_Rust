use std::collections::HashMap;

fn main() {

    let mut arr = vec![4,3,8,4,1,5,9,0,4,2,8]; 
    let mut avr: f64 = 0.0; 
    let mut mode:  i32 = 0;
    let mut map = HashMap::new();

    arr.sort();

    let median = arr.len() as i32 / 2 as i32;

    for i in &arr {
        avr += *i as f64;
    }

    avr = avr / arr.len() as f64; 

    for i in &arr {
        let count = map.entry(i).or_insert(0);

        *count += 1;
    }

    for (_, value) in map.iter() {

        mode = if *value > mode {
            *value as i32 
        } else {
            mode
        }
    }

    println!("Average: {}", avr);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}

