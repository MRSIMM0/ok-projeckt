
use std::fs::File;
use std::path::Path;
use std::io::Write;

use rand::Rng;

use std::process::Command;

fn main() {

    // let mut g_temp: f64 = 0.0;

    // let mut g_cool: f64 =0.0;

    // let mut g_iter: i32 = 0;
    
    // let mut g_err = -9999.9;



    //     let baseline = heuristic();

    //     for x in (5000..20000).step_by(1500) {
            
        
    //         let temperature:f64 = x as f64;

    //         for y in 1..10{
    //             let cool_rate:f64 = 0.001 * y as f64;
    //             for t in (500..5000).step_by(100){
                    
                 
    //                 let iterations:i32 = t;

    //                 let mut average = Vec::new(); 
                   
    //                 for p in 1..20{
    //                     let ex = metaheurystyka(temperature, cool_rate, iterations);
    //                     average.push(ex);
    //                 }
                    
    //                 let avg : f64 = average.iter().sum::<f64>() as f64 / average.len() as f64;
    //                 let err = (baseline - avg)/baseline;

    //                 if err>g_err{
    //                     g_err = err;
    //                     g_temp =temperature;
    //                     g_iter = iterations;
    //                     g_cool = cool_rate;
    //                     println!("temp: {}\ncool_rate: {}\niter: {}\navg: {}\nheur: {}\nerr: {}% \n",temperature,iterations,cool_rate,avg,baseline,err*100.0);

    //                 }

    //             }
    //         }
    //     }
    }




fn metaheurystyka(temperature:f64,cool_rate:f64,iterations:i32) -> f64{
    let dist = Command::new("../metaheurystyka/target/release/ok-sym")
    .arg("../data.txt")
    .args([ format!("{}",temperature),format!("{}",cool_rate),format!("{}",iterations)])
    .output()
    .expect("command failed to start");

    let str = std::str::from_utf8(&dist.stdout).expect("invalid utf-8 sequence");

    let f: f64 = str.parse().unwrap();

    return f;
}

fn heuristic() -> f64{
    let dist = Command::new("../herystyka/target/release/project")
    .arg("../data.txt")
    .output()
    .expect("command failed to start");

    let str = std::str::from_utf8(&dist.stdout).expect("invalid utf-8 sequence");

    let f: f64 = str.parse().unwrap();

    return f;

}

fn generate(n: usize) -> Vec<(i32, i32)> {
    let mut arr = Vec::new();

    let mut rng = rand::thread_rng();

    for _x in 0..n {
        arr.push((rng.gen_range(0..2_000), rng.gen_range(0..2_000)));
    }

    return arr;
}

fn write_vector_to_file(file_name: &str, arr: &mut Vec<(i32, i32)>) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut result = format!("{}", arr.len());

    for i in arr {
        let (x, y) = i;
        result = format!("{}\n{} {}", result, x, y);
    }
    file.write_all(result.as_bytes()).expect("write failed");

    println!("data written to file");
}
