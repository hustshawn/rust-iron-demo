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
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;              // use random number generator
use rustc_serialize::json;


#[derive(RustcEncodable)]
struct JsonResponse {
    response: String
}
// pick a string at random
fn pick_resposne() -> String {

    // generate a number between 1 to 3
    let num = rand::thread_rng().gen_range(1, 4);

    // match the random number and pick a random string
    let response = match num {
        1 =>    "Hello World!",
        2 =>    "Did you see that ludicrous display last night?",
        3 =>    "Nice weather for ducks",
        _ =>    ""
    };

    // return the string
    response.to_string()
}

fn main() {

    // check it's working - this will appear in your terminal
    println!("{:?}", pick_resposne());

    Iron::new(|_: &mut Request| {
        let content_type = "application/json".parse::<Mime>().unwrap();

        // create the response
        let response = JsonResponse { response: pick_resposne() };

        // convert the response struct to JSON
        let out = json::encode(&response).unwrap();

        Ok(Response::with((content_type, status::Ok, out)))
    }).http("localhost:8088").unwrap();
}
