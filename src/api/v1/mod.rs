mod model;

use rocket::{delete, get, post, put, Route, routes, State};
use rocket::http::Status;
use rocket::serde::json::Json;

use model::leg::Leg;
use crate::api::v1::model::race::Race;
use crate::polar::PolarService;
use crate::race;
use crate::race::{RaceError, RaceService};

pub(crate) fn routes() -> Vec<Route> {
    routes![list, get, post, put, delete, archive, restore, post_leg]
}

#[get("/races?<archived>")]
async fn list(race_service: &State<RaceService>, archived: Option<bool>) -> Result<Json<Vec<Race>>, Status> {

    match race_service.list(archived).await {
        Ok(races) => Ok(Json(races.into_iter().map(|r| r.into()).collect())),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/races/<race_id>")]
async fn get(race_service: &State<RaceService>, race_id: String) -> Result<Json<Race>, Status> {

    match race_service.get(race_id).await {
        Ok(None) => Err(Status::NotFound),
        Ok(Some(race)) => Ok(Json(race.into())),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[post("/races", data = "<race>")]
async fn post(race_service: &State<RaceService>, race: Json<Race>) -> Status {

    match race_service.create(&race.into_inner().into()).await {
        Ok(_) => Status::Created,
        Err(error) => {
            match error.downcast_ref::<RaceError>() {
                Some(RaceError::AlreadyExists(_)) => Status::Conflict,
                Some(RaceError::IdIsMandatory()) => Status::BadRequest,
                _ => Status::InternalServerError,
            }
        }
    }
}

#[post("/races/<race_id>/archive")]
async fn archive(race_service: &State<RaceService>, race_id: String) -> Status {
    match race_service.archive(race_id).await {
        Ok(_) => Status::Ok,
        Err(error) => {
            match error.downcast_ref::<RaceError>() {
                Some(RaceError::NotFound(_)) => Status::NotFound,
                _ => Status::InternalServerError,
            }
        }
    }
}

#[post("/races/<race_id>/restore")]
async fn restore(race_service: &State<RaceService>, race_id: String) -> Status {
    match race_service.restore(race_id).await {
        Ok(_) => Status::Created,
        Err(error) => {
            match error.downcast_ref::<RaceError>() {
                Some(RaceError::NotFound(_)) => Status::NotFound,
                Some(RaceError::AlreadyExists(_)) => Status::Conflict,
                _ => Status::InternalServerError,
            }
        }
    }
}

#[put("/races/<race_id>", data = "<race>")]
async fn put(race_service: &State<RaceService>, race_id: String, race: Json<Race>) -> Status {

    match race_service.update(race_id, &race.into_inner().into()).await {
        Ok(_) => Status::NoContent,
        Err(error) => {
            match error.downcast_ref::<RaceError>() {
                Some(RaceError::NotFound(_)) => Status::NotFound,
                _ => Status::InternalServerError,
            }
        }
    }
}

#[delete("/races/<race_id>")]
async fn delete(race_service: &State<RaceService>, race_id: String) -> Status {

    match race_service.delete(race_id).await {
        Ok(_) => Status::NoContent,
        Err(error) => {
            match error.downcast_ref::<RaceError>() {
                Some(RaceError::NotFound(_)) => Status::NotFound,
                _ => Status::InternalServerError,
            }
        }
    }
}

#[post("/legs", data = "<leg>")]
async fn post_leg(race_service: &State<RaceService>, polar_service: &State<PolarService>, leg: Json<Leg>) -> Status {

    let leg = leg.into_inner();

    let boat = polar_service.get_boat(leg.boat.polar_id).await.unwrap_or(String::from(""));

    let mut race: race::Race = leg.into();

    race.boat = boat;

    match race_service.create(&race).await {
        Ok(_) => Status::Created,
        Err(error) => {
            match error.downcast_ref::<RaceError>() {
                Some(RaceError::AlreadyExists(_)) => Status::Conflict,
                _ => Status::InternalServerError,
            }
        }
    }
}
