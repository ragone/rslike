use engine::Map;

use std::fmt;

pub use self::string_builder::MapFromString;
pub use self::file_builder::MapFromFile;

mod string_builder;
mod file_builder;

pub trait MapBuilder {
    fn build(&self) -> Result<Map, MapBuildError>;
}

#[derive(Copy)]
pub struct MapBuildError {
    pub msg: &'static str,
}

impl MapBuildError {
    pub fn new(message: &'static str) -> MapBuildError {
        MapBuildError { msg: message }
    }
}

impl fmt::Show for MapBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MapBuildError: {}", self.msg)
    }
}

pub type MapBuildResult = Result<Map, MapBuildError>;
