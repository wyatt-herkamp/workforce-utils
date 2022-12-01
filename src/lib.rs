pub mod build_calander;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Time = DateTime<Utc>;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Schedule {
    start: Time,
    end: Time,
    duration: i64,
    #[serde(rename = "payableDuration")]
    payable_duration: i64,
    days: HashMap<String, Day>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Day {
    start: Time,
    end: Time,
    duration: i64,
    #[serde(rename = "payableDuration")]
    payable_duration: i64,
    shifts: HashMap<String, Shift>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Shift {
    #[serde(rename = "shiftId")]
    shift_id: String,
    start: Time,
    end: Time,
    duration: i64,
    #[serde(rename = "payableDuration")]
    payable_duration: i64,
    segments: Vec<Segment>,
    published: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Segment {
    #[serde(rename = "shiftId")]
    shift_id: String,
    start: Time,
    end: Time,
    duration: i64,
    #[serde(rename = "payableDuration")]
    payable_duration: i64,
    #[serde(default)]
    department: String,
    #[serde(rename = "type")]
    segment_type: Type,
    published: bool,
    location: String,
    payable: bool,
    details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Type {
    #[serde(rename = "break")]
    Break,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "special")]
    Special,
}
