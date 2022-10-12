use std::vec::Vec;
use tch;
use tch::{nn, Kind, Tensor, Device};
use tch::{nn::Module, nn::OptimizerConfig};
use csv::{Reader, StringRecord};

use rand;
use rand::thread_rng;
use rand::seq::SliceRandom;

use std::error::Error;

static FEATURE_DIM: i64 = 4;
static HIDDEN_NODES: i64 = 10;
static LABELS: i64 = 3;


#[derive(Debug)]
struct Net {
    fc1: nn::Linear,
    fc2: nn::Linear,
}

impl Net {
    fn new(vs: &nn::Path) -> Net {
        let fc1 = nn::linear(vs, FEATURE_DIM, HIDDEN_NODES, Default::default());
        let fc2 = nn::linear(vs, HIDDEN_NODES, LABELS, Default::default());
        Net {fc1, fc2}
    }
}

impl Module for Net {
    fn forward(&self, xs: &Tensor) -> Tensor {
        xs.apply(&self.fc1).relu().apply(&self.fc2)
    }
}

#[derive(Debug, Clone)]
pub struct Flower {
    sepal_length: f32,
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
    species: String,
}

impl Flower {
    pub fn into_feature_vector(&self) -> Vec<f32> {
        vec![self.sepal_length, self.sepal_width, self.petal_length, self.petal_width]
    }

    pub fn into_labels(&self) -> f32 {
        match self.species.as_str() {
            "setosa" => 0.,
            "versicolor" => 1.,
            "virginica" => 2.,
            l => panic!("Not able to parse the target. Some other target got passed. {:?}", l),            
        }
    }
}

pub fn load_csv() -> Vec<Flower> {
    let rdr = Reader::from_path("dataset/iris-dataset.csv");
    let mut data: Vec<Flower> = Vec::new();

    for result in rdr.expect("REASON").records() {
        let record: StringRecord = result.expect("REASON");
        let sepal_length: f32 = record[0].parse().unwrap();
        let sepal_width: f32 = record[1].parse().unwrap();
        let petal_length: f32 = record[2].parse().unwrap();
        let petal_width: f32 = record[3].parse().unwrap();
        let species: String = record[4].parse().unwrap();

        let flower = Flower {
            sepal_length,
            sepal_width,
            petal_length,
            petal_width,
            species
        };
        data.push(flower);
    }
    data.shuffle(&mut thread_rng());
    data
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = load_csv();

    let test_size: f64 = 0.5;
    let test_size: f64 = data.len() as f64 * test_size;
    let test_size = test_size.round() as usize;
    
    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    let flower_x_train: Vec<f64> = train_data.iter().flat_map(|r| r.into_feature_vector()).map(|x| x as f64).collect();
    let flower_y_train: Vec<f64> = train_data.iter().map(|r| r.into_labels()).map(|x| x as f64).collect();

    let flower_x_test: Vec<f64> = test_data.iter().flat_map(|r| r.into_feature_vector()).map(|x| x as f64).collect();
    let flower_y_test: Vec<f64> = test_data.iter().map(|r| r.into_labels()).map(|x| x as f64).collect();

    let flower_x_train = Tensor::of_slice(flower_x_train.as_slice()).to_kind(Kind::Float);
    let flower_y_train = Tensor::of_slice(flower_y_train.as_slice()).to_kind(Kind::Int64);
    let flower_x_test = Tensor::of_slice(flower_x_test.as_slice()).to_kind(Kind::Float);
    let flower_y_test = Tensor::of_slice(flower_y_test.as_slice()).to_kind(Kind::Int64);

    let train_size = train_size as i64;
    let test_size = test_size as i64;
    let flower_x_train = flower_x_train.view([train_size, FEATURE_DIM]);
    let flower_x_test = flower_x_test.view([test_size, FEATURE_DIM]);
    let flower_y_train = flower_y_train.view([train_size]);
    let flower_y_test = flower_y_test.view([test_size]);

    let vs = nn::VarStore::new(Device::Cpu);
    let net = Net::new(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-3)?;
    for epoch in 1..200 {
        let loss = net
            .forward(&flower_x_train)
            .cross_entropy_for_logits(&flower_y_train);
        opt.backward_step(&loss);
        let test_accuracy = net
            .forward(&flower_x_test)
            .accuracy_for_logits(&flower_y_test);
        println!(
            "epoch: {:4} train loss: {:8.5} test acc: {:5.2}%",
            epoch,
            f64::from(&loss),
            100. * f64::from(&test_accuracy),
        );
    };
    Ok(())
}