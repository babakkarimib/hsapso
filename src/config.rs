use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub p_num: usize,
    pub p_values: usize,
    pub t_value: usize,
    pub harmony_size: usize,
    pub adjust_rate: f64
}
