extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    println!("Serving on http://localhost:3000...");
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut resp  = Response::new();
    resp.set_mut(status::Ok);
    resp.set_mut(mime!(Text/Html; Charset=Utf8));
    resp.set_mut(r#"
        <title>gcd</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute</button>
        </form>
    "#);

    Ok(resp)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("error parsing form data: {:?}\n", e));
            return Ok(resp);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            resp.set_mut(status::BadRequest);
            resp.set_mut("can't get n in form");
            return Ok(resp);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(unparsed) {
            Err(e) => {
                resp.set_mut(status::BadRequest);
                resp.set_mut(format!("failed to parse value: {:?}", e));
                return Ok(resp);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    let mut d = numbers[0];
    for n in &numbers[1..] {
        d = gcd(d, *n)
    }

    resp.set_mut(status::Ok);
    resp.set_mut(mime!(Text/Html; Charset=Utf8));
    resp.set_mut(
        format!("the greated common divisor of numbers {:?} is {}", numbers, d)
    );
    Ok(resp)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}