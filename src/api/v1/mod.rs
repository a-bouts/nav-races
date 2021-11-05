use std::collections::HashMap;

use rocket::{delete, get, post, put, Route, routes, State};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::api::leg::Leg;
use crate::race::{Race, RaceError, RaceService};

pub(crate) fn routes() -> Vec<Route> {
    routes![list, get, post, put, delete, archive, restore, post_leg]
}

#[get("/races")]
async fn list(race_service: &State<RaceService>) -> Result<Json<HashMap<String, Race>>, Status> {

    match race_service.list().await {
        Ok(races) => Ok(Json(races)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/races/<race_id>")]
async fn get(race_service: &State<RaceService>, race_id: String) -> Result<Json<Race>, Status> {

    match race_service.get(race_id).await {
        Ok(None) => Err(Status::NotFound),
        Ok(Some(race)) => Ok(Json(race)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[post("/races", data = "<race>")]
async fn post(race_service: &State<RaceService>, race: Json<Race>) -> Status {

    match race_service.create(&race.into_inner()).await {
        Ok(_) => Status::Created,
        Err(error) => {
            match error.downcast_ref::<RaceError>() {
                Some(RaceError::AlreadyExists(_)) => Status::Conflict,
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

    match race_service.update(race_id, &race.into_inner()).await {
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
async fn post_leg(race_service: &State<RaceService>, leg: Json<Leg>) -> Status {

    let race: Race = leg.into_inner().into();

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