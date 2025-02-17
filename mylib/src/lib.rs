/* ********************************************************************************************************* */
/*                                                                                                           */
/*                                                              :::::::::: ::::::::   :::::::: :::::::::::   */
/*   lib.rs                                                    :+:       :+:    :+: :+:    :+:    :+:        */
/*                                                            +:+       +:+        +:+           +:+         */
/*   By: YAHIA ABDCHAFAA Adam, SALHAB Charbel, ELOY Theo     +#++:++#  +#++:++#++ :#:           +#+          */
/*                                                          +#+              +#+ +#+   +#+#    +#+           */
/*   Created: 2024/03/22 19:38:54                          #+#       #+#    #+# #+#    #+#    #+#            */
/*   3IABD1 2023-2024                                     ########## ########   ######## ###########         */
/*                                                                                                           */
/* ********************************************************************************************************* */

pub mod linear_model;
pub mod multilayer_perceptron;
pub mod radial_basis_function_network;
pub mod support_vector_machine;
pub mod load_dataset;

#[allow(unused_imports)]
pub use multilayer_perceptron::{
    MultiLayerPerceptron,
    init_mlp,
    train_mlp,
    predict_mlp,
    free_mlp,
    save_mlp_model,
    loads_mlp_model
};

#[allow(unused_imports)]
pub use linear_model::{
    LinearModel,
    init_linear_model,
    train_linear_model,
    predict_linear_model,
    free_linear_model,
    save_linear_model,
    loads_linear_model
};

#[allow(unused_imports)]
pub use radial_basis_function_network::{
    RadialBasisFunctionNetwork,
    init_rbf,
    train_rbf_regression,
    train_rbf_rosenblatt,
    predict_rbf_regression,
    predict_rbf_classification,
    free_rbf,
    save_rbf_model,
    rbf_to_json
};

#[allow(unused_imports)]
pub use support_vector_machine::{
    SVMModel,
    init_svm,
    train_svm,
    predict_svm,
    free_svm
};

#[allow(unused_imports)]
pub use load_dataset::{
    loads_mlp_dataset,
    loads_ml_dataset,
    create_serialized_mlp_dataset,
    create_serialized_ml_dataset,
    loads_serialized_ml_dataset,
    loads_serialized_mlp_dataset,
    image_resize_vec,
};