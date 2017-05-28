// =======================================================================
// COMPILER ATTRIBUTES
// =======================================================================
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![deny(unused_must_use)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![allow(non_camel_case_types)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
extern crate amiwo;
#[allow(unused_imports)] #[macro_use] extern crate log;
extern crate rocket;
#[macro_use] extern crate serde_json;

#[cfg(test)]
extern crate hyper;

mod routes;

// =======================================================================
// MAIN FUNCTION
// =======================================================================
pub fn main() {
    rocket::ignite()
        .mount("/test", routes![routes::test::back_hello, routes::test::back_echo])
        .launch();
}