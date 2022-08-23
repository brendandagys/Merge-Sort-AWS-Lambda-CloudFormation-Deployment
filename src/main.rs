mod helpers;
mod types;

use crate::{
    helpers::print_time,
    types::{CustomOutput, ParsedRequestBody},
};
use lambda_http::{http::StatusCode, service_fn, Body, Response};
use log;
use simple_logger;

fn merge_sorted_arrays(left: &[i32], right: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);

    let mut merged = Vec::with_capacity(left.len() + right.len());

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1
        } else {
            merged.push(right[j]);
            j += 1
        }
    }

    merged.append(&mut match i < left.len() {
        true => left[i..].to_vec(),
        false => right[j..].to_vec(),
    });

    merged
}

fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() == 1 {
        return arr.to_vec();
    }

    let middle = arr.len() / 2;

    let left = merge_sort(&arr[..middle]);
    let right = merge_sort(&arr[middle..]);

    merge_sorted_arrays(&left, &right)
}

async fn handler(e: lambda_http::Request) -> Result<Response<Body>, lambda_http::Error> {
    println!("{:?}", e);

    match e.body() {
        Body::Text(body) => match serde_json::from_str::<ParsedRequestBody>(body) {
            Ok(body) => {
                println!("NAME: {}", body.name);

                let sorted_numbers = merge_sort(&body.numbers);

                println!("SORTED NUMBERS: {:?}", sorted_numbers);
                print_time("FINISHED!");

                return Ok(Response::builder().status(StatusCode::OK).body(Body::Text(
                    serde_json::to_string(&CustomOutput { sorted_numbers })?,
                ))?);
            }
            Err(e) => {
                println!("ERROR: {:?}", e);

                return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(
                    Body::Text(
                        "Please provide both necessary parameters: `numbers` and `name`.".into(),
                    ),
                )?);
            }
        },
        _ => panic!("The provided body was not a `Text` body!"),
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    simple_logger::init_with_level(log::Level::Info)?;

    lambda_http::run(service_fn(handler)).await?;

    Ok(())
}
