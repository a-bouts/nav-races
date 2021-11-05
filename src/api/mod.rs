use rocket::{Build, Rocket};

pub(crate) mod v1;
mod leg;

pub(crate) fn init() -> Rocket<Build> {

    rocket::build()
        .mount("/api/v1", v1::routes())
}
