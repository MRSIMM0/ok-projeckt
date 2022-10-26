use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::cmp::Ordering;
use std::time::{Instant};
fn main(){

    let file_name = "./viz/da.txt";

    let mut number:usize = 100;

    let mut values = generate(&mut number);
  
    write_vector_to_file(file_name,&mut values);
    
    let mut data = read_from_file(file_name);

    let output_file = "./viz/path.txt";

    let now = Instant::now();

    let mut out = heuristic(&mut data);

    let time = now.elapsed().as_secs_f64();

    write_out_to_file(output_file, &mut out);


    println!("{}", time);

}

fn generate(n:&mut usize) -> Vec<(i32,i32)>{

    let mut arr=Vec::new();

    let mut rng = rand::thread_rng();

    for _x in 0..*n {
        arr.push((rng.gen_range(0..2_000),rng.gen_range(0..2_000)));
    }

   return arr;
}

fn write_vector_to_file(file_name: &str, arr: &mut Vec<(i32,i32)>){

    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut result = format!("{}",arr.len());

    for i in arr {
        let (x,y) = i;
        result = format!("{}\n{} {}",result,x,y);
    }
    file.write_all(result.as_bytes()).expect("write failed");

    println!("data written to file" );

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



fn heuristic(arr: &mut Vec<(i32,i32)>) -> Vec::<i32>{

    let mut  visisted = Vec::<i32>::new();

    recur(arr,&mut visisted,0);

    return visisted;
}


fn recur(arr: &mut Vec<(i32,i32)>, visisted: &mut Vec<i32>, current: i32){
    visisted.push(current);

    let current_element = arr[current as usize];


    if arr.iter().len() == visisted.iter().len(){
        return ;
    }
   
    let distances = arr.iter()
    .map(|x| calculate_distance(current_element,*x) )
    .collect::<Vec<f64>>();

    let mut sorted = distances.clone();

    sorted.sort_by(cmp_f64);


    let mut min = 0;


    while visisted.contains(&(distances.iter()
    .position(|&x| x == sorted[min]).unwrap() as i32) ) {
        min+=1;
    }

    let curr =distances.iter()
    .position(|&x| x == sorted[min]).unwrap() as i32;

    recur(arr,visisted,curr);
}

fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

fn calculate_distance(x_1: (i32,i32), x_2: (i32,i32)) -> f64 {
    let  (x1,y1) = x_1;
    let (x2,y2) = x_2;
    return(f64::powf((x2-x1).into(),2.0) + f64::powf((y2-y1).into(),2.0)).sqrt();
}