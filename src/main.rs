use std::time::SystemTime;
use rand::Rng;
use dialoguer::Input;
use humantime::{format_rfc3339, format_duration};
use std::fs::File;
use serde_json;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

mod harmony;
mod config;
use harmony::Harmony;
use config::Config;

fn hsaga(
    p_num: usize,
    p_values: usize,
    t_value: usize,
    harmony_size: usize,
    full_coverage: usize,
    adjust_rate: f64,
    print_map: bool
) -> Harmony {
    let mut harmonies = vec![];
    for _ in 0..harmony_size {
        let mut harmony = Harmony::new(p_num, p_values, t_value, print_map);
        harmony.init();
        harmonies.push(harmony);
    }

    loop {
        let best_harmony = harmonies[0].clone();
        if rand::rng().random::<f64>() < adjust_rate {
            let harmony = &mut harmonies[rand::rng().random_range(0..harmony_size)];
            let best_particle = best_harmony.test_suite[best_harmony.size - 1].clone();

            let new_test_case = harmony.pso(best_particle);
        
            harmony.add_test_case(new_test_case);
        } else {
            let random_harmony = harmonies[rand::rng().random_range(0..harmony_size)].randomized_clone();
            if random_harmony < harmonies[harmony_size - 1] {
                harmonies[harmony_size - 1] = random_harmony;
            } 
        }
        
        harmonies.sort();

        if harmonies[0].fitness == full_coverage as usize {
            return harmonies[0].clone();
        }
    }
}

fn get_input<T>(prompt: &str, default: T) -> T
where
    T: std::str::FromStr + std::string::ToString,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let input_str = Input::<String>::new()
        .with_prompt(prompt)
        .default(default.to_string())
        .interact()
        .expect("Failed to read input");

    input_str.parse().expect("Failed to parse input")
}

fn main() {
    let mut config: Config = serde_json::from_reader(&File::open("./config.json").unwrap()).unwrap();
    println!("------------------------------------------------------------------------------");
    println!("Minimum Test Suite Search using Harmony Search Algorithm and PSO Algorithm\nCA (N; t_value, p_values.pow(p_num)):");
    println!("-------------------------------------");
    config.p_num = get_input("Enter p_num", config.p_num);
    config.p_values = get_input("Enter p_values", config.p_values);
    config.t_value = get_input("Enter t_value", config.t_value);
    config.harmony_size = get_input("Enter harmony_size", config.harmony_size);
    config.adjust_rate = get_input("Enter adjust_rate", config.adjust_rate);
    let rep = get_input("Enter repetitions", 11);
    let print_map = get_input("Print coverage map", false);

    let full_coverage = combination(config.p_num, config.t_value) * (config.p_values).pow(config.t_value as u32);
    
    println!();
    serde_json::to_writer(File::create("./config.json").unwrap(), &config).unwrap();
    
    let best_harmony = Arc::new(Mutex::new(Harmony::new(config.p_num, config.p_values, config.t_value, print_map)));
    let best = Arc::new(Mutex::new(i32::MAX));
    let sum = Arc::new(Mutex::new(0));
    
    let start = SystemTime::now();
    println!("Start: {}\n", format_rfc3339(start));
    
    (0..rep).into_par_iter().for_each(|i| {
        let harmony = hsaga(
            config.p_num,
            config.p_values,
            config.t_value,
            config.harmony_size,
            full_coverage,
            config.adjust_rate,
            print_map,
        );
        println!("TEST {} DONE - SIZE: {}", i + 1, harmony.size);
    
        let mut sum_lock = sum.lock().unwrap();
        *sum_lock += harmony.size;
        drop(sum_lock);
    
        let mut best_lock = best.lock().unwrap();
        if harmony.size < (*best_lock).try_into().unwrap() {
            *best_lock = harmony.size as i32;
            *best_harmony.lock().unwrap() = harmony;
        }
    });
    let end = SystemTime::now();
    let duration = format_duration(end.duration_since(start).unwrap()).to_string();
    let avg_size = *sum.lock().unwrap() as f64 / rep as f64;
    let best_harmony = best_harmony.lock().unwrap();

    println!("\nBest Harmony:");
    for (i, (h, w)) in best_harmony.test_suite.iter().zip(best_harmony.weight_list.iter()).enumerate() {
        println!("{:2}: {:?}, weight: {}", i + 1, h, w);
    }
    if print_map {
        println!("{}:", "\nCoverage Map");
        for row in best_harmony.ca_map.iter() {
            println!("{:?}", row);
        }
    }
    println!();
    println!("Best Size: {}", best_harmony.size);
    println!("Average Size: {}", avg_size);
    println!("Total Fitness: {}", best_harmony.fitness);
    println!("Full Coverage Fitness: {}", full_coverage);
    println!("Exhaustive: {}\n", config.p_values.pow(config.p_num as u32));
    println!("Duration: {}\n", &duration[..=duration.find("ms").unwrap() + 1]);
}

fn combination(n: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }
    (n * combination(n - 1, k - 1)) / k
}
