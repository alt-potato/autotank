use crate::util::math::{Scalar, Vec2};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bullet {
    pub id: u32,
    pub position: Vec2,
    pub velocity: Vec2
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmState {
    // TODO: actually implement lol
    pub pc: u32,
    pub sp: u32,
    pub stack: Vec<u32>,
    pub memory: Vec<u32>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tank {
    pub id: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub angle: Scalar,
    pub turret_angle: Scalar,
    pub health: u32, // TODO: replace with component health
    pub vm: VmState,
    pub team_id: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimState {
    pub time: u64,
    pub seed: u64,
    pub tanks: Vec<Tank>,
    pub bullets: Vec<Bullet>
}
