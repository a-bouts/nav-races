use rocket::{Build, Rocket};

pub(crate) mod v1;

pub(crate) fn init() -> Rocket<Build> {

    rocket::build()
        .mount("/races/api/v1", v1::routes())
}
