// extern crate iron;
// extern crate time;

// use iron::prelude::*;
// use iron::{BeforeMiddleware, AfterMiddleware, typemap};
// use time::precise_time_ns;

// struct ResponseTime;

// impl typemap::Key for ResponseTime {
//     type Value = u64;
// }

// impl BeforeMiddleware for ResponseTime {
//     fn before(&self, req: &mut Request) -> IronResult<()> {
//         req.extensions.insert::<ResponseTime>(precise_time_ns());
//         Ok(())
//     }
// }

// impl AfterMiddleware for ResponseTime {
//     fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
//         let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
//         println!("Request took: {} ms", (delta as f64) / 100000.0);
//         Ok(res)
//     }
// }

// fn hello_world(_: &mut Request) -> IronResult<Response> {
//     Ok(Response::with((iron::status::Ok, "Hellow world!")))
// }

// fn main() {
//     let mut chain = Chain::new(hello_world);
//     chain.link_before(ResponseTime);
//     chain.link_after(ResponseTime);
//     Iron::new(chain).http("localhost:8088").unwrap();
// }

// main.rs
extern crate iron;
extern crate rand;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;              // use random number generator
use router::Router;
use rustc_serialize::json;
use std::io::Read;


#[derive(RustcEncodable, RustcDecodable)]
struct JsonResponse {
    response: String,
    success: bool,
    error_message: String
}

impl JsonResponse {
    fn success(response: String) -> Self {
        JsonResponse { response: response, success: true, error_message: "".to_string() }
    }

    fn error(msg: String) -> Self {
        JsonResponse { response: "".to_string(), success: false, error_message: msg}
    }
}

#[derive(RustcDecodable)]
struct JsonRequest {
    name: String
}

// pick a string at random
fn pick_response(name: String) -> String {

    // generate a number between 1 to 3
    let num = rand::thread_rng().gen_range(1, 4);

    // match the random number and pick a random string
    let response = match num {
        1 =>    format!("Hello {}!", name),
        2 =>    format!("Did you see that ludicrous display last night, {}?", name),
        3 =>    format!("Nice weather for ducks, isn't it {}?", name),
        _ =>    format!("")
    };

    // return the string
    response.to_string()
}

pub fn get_name(name: String) -> String {
    pick_response(name)
}

fn handler(req: &mut Request) -> IronResult<Response> {
  let response = JsonResponse::success(get_name("Brian".to_string()));
  let out = json::encode(&response).expect("Failed to encode response");

  let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
  Ok(Response::with((content_type, status::Ok, out)))
}

fn post_handler(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).expect("Failed to read request body");
    println!("{:?}", payload);

    let out = match json::decode(&payload) {
        Err(e) => {
            let response = JsonResponse::error(format!("Error parsing JSON: {:?}", e));
            json::encode(&response).ok().expect("Error encoding response")
        },
        Ok(incoming) => {
            // Rust needs to know the type of incoming before we can use it in the get_name,
            // so set to a variable with a type
            let converted: JsonRequest = incoming;
            let response = JsonResponse::success(get_name(converted.name));
            json::encode(&response).expect("Error encoding response")
        }
    };

    // print out the JSON as usual
    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, out)))
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", post_handler, "post_name");

    println!("Listening on localhost:8088");
    Iron::new(router).http("localhost:8088").ok();
}
