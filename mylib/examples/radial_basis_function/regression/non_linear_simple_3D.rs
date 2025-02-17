#[allow(unused_imports)]
use mylib::{RadialBasisFunctionNetwork, init_rbf, train_rbf_regression, predict_rbf_regression, free_rbf};

fn main() {
    let x: Vec<Vec<f32>> = vec![
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0],
        vec![0.0, 0.0],
    ];
    let y: Vec<f32> = vec![
        2.0,
        1.0,
        -2.0,
        -1.0
    ];
    let data_size = y.len();

    let x_flaten: Vec<f32> = x.clone().into_iter().flatten().collect::<Vec<f32>>();
    let x_ptr: *const f32 = Vec::leak(x_flaten.clone()).as_ptr();
    let y_ptr: *const f32 = Vec::leak(y.clone()).as_ptr();

    let input_dim = 2;
    let cluster_num = 4;
    let gamma = 1.0;

    let rbf_model: *mut RadialBasisFunctionNetwork = init_rbf(input_dim, cluster_num, gamma);
    train_rbf_regression(rbf_model, x_ptr as *mut f32, y_ptr as *mut f32, input_dim, data_size as i32);

    println!("");
    println!("\n Non Linear Simple 3D : RBF Model : OK");
    println!("");
    for i in 0..data_size {
        let input_ptr: *mut f32 = Vec::leak(x[i].clone()).as_mut_ptr();
        let output = predict_rbf_regression(rbf_model, input_ptr);
        println!("X: {:?}, Y: {:?} ---> RBF model: {:?}", x[i], y[i], output);
    }
    println!("");

    free_rbf(rbf_model);
}