use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub owner: DevInfo,
    pub plot: PlotInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlotInfo {
    pub name: Option<String>,
    pub id: Option<u64>,
    pub size: PlotSize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DevInfo {
    pub name: Option<String>,
    pub rank: PlotRank,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum PlotRank {
    Default = 0,
    Noble,
    Emperor,
    Mythic,
    Overlord,
    Admin
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum PlotSize {
    Basic,
    Large,
    Massive,
    Mega
}

impl PlotSize {
    pub fn max_blocks(&self) -> usize {
        match self {
            PlotSize::Basic => 24,
            PlotSize::Large => 49,
            PlotSize::Massive => 149,
            PlotSize::Mega => 149
        }
    }
}