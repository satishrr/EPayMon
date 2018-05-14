#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use(bson, doc)]
extern crate bson;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;
extern crate rustforum;
extern crate diesel;

use rocket_contrib::{JSON, Value};
use bson::Bson;
use std::sync::Arc;
use jwt::{encode, decode, Header, Algorithm};
use jwt::errors::{Error};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use self::rustforum::*;
use self::rustforum::models::*;
use self::diesel::prelude::*;


#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Registration {
    sub: String,
    company: String
}

 
impl Registration {
    fn is_valid(&self) -> bool {
 
        // Validation code 
        true
    }
}

const SECRET_KEY: &'static str = "satish12345";

struct Token (String);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Token, ()> {
        let keys: Vec<_> = request.headers().get("api-token").collect();
        
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        return Outcome::Success(Token(key.to_string()));
    }
}

#[derive(Deserialize)]
struct UserBean {
    user_name: String,
    wallet_info: String
}

#[derive(Serialize)]
struct UserBeanimpl {
    id: i32,
    user_name: String,
    wallet_info: String,
    user_id: String
}

#[get("/user_info")]
fn list_user_info(token: Token) -> JSON<Value> {

    let token_data = match decode::<Claims>(&token.0, SECRET_KEY.as_ref(), Algorithm::HS256) {
        Ok(c) => c,
        Err(err) => match err {
            Error::InvalidToken => panic!(),
            _ => panic!()
        }
    };
    
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
    println!("{:?}", token_data.claims.is_valid());

    use rustforum::schema::user::dsl::*;

    let connection = establish_connection();

    let results = user
        .load::<user_info>(&connection)
        .expect("Error loading posts");

    println!("Found {} user", results.len());
    
    let mut rows: Vec<UserBeanimpl> = vec![];

    for post in results {
        let user_info = UserBeanimpl {id: post.id, user_name: post.user_name, wallet_info: post.wallet_info, user_id: post.user_id};
        rows.push(user_info);
    }

    println!("Rows length: {}", rows.len());
    
    JSON(json!({
        "message": "Getting the user_info list...",
        "data": rows
    }))
}


#[get("/user_info/<uid>")]
fn get_user_info(token: Token, uid: &str) -> JSON<Value> {

    let token_data = match decode::<Claims>(&token.0, SECRET_KEY.as_ref(), Algorithm::HS256) {
        Ok(c) => c,
        Err(err) => match err {
            Error::InvalidToken => panic!(),
            _ => panic!()
        }
    };
    
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
    println!("{:?}", token_data.claims.is_valid());

    use rustforum::schema::user::dsl::*;

    let connection = establish_connection();

    println!("{}", uid);

    let row_id = uid.parse::<i32>().unwrap();

    let row = user
        .find(row_id)
        .first::<user_info>(&connection)
        .expect("Error loading posts");

    println!("{}", row.id);
    println!("{}", row.user_name);

	JSON(json!({
        "message": format!("Getting the user_info with id: {}", uid),
        "data": {
            "id": row.id,
            "user_name": row.user_name,
            "wallet_info":row.wallet_info,
            "user_id": row.user_id
        }
    }))

}

#[post("/user_info", format = "application/json", data = "<user_info>")]
fn create_user_info(token: Token, user_info: JSON<UserBean>) -> JSON<Value> {
    let user_name: String = user_info.0.user_name;
    let wallet_info: String = user_info.0.wallet_info;

    JSON(json!({
        "message": "Create the new user_info..",
        "data": {
            "user_name": format!("{}", user_name),
            "wallet_info": format!("{}", wallet_info)
        }
    }))
}

#[put("/user_info", format = "application/json", data = "<user_info>")]
fn update_user_info(token: Token, user_info: JSON<UserBean>) -> JSON<Value> {
    let user_name: String = user_info.0.user_name;
    let wallet_info: String = user_info.0.wallet_info;

    JSON(json!({
        "message": "Create the new user_info..",
        "data": {
            "user_name": format!("{}", user_name),
            "wallet_info": format!("{}", wallet_info)
        }
    }))
}


#[delete("/user_info/<id>")]
fn delete_user_info(id: &str) -> JSON<Value> {

    JSON(json!({
        "message": format!("Deleting the user_info with id: {}", id)
    }))

}

#[get("/")]
fn index() -> JSON<Value> {

    let my_claims = Claims {
        sub: "satish.mashale@gmail.com".to_owned(),
        company: "paymon".to_owned()
    };

    let token = match encode(Header::default(), &my_claims, SECRET_KEY.as_ref()) {
        Ok(t) => t,
        Err(_) => panic!() // in practice you would return the error
    };

    println!("{:?}", token);

    JSON(json!({
        "message": "Welcome to Form API :D"
    }))
}

fn main() {
    rocket::ignite()
    	.mount("/", routes![index, list_user_info, get_user_info, create_user_info, update_user_info, delete_user_info])
    	.launch();
}
