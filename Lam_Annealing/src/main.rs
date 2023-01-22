use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::{self, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let file_name = "../data.txt";

    let data = read_from_file(file_name);

    println!("{}", calculate_energy(&lam_annealing(data, 10_000_000)))
}

fn read_from_file(file_name: &str) -> Vec<(i32, i32)> {
    let path = Path::new(file_name);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if index == 0 {
            continue;
        }

        println!("{}",line);
        let numbers = line
            .split_whitespace()
            .map(|x| { println!("{}",x); x.trim_end().parse::<f32>().unwrap() as i32} )
            .collect::<Vec<i32>>();
            
        result.push((numbers[1],numbers[2]));
    }

    return result;
}

fn calculate_distance(x_1: (i32, i32), x_2: (i32, i32)) -> f64 {
    let (x1, y1) = x_1;
    let (x2, y2) = x_2;
    return (f64::powf((x2 - x1).into(), 2.0) + f64::powf((y2 - y1).into(), 2.0)).sqrt();
}

fn lam_annealing(points: Vec<(i32, i32)>, iters: i32) -> Vec<(i32, i32)> {
    //Init state
    let mut rng = rand::thread_rng();

    let mut current = points.clone();

    current.shuffle(&mut thread_rng());

    let mut best = current.clone();

    let mut accept_rate = 0.5;
    

    let mut T = 0.5;

    let mut m_0 = 0.56;

    let m_1: f64 = 560.0_f64.powf(-1.0 / (0.15 * iters as f64));

    let m_2: f64 = 440.0_f64.powf(-1.0 / (iters as f64 -  0.35 * iters as f64));

    let mut lam_rate = 0.0;

    for i in 1..iters {
        let mut selected = current.clone();
        selected.shuffle(&mut thread_rng());

        let selected_energy = calculate_energy(&selected);

        let current_energy = calculate_energy(&current);

        if selected_energy < current_energy
            || rng.gen_range(0.0..1.0) < acceptance(current_energy, selected_energy, T)
        {
            current = selected.clone();

            accept_rate = (0.998 * accept_rate) + 0.002;

            if calculate_energy(&best) > calculate_energy(&current){
                best = current.clone();
            }
        } else {
            accept_rate = 0.998 * accept_rate;
        }

        if i as f32 - (0.15 * iters as f32) > 0.0 {
            m_0 = m_0 * m_1;
            lam_rate = 0.44 + m_0;
        } else {
            if i as f32 - (0.65 * iters as f32) < 0.0 {
                lam_rate = lam_rate * m_2;
            } else {
                lam_rate = 0.44
            }
        }

        if accept_rate > lam_rate {
            T = 0.999 * T;
        } else {
            T = T / 0.999;
        }
    }

    return best;
}

fn acceptance(energy: f64, new_energy: f64, temp: f64) -> f64 {

    return ((energy - new_energy) / temp).exp();
}

fn calculate_energy(path: &Vec<(i32, i32)>) -> f64 {
    let size = path.iter().len();

    let mut distance = calculate_distance(*path.get(0).unwrap(), *path.get(size - 1).unwrap());

    for i in 0..size - 2 {
        distance += calculate_distance(*path.get(i).unwrap(), *path.get(i + 1).unwrap());
    }

    return distance;
}
