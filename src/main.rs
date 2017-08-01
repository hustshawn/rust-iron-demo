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
    response: String
}
// pick a string at random
fn pick_resposne(name: String) -> String {

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

fn handler(req: &mut Request) -> IronResult<Response> {
    let response = JsonResponse { response: pick_resposne("Shawn".to_string()) };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn post_handler(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    println!("{:?}", payload);

    // We're expecting the POST to match the format of our JsonResponse struct
    // eg. { "response": "Brian"}
    let incoming: JsonResponse = json::decode(&payload).unwrap();

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse { response: pick_resposne(incoming.response) };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", post_handler, "post_name");

    Iron::new(router).http("localhost:8088").unwrap();
}
