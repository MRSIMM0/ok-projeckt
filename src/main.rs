use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;

fn main(){

    let file_name = "data.txt";

    let mut number:usize = 30;

    let mut values = generate(&mut number);
  
    write_vector_to_file(file_name,&mut values);
    
    let data = read_from_file(file_name);

}

fn generate(n:&mut usize) -> Vec<(i32,i32)>{

    let mut arr=Vec::new();
    let mut rng = rand::thread_rng();
    for _x in 0..*n {
        arr.push((rng.gen_range(0..2_000_000),rng.gen_range(0..2_000_000)));
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

    println!("{}",result);
    
    file.write_all(result.as_bytes()).expect("write failed");

    println!("data written to file" );

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