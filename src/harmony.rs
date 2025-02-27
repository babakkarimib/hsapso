use itertools::Itertools;
use rand::Rng;
#[derive(Clone, Eq, Ord)]
pub struct Harmony {
    p_num: usize,
    p_values: usize,
    t_value: usize,
    comb_indices: Vec<Vec<usize>>,
    pub ca_map: Vec<Vec<usize>>,
    pub test_suite: Vec<Vec<usize>>,
    pub weight_list: Vec<usize>,
    pub fitness: usize,
    pub size: usize,
    pub print_map: bool
}

impl Harmony {
    pub fn new(p_num: usize, p_values: usize, t_value: usize, print_map: bool) -> Self {
        let (comb_indices, ca_map) = Harmony::cover_map_matrix(p_num, p_values, t_value);
        Harmony { p_num, p_values, t_value, comb_indices, ca_map: ca_map.to_vec(), test_suite: Vec::new(), weight_list: Vec::new(), fitness: 0, size: 0, print_map }
    }

    pub fn init(&mut self) {
        let test_case = self.create_test_case();

        self.weight_list = vec![self.check_test_case(&test_case, true)];
        self.fitness = self.weight_list[0];

        self.size = 1;

        self.test_suite.push(test_case);
    }

    fn create_test_case(&self) -> Vec<usize> {
        (0..self.p_num)
            .map(|_| rand::rng().random_range(0..self.p_values))
            .collect::<Vec<usize>>()
    }

    pub fn get_random_test_case(&self) -> Vec<usize> {
        self.test_suite[rand::rng().random_range(0..self.size)].clone()
    }

    pub fn add_test_case(&mut self, test_case: Vec<usize>) {
        if self.check_against(&test_case) == 0 {
            return;
        }
        let weight = self.check_test_case(&test_case, true);
        self.fitness += weight;
        self.weight_list.push(weight);
        self.size += 1;
        self.test_suite.push(test_case);
    }

    pub fn check_against(&mut self, test_case: &Vec<usize>) -> usize {
        self.check_test_case(test_case, false)
    }
    
    fn check_test_case(&mut self, test_case: &Vec<usize>, mark: bool) -> usize {
        let mut weight = 0;

        self.ca_map.iter_mut().enumerate().for_each(|(i, ca_row)| {
            let test = self.comb_indices[i]
                .iter()
                .map(|&index| test_case[index].to_string())
                .collect::<String>();
            let decimal_number = usize::from_str_radix(&test, self.p_values as u32).unwrap() as usize;
            
            if ca_row[decimal_number] == 0 {
                if mark {
                    ca_row[decimal_number] = if self.print_map {self.size as usize + 1} else {1};
                }
                weight += 1;
            }
        });
        weight
    }

    pub fn randomized_clone(&mut self) -> Harmony {
        let index = rand::rng().random_range(0..self.size);
        let mut copy = Harmony::new(self.p_num, self.p_values, self.t_value, self.print_map);

        for i in 0..self.size {
            copy.add_test_case(if i == index {self.create_test_case()} else {self.test_suite[i].to_vec()});
        }
        copy
    }

    pub fn pso(&self, best_particle: Vec<usize>) -> Vec<usize> {
        let current_particle = self.get_random_test_case();
        let last_particle = self.test_suite[self.size - 1].clone();

        let c1 = 0.7;
        let c2= 0.7;
        let r1 = rand::rng().random::<f64>();
        let r2 = rand::rng().random::<f64>();
        let w = 0.9;
    
        let mut new_test_case = Vec::with_capacity(self.p_num);
        for i in 0..self.p_num {
            let original_value = current_particle[i];
            let cognitive_velocity = w * c1 * r1 * (last_particle[i] as f64 - original_value as f64);
            let social_velocity = w * c2 * r2 * (best_particle[i] as f64 - original_value as f64);
            let new_value = (original_value as f64 + cognitive_velocity + social_velocity)
                .clamp(0.0, self.p_values as f64 - 1.0) as usize;
            new_test_case.push(new_value);
        }

        new_test_case
    }

    fn cover_map_matrix(p_num: usize, p_values: usize, t_value: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
        if p_num < 3 {
            panic!("p_num must be bigger than or equal to 3.");
        }
    
        if t_value < 2 || t_value >= p_num {
            panic!("'t_value = {}' parameter must be greater than or equal to 2 and less than 'p_num = {}'.", t_value, p_num);
        }
    
        let comb_indices = (0..p_num).combinations(t_value).collect::<Vec<Vec<usize>>>();
        let ca_map = vec![vec![0; (p_values as usize).pow(t_value as u32)]; comb_indices.len()];
    
        (comb_indices, ca_map)
    }
}

impl PartialEq for Harmony {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness && self.size == other.size
    }
}

impl PartialOrd for Harmony {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match other.fitness.cmp(&self.fitness) {
            std::cmp::Ordering::Equal => self.size.cmp(&other.size),
            other => other,
        })
    }
}
