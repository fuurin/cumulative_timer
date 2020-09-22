use crate::models::User;
use crate::schema::users::dsl::*;
use crate::db::connection;
use rocket_contrib::json::Json;
use diesel::prelude::*;

#[get("/users")]
pub fn index() -> Json<Vec<User>> {
    let connection = connection::establish();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");
    Json(results)
}

#[get("/users/<user_id>")]
pub fn show(user_id: i32) -> Option<Json<User>> {
    let connection = connection::establish();
    let result = users
        .filter(id.eq(user_id))
        .get_result::<User>(&connection);
    
    match result {
        Ok(user) => Some(Json(user)),
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {
    use crate::models::User;
    use rocket;
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};
    use serde_json;

    #[test]
    fn show_all_users() {
        let first_user = User {
            id: 1,
            name: String::from("first_user"),
            password: String::from("password")
        };
        let second_user = User {
            id: 2,
            name: String::from("second_user"),
            password: String::from("password")
        };
        let client = Client::new(rocket::ignite().mount("/", routes![super::index])).unwrap();
        let mut res = client.get("/users/")
            .header(ContentType::JSON)
            .dispatch();
        let body = res.body().unwrap().into_string().unwrap();
        assert_eq!(
            body, 
            serde_json::to_string(&vec![first_user, second_user]).unwrap()
        );
    }

    #[test]
    fn show_one_user() {
        let client = Client::new(rocket::ignite().mount("/", routes![super::show])).unwrap();
        let mut res = client.get("/users/1/")
            .header(ContentType::JSON)
            .dispatch();
        let body = res.body().unwrap().into_string().unwrap();
        assert_eq!(
            body, 
            serde_json::to_string(&User {
                id: 1,
                name: String::from("first_user"),
                password: String::from("password")
            }).unwrap()
        ); 
    }

    #[test]
    fn dont_show_non_existent_user() {
        let client = Client::new(
            rocket::ignite()
                .mount("/", routes![super::show])
        ).unwrap();
        let res = client.get("/users/0/")
            .header(ContentType::JSON)
            .dispatch();
        let status = res.status();
        assert_eq!(status, Status::NotFound); 
    }
}