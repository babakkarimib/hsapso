use std::time::SystemTime;
use rand::Rng;
use dialoguer::Input;
use humantime::{format_rfc3339, format_duration};
use std::fs::File;
use serde_json;

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
    let mut rng = rand::thread_rng();

    let mut harmonies = vec![];
    for _ in 0..harmony_size {
        let mut harmony = Harmony::new(p_num, p_values, t_value, print_map);
        harmony.init();
        harmonies.push(harmony);
    }

    loop {
        let best_harmony = harmonies[0].clone();
        if rng.gen::<f64>() < adjust_rate {
            let harmony = &mut harmonies[rand::thread_rng().gen_range(0..harmony_size)];
            let c1 = 0.7;
            let c2= 0.7;
            let current_particle = harmony.get_random_test_case();
            let r1 = rand::thread_rng().gen::<f64>();
            let r2 = rand::thread_rng().gen::<f64>();
            let w = 0.9;
        
            let mut new_test_case = Vec::with_capacity(p_num);
            for i in 0..p_num {
                let original_value = current_particle[i];
                let cognitive_velocity = w * c1 * r1 * (harmony.test_suite[0][i] as f64 - original_value as f64);
                let social_velocity = w * c2 * r2 * (best_harmony.test_suite[0][i] as f64 - original_value as f64);
                let new_value = (original_value as f64 + cognitive_velocity + social_velocity)
                    .clamp(0.0, p_values as f64 - 1.0) as usize;
                new_test_case.push(new_value);
            }
        
            harmony.add_test_case(new_test_case);
        } else {
            harmonies[harmony_size - 1] = harmonies[rand::thread_rng().gen_range(0..harmony_size)].randomized_clone();
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
    let rep = get_input("Enter repetitions", 5);
    let print_map = get_input("Print coverage map", false);

    let full_coverage = combination(config.p_num, config.t_value) * (config.p_values).pow(config.t_value as u32);
    
    println!();
    serde_json::to_writer(File::create("./config.json").unwrap(), &config).unwrap();
    
    let start = SystemTime::now();
    println!("Start: {}\n", format_rfc3339(start));
    let mut sum =  0;
    let mut best = usize::MAX;
    let mut best_harmony: Harmony = Harmony::new(config.p_num, config.p_values, config.t_value, print_map);
    for i in 0..rep {
        let harmony = hsaga(
            config.p_num,
            config.p_values,
            config.t_value,
            config.harmony_size,
            full_coverage,
            config.adjust_rate,
            print_map
        );
        println!("TEST {} DONE - SIZE: {}", i + 1, harmony.size);
        sum += harmony.size;
        if harmony.size < best {
            best = harmony.size;
            best_harmony = harmony;
        }
    }
    
    let end = SystemTime::now();
    let duration = format_duration(end.duration_since(start).unwrap()).to_string();
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
    println!("Average Size: {}", sum as f64 / rep as f64);
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
