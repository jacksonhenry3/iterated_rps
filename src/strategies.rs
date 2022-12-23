use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{collections::HashMap, ops::Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Strategy {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Default for Strategy {
    fn default() -> Self {
        Strategy::Rock
    }
}

pub fn weighted_boltzman(beta: f32, ns: &HashMap<Strategy, f64>) -> Strategy {
    let mut rng = rand::thread_rng();
    let mut sum = 0.0;
    let mut weights = HashMap::new();
    for (k, v) in ns {
        let w = (v * beta as f64).exp();
        weights.insert(k, w);
        sum += w;
    }
    //if ns has only one strategy, return that strategy
    if weights.len() == 1 || sum == 0.0{
        return **weights.keys().next().unwrap();
    }

    //if one weight is much larger than the otehrs return that strategy
    let max = weights.values().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min = weights.values().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    if max / min > 100.0 {
        //get the inde of the largest
        let index = weights.values().position(|x| x == max).unwrap();
        return **weights.keys().nth(index).unwrap();
    }

    let mut r = rng.gen_range(0.0..sum);
    for (k, v) in weights {
        r -= v;
        if r <= 0.0 {
            return *k;
        }
    }
    panic!("weighted_boltzman failed")
}

//allow for random strategy generation
impl Distribution<Strategy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Strategy {
        match rng.gen_range(0..=2) {
            0 => Strategy::Rock,
            1 => Strategy::Paper,
            _ => Strategy::Scissors,
        }
    }
}

pub struct PayoffMatrix {
    pub matrix: [[f32; 3]; 3],
}

impl Index<(Strategy, Strategy)> for PayoffMatrix {
    type Output = f32;

    fn index(&self, index: (Strategy, Strategy)) -> &f32 {
        &self.matrix[index.0 as usize][index.1 as usize]
    }
}
