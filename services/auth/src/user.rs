use std::sync::Mutex;
use actix_web::web;
use crate::role;


#[derive(Clone)]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub username: String,
    pub pw: String,
    pub role: role::Role,
}

pub type UserRegister = std::collections::HashMap<String, User>;

pub (crate) fn init_users() -> web::Data<Mutex<UserRegister>> {
    let mut map = UserRegister::new();

    map.insert(
        String::from("1"),
        User {
            uuid: String::from("1"),
            username: String::from("User_0"),
            email: String::from("user_0@myemail.com"),
            pw: String::from("12345678"),
            role: role::Role::Visitor,
        },
    );

    map.insert(
        String::from("2"),
        User {
            uuid: String::from("2"),
            username: String::from("User_1"),
            email: String::from("user_1@myemail.com"),
            pw: String::from("98765432"),
            role: role::Role::Visitor,
        },
    );

    web::Data::new(Mutex::new(map))

}