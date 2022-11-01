
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;

use rand::{self, Rng};
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let data = read_from_file("../data.txt");
    let mut output : Vec<i32> = Vec::new();

    //for 10 elements
    let temp = 300000.0;
    let cool_rate = 0.001;
    let best =  sim_ann(temp,cool_rate,data.clone());
    
    //writing output path
    for y in &best {
        output.push(data.iter()
        .position(|&x| x == *y).unwrap() as i32);
    }


    println!("{} {}",temp,cool_rate);

    write_out_to_file("../met-path.txt", &mut output);
    
}


fn read_from_file(file_name: &str) -> Vec<(i32,i32)>{

    let path = Path::new(file_name);
    let display = path.display();

    let file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file ,
    };
    
    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for (index, line) in reader.lines().enumerate() {

        let line = line.unwrap();

        if index == 0{
            continue;
        }

        let x = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        result.push((x[0],x[1]));

    }

    return result;
}

fn calculate_distance(x_1: (i32,i32), x_2: (i32,i32)) -> f64 {
    let  (x1,y1) = x_1;
    let (x2,y2) = x_2;
    return(f64::powf((x2-x1).into(),2.0) + f64::powf((y2-y1).into(),2.0)).sqrt();
}

fn sim_ann(mut temp:f64,cool_rate:f64,points: Vec<(i32,i32)>) -> Vec<(i32,i32)> {
    let mut rng = rand::thread_rng();

    let mut current  = points.clone();

    current.shuffle(&mut thread_rng());

    let mut best = current.clone();

    let points_len = points.iter().len();

    while temp > 1.0 {

        let mut new_solution = current.clone();

        new_solution.swap(rng.gen_range(0..points_len),rng.gen_range(0..points_len));

        let current_energy = eval(&new_solution);
        let new_energy = eval(&current);

        if acceptance(current_energy,new_energy,temp) > rng.gen_range(0.0..1.0){
            current = new_solution.clone();
        }
        
        if eval(&current) < eval(&best) {
            best = current.clone();
            println!("{:?}",eval(&best));
            
        }


        temp *= 1.0 - cool_rate; 

    }

    return best;

}


fn eval(path:&Vec<(i32,i32)>) -> f64{

    let size = path.iter().len();

    let mut distance = calculate_distance(*path.get(0).unwrap(), *path.get(size-1).unwrap());
    
    for i in 0..size-2{
        distance+=calculate_distance(*path.get(i).unwrap(), *path.get(i+1).unwrap());
    }

    return distance;

}

fn acceptance(energy:f64,new_energy:f64,temp:f64) -> f64{
    if new_energy < energy {
        return 1.0;
    }
    return ((energy - new_energy) / temp).exp();
}



fn write_out_to_file(file_name: &str, arr: &mut Vec<i32>){
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut result = format!("{}",arr.len());

    for i in arr {
        result = format!("{}\n{}",result,i);
    }

    file.write_all(result.as_bytes()).expect("write failed");

    println!("out written to file" );

}