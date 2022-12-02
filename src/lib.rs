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
    pub days: HashMap<String, Day>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Day {
    pub start: Time,
    pub end: Time,
    pub duration: i64,
    #[serde(rename = "payableDuration")]
    pub payable_duration: i64,
    pub shifts: HashMap<String, Shift>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Shift {
    #[serde(rename = "shiftId")]
    pub shift_id: String,
    pub start: Time,
    pub end: Time,
    pub duration: i64,
    #[serde(rename = "payableDuration")]
    pub payable_duration: i64,
    pub segments: Vec<Segment>,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Segment {
    #[serde(rename = "shiftId")]
    pub shift_id: String,
    pub start: Time,
    pub end: Time,
    duration: i64,
    #[serde(rename = "payableDuration")]
    payable_duration: i64,
    #[serde(default)]
    pub department: String,
    #[serde(rename = "type")]
    pub segment_type: Type,
    published: bool,
    pub location: String,
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
