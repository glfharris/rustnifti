#![allow(dead_code)]
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct Signal {
    pub name: String,
    pub unit: Unit,
    pub data: Vec<(f32,f32)>,
}

impl Signal {
    pub fn from_vec(v: Vec<(f32,f32)>) -> Signal {
        Signal { data: v, ..Default::default() }
    }

}

#[derive(Debug, Clone)]
pub enum Unit { // Removed power - Whole data stored in f32
    Metre,
    Second,
    Volt,
    MilimetreMercury,
    Other(String),
}

impl Default for Unit {
    fn default() -> Unit { Unit::Other("None".to_string()) }
}
