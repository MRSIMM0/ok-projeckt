use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::{self, Rng};
use std::env;
use std::io::{self, Read};
use std::path::Path;
use std::fs::File;
use std::cmp::Ordering;
use std::io::prelude::*;
use std::io::BufReader;
use std::result;


fn main() {
    let args: Vec<String> = env::args().collect();


    let mut temp = 18_200.0;
    let mut cool_rate = 0.001;
    let mut iters = 1000;

    if args.len() > 3 {
        temp = args[1].parse().unwrap();
        cool_rate = args[2].parse().unwrap();
        iters = args[3].parse().unwrap();
    }

    let stdin = io::stdin();
    let mut stdin = stdin.lock(); 

    let mut line = String::new();

    let mut result: Vec<(i32, i32)> = Vec::new();

    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 {
            break;
        }
     
        for (index, l) in line.lines().enumerate() {
    
            if index == 0 {
                continue;
            }
            let x = l
                .split_whitespace()
                .map(|x| x.trim_end().parse::<f32>().unwrap() as i32)
                .collect::<Vec<i32>>();
            result.push((x[1], x[2]));
        }
    

        line.clear();
    }

    let best = sim_ann(temp, cool_rate, result.clone(), iters);

    // let mut res: Vec<i32> = Vec::new();

    // best.iter().for_each(|&e|  {res.push( result.iter().position(|&x| x == e).unwrap().try_into().unwrap());});
    // print!("{:?}",res);
    

    // write_out_to_file("../path.txt", &mut res);
    print!("{:?}", eval(&best));
}


fn calculate_distance(x_1: (i32, i32), x_2: (i32, i32)) -> f64 {
    let (x1, y1) = x_1;
    let (x2, y2) = x_2;
    return (f64::powf((x2 - x1).into(), 2.0) + f64::powf((y2 - y1).into(), 2.0)).sqrt();
}

fn sim_ann(mut temp: f64, cool_rate: f64, points: Vec<(i32, i32)>, iters: i32) -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();

    let mut current = points.clone();

    let t0 = temp.clone();

    current.shuffle(&mut thread_rng());

    let mut best = current.clone();

    let points_len = points.iter().len();

    let mut iter = 0;

    while temp > 1.0 && iter<iters {
 
            let mut new_solution = current.clone();

            new_solution.swap(rng.gen_range(0..points_len), rng.gen_range(0..points_len));

            let current_energy = eval(&new_solution);
            let new_energy = eval(&current);

            if eval(&current) <= eval(&best) {
                best = current.clone();
            } else {
                if acceptance(new_energy, current_energy, temp) > rng.gen_range(0.0..1.0) {
                    current = new_solution.clone();
                }
            }
        
    
        temp = t0 * cool_rate.powf(iter.into());
        iter+=1;
    }

    return best;
}



fn eval(path: &Vec<(i32, i32)>) -> f64 {
    let size = path.iter().len();

    let mut distance = calculate_distance(*path.get(0).unwrap(), *path.get(size - 1).unwrap());

    for i in 0..size - 2 {
        distance += calculate_distance(*path.get(i).unwrap(), *path.get(i + 1).unwrap());
    }

    return distance;
}

fn acceptance(energy: f64, new_energy: f64, temp: f64) -> f64 {
    if new_energy < energy {
        return 1.0;
    }
    return -1.0/(1.0+ (energy - new_energy) / temp).exp();
}

fn write_out_to_file(file_name: &str, arr: &mut Vec<i32>) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut result = format!("{}", arr.len());

    for i in arr {
        result = format!("{}\n{}", result, i);
    }

    file.write_all(result.as_bytes()).expect("write failed");

    println!("out written to file");
}