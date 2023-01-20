use crate::dnn::ActivationCache;
use ndarray::prelude::*;
use polars::{prelude::*, export::num::ToPrimitive};
use serde_json::Value;
use std::{collections::HashMap, f32::consts::E};

fn relu(z: f32) -> f32 {
    if z > 0.0 {
        z
    } else {
        0.0
    }
}

pub fn relu_prime(z: f32) -> f32 {
    if z > 0.0 {
        1.0
    } else {
        0.0
    }
}

pub fn sigmoid(z: f32) -> f32 {
    1.0 / (1.0 + E.powf(-z))
}

fn sigmoid_prime(z: f32) -> f32 {
    sigmoid(z) * (1.0 - sigmoid(z))
}

/// returns matrix with sigmoid activation applied to all values
///  # Arguments
///  * `Z` - A matrix   
pub fn sigmoid_activation(z: Array2<f32>) -> (Array2<f32>, ActivationCache) {
    (z.map(|x| sigmoid(*x)), ActivationCache { z })
}

pub fn sigmoid_backward(da: &Array2<f32>, z: Array2<f32>) -> Array2<f32> {
    da * z.map(|x| sigmoid_prime(*x))
}

pub fn relu_backward(da: &Array2<f32>, z: Array2<f32>) -> Array2<f32> {
    da * z.map(|x| relu_prime(*x))
}

pub fn relu_activation(z: Array2<f32>) -> (Array2<f32>, ActivationCache) {
    (z.map(|x| relu(*x)), ActivationCache { z })
}

/**
Loads data from a .csv file to a Polars DataFrame
*/
pub fn load_data_as_dataframe(path: &str) -> (DataFrame, DataFrame) {
    let data = CsvReader::from_path(path).unwrap().finish().unwrap();

    let x_train_data = data.drop("y").unwrap();
    let y_train_data = data.select(["y"]).unwrap();

    (x_train_data, y_train_data)
}

/**
Converts DataFrame to ndarray - Array2<f32>
*/
pub fn array_from_dataframe(dataframe: &DataFrame) -> Array2<f32> {
    dataframe
        .to_ndarray::<Float32Type>()
        .unwrap()
        .reversed_axes()
}

pub fn load_weights_from_json() ->HashMap<String, Array2<f32>>  {
    let text = std::fs::read_to_string("weights.json").unwrap();
    let weights_json: serde_json::Value = serde_json::from_str(&text).unwrap();
    // println!("{}",weights_json["b4"]);

    let mut parameters: HashMap<String, Array2<f32>> = HashMap::new();

    for (key, val) in weights_json.as_object().unwrap() {
        let dims = val["dim"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_i64().unwrap().to_usize().unwrap())
            .collect::<Vec<usize>>();
        let data = val["data"].as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_f64().unwrap().to_f32().unwrap())
        .collect::<Vec<f32>>();

        let matrix = Array2::from_shape_vec((dims[0],dims[1]), data).unwrap();
        parameters.insert(key.to_string(), matrix);

    }
    parameters
}
