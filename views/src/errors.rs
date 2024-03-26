// use serde::de::value::Error as DeserializeError;
// use std::error::Error;

// fn find_error_source<'a, T>(err_in: &'a (dyn Error + 'static)) -> Option<&'a T>
// where
//     T: Error + Clone + 'static,
// {
//     if let Some(err) = err_in.downcast_ref::<T>() {
//         println!("successfully casted {:?}", err);
//         Some(err)
//     } else if let Some(source) = err_in.source() {
//         println!("recursing into {:?}", err_in);
//         find_error_source(source)
//     } else {
//         None
//     }
// }

// pub fn extract_serde_error_list(err: &(dyn Error + 'static)) -> Vec<i32> {
//     // let err_val = err.into();
//     let serde_error = find_error_source::<DeserializeError>(err).unwrap();
//     println!("run stuff {:?}", serde_error);
//     vec![]
// }
