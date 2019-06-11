use chrono::{DateTime, Utc};
use now_lambda::Body;
use serde::{Deserialize, Serialize};
use serde_json;
use std::convert::From;
use uuid::Uuid;

/// A barcoded piece of equipment
#[derive(Deserialize, Serialize)]
pub struct Piece {
    /// The unique id of the piece
    pub id: Uuid,
    /// The barcode of the piece
    pub barcode: String,
    /// The name of the piece
    pub name: String,
    /// The serial number of the piece
    pub serial: String,
    /// The status of the piece
    pub status: String,
    /// The id of the equipment the piece belongs to
    pub equipment_id: Uuid,
}

/// A specific type of equipment
#[derive(Deserialize, Serialize)]
pub struct Equipment {
    /// The unique id of the equipment
    pub id: Uuid,
    /// The manufacturer of the equipment
    pub manufacturer: String,
    /// The model of the equipment
    pub model: String,
    /// The barcoded pieces of the equipment
    pub pieces: Vec<Piece>,
    /// The id of the kit the equipment may belong to
    pub kit_id: Uuid,
    /// The id of the category the equipment may belong to
    pub category_id: Uuid,
}

impl From<Body> for Equipment {
    fn from(body: Body) -> Equipment {
        let data = String::from(body);
        let equipment: Equipment = serde_json::from_str(&data).unwrap();
        return equipment;
    }
}

#[derive(Serialize, Deserialize)]
/// A kit of different types of equipment
pub struct Kit {
    /// The unique id of the kit
    pub id: Uuid,
    /// The name of the kit
    pub name: String,
    /// The equipment in the kit
    pub equipment: Vec<Equipment>,
    /// The id of the category the kit may belong to
    pub category_id: Uuid,
}

#[derive(Serialize, Deserialize)]
/// A category of equipment and kits
pub struct Category {
    /// The unique id of the category
    pub id: Uuid,
    /// The name of the category
    pub name: String,
    /// The equipment in the category that is not in a kit
    pub kitless_equipment: Vec<Equipment>,
    /// The kits in the category
    pub kits: Vec<Kit>,
}

#[derive(Serialize, Deserialize)]
/// A room that can be reserved
pub struct Room {
    /// The unique id of the room
    pub id: Uuid,
    /// The name of the room
    pub name: String,
    /// The location of the room
    pub location: String,
    /// The description of the room
    pub description: String,
}

#[derive(Serialize, Deserialize)]
/// A person in the system
pub struct Person {
    /// The unique id of the person
    pub id: Uuid,
    /// The username of the perso
    pub username: String,
    /// The first name of the person
    pub first_name: String,
    /// The last name of the person
    pub last_name: String,
    /// The role of the person in the system
    pub role: String,
}

#[derive(Serialize, Deserialize)]
/// A project done within a course
pub struct Project {
    /// The unique id of the project
    pub id: Uuid,
    /// The name of the project
    pub name: String,
    /// The id of the course the project belongs to
    pub course_id: Uuid,
}

#[derive(Serialize, Deserialize)]
/// An instructional course taken by students in the system
pub struct Course {
    /// The unique id of the course
    pub id: Uuid,
    /// The name of the course
    pub name: String,
    /// The code of the course
    pub code: String,
    /// The semester of the course
    pub semester: String,
    /// The instructor of the course
    pub instructor: Person,
    /// The students in the course
    pub students: Vec<Person>,
    /// The projects in the course
    pub projects: Vec<Project>,
    /// The equipment allowed for use in the course
    pub equipment: Vec<Equipment>,
    /// The kits allowed for use in the course
    pub kits: Vec<Kit>,
    /// The categories of kits and equipment allowed for use in the course
    pub categories: Vec<Category>,
    /// The rooms allowed for use in the course
    pub rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize)]
/// A reservation of equipment, kits, or rooms
pub struct Reservation {
    /// The unique id of the reservation
    pub id: Uuid,
    /// The person requesting the reservation
    pub requestor: Person,
    /// The person creating the reservation, could be the same as the requestor
    pub creator: Person,
    /// The course the reservation is for
    pub course: Course,
    /// The project the reservation is for
    pub project: Project,
    /// The equipment on the reservation
    pub equipment: Vec<Equipment>,
    /// The kits on the reservation
    pub kits: Vec<Kit>,
    /// The rooms on the reservation
    pub rooms: Vec<Room>,
    /// The start date and time of the reservation
    pub start: DateTime<Utc>,
    /// The end date and time of the reservation
    pub end: DateTime<Utc>,
}
