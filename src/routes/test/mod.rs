// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use amiwo::FormHashMap;
use amiwo::ResponseJSON;

// =======================================================================
// ROUTES DEFINITIONS
// =======================================================================
#[get("/back/hello")]
fn back_hello() -> ResponseJSON {
    ResponseJSON::ok()
        .data(json!({
            "message": "Hello world"
        }))
}

#[post("/back/echo", data="<params>")]
fn back_echo(params: FormHashMap) -> ResponseJSON {
    ResponseJSON::ok()
        .data(json!({
            "message": "Hello world",
            "echo": format!("I received from you > {}", params["echo"])
        }))
}

// =======================================================================
// TEST ROUTES
// =======================================================================
#[cfg(test)]
mod test {
    #![allow(unmounted_route)]

    use amiwo::ResponseJSON;
    use amiwo::IsResponseJSON;

    use hyper;
    use hyper::client::Client;
    
    use rocket;
    use rocket::http::{ ContentType, Status, Method };
    use rocket::testing::MockRequest;
    
    #[test]
    fn test_back_hello() {
        let rocket = rocket::ignite()
            .mount("/", routes![super::back_hello]);
        
        let mut req = MockRequest::new(Method::Get, "/back/hello");
        let mut response = req.dispatch_with(&rocket);

        assert_eq!(response.status(), Status::Ok);
        let rjson = response.body()
            .map_or(
                ResponseJSON::error(),
                |body| ResponseJSON::from_reader(body.into_inner()).unwrap()
            );

        assert_eq!(rjson.is_ok_json(), true);
        assert_eq!(rjson.http_code, 200);
        assert_eq!(rjson.data.get("message").unwrap(), "Hello world");
    }
    
    #[test]
    fn test_back_echo() {
        let rocket = rocket::ignite()
            .mount("/", routes![super::back_echo]);
        
        // Form data
        let mut req = MockRequest::new(Method::Post, "/back/echo")
            .header(ContentType::Form)
            .body("echo=echo123");
        let mut response = req.dispatch_with(&rocket);

        assert_eq!(response.status(), Status::Ok);
        let rjson = response.body()
            .map_or(
                ResponseJSON::error(),
                |body| ResponseJSON::from_reader(body.into_inner()).unwrap()
            );

        assert_eq!(rjson.is_ok_json(), true);
        assert_eq!(rjson.http_code, 200);
        assert_eq!(rjson.data["message"], "Hello world");
        assert_eq!(rjson.data["echo"], "I received from you > \"echo123\"");

        // JSON data
        let mut req = MockRequest::new(Method::Post, "/back/echo")
            .header(ContentType::JSON)
            .body(r#"{ "echo": "echo123" }"#);
        let mut response = req.dispatch_with(&rocket);

        assert_eq!(response.status(), Status::Ok);
        let rjson = response.body()
            .map_or(
                ResponseJSON::error(),
                |body| ResponseJSON::from_reader(body.into_inner()).unwrap()
            );

        assert_eq!(rjson.is_ok_json(), true);
        assert_eq!(rjson.http_code, 200);
        assert_eq!(rjson.data["message"], "Hello world");
        assert_eq!(rjson.data["echo"], "I received from you > \"echo123\"");
    }
}