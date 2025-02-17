use std::ffi::CString;
#[allow(unused_imports)]
use mylib::{free_mlp, init_mlp, predict_mlp, save_mlp_model, train_mlp, MultiLayerPerceptron};

fn main() {
    let x: Vec<Vec<f32>> = vec![
        vec![1.0, 1.0],
        vec![2.0, 2.0],
        vec![3.0, 1.0],
    ];
    let y: Vec<f32> = vec![2.0, 3.0, 2.5];

    let data_size = y.len();
    let x_flatten: Vec<f32> = x.clone().into_iter().flatten().collect();
    let y_train: Vec<f32> = y.clone();
    let x_train_ptr: *const f32 = x_flatten.as_ptr();
    let y_train_ptr: *const f32 = y_train.as_ptr();
    let x_test_ptr: *const f32 = x_flatten.as_ptr();
    let y_test_ptr: *const f32 = y_train.as_ptr();
    let npl: Vec<u32> = vec![2, 1];  // 1 entrée, une couche cachée de 2 neurones, 1 sortie
    let mlp: *mut MultiLayerPerceptron = init_mlp(npl.as_ptr(), npl.len() as u32, false);

    let c_log_filename = CString::new("rg_linear_simple_3D").expect("CString::new failed");
    let c_model_filename = CString::new("../models/examples/mlp/classification/linear_simple_3D.json").expect("CString::new failed");

    unsafe {
        let success = train_mlp(
            mlp,
            x_train_ptr,
            y_train_ptr,
            data_size as u32,
            x_test_ptr,
            y_test_ptr,
            data_size as u32,
            0.001,
            10_000,
            c_log_filename.as_ptr(),
            c_model_filename.as_ptr(),
            false,
            false,
            false,
        );

        if success {
            let mut total_mse = 0.0;
            let mut max_error = 0.0;
            for i in 0..data_size {
                let input_ptr: *const f32 = x[i].as_ptr();
                let output: *mut f32 = predict_mlp(mlp, input_ptr);
                if !output.is_null() {
                    let res: Vec<f32> = Vec::from_raw_parts(output, 1, 1);
                    println!("X: {:?}, Y: {:?} ---> MLP model: {:?}", x[i], y[i], res);
                    
                    let error = (res[0] - y[i]).abs();
                    total_mse += error * error;
                    if error > max_error {
                        max_error = error;
                    }
                }
            }
            let mse = total_mse / data_size as f32;
            let rmse = mse.sqrt();
            println!("Mean Squared Error: {:.4}", mse);
            println!("Root Mean Squared Error: {:.4}", rmse);
            println!("Max Absolute Error: {:.4}", max_error);

            // let model_file_name = "model.json";
            // let model_file_name = CString::new(model_file_name).expect("CString::new failed");
            // save_mlp_model(mlp, model_file_name.as_ptr());
        } else {
            println!("Training failed.");
        }
        free_mlp(mlp);
    }
}