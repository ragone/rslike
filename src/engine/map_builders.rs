use engine::{Tile, Map};
use util::FromChar;
use util::units::Size;

pub trait IntoMap {
    fn as_map(self) -> MapBuildResult;
}

impl IntoMap for Vec<Vec<Tile>> {
    fn as_map(self) -> MapBuildResult {
        // TODO: check dimensions are the same
        let size = Size::new(self.len() as i32, self[0].len() as i32);

        Ok(Map {
            tiles: self,
            size: size,
        })
    }
}

fn build_line(l: &&str) -> Vec<Tile> {
    l.chars().map(|c| Tile::from_char(c)).collect()
}

impl IntoMap for String {
    fn as_map(self) -> MapBuildResult {
        let lines: Vec<&str> = self.split('\n').filter(|l| !l.is_empty()).collect();

        if !lines.iter().all(|x| x.len() == lines[0].len()) { return Err("Different length lines") };

        let tiles: Vec<Vec<Tile>> = lines.iter().map(build_line).collect();

        Ok(Map::new(tiles))
    }
}

pub type MapBuildResult = Result<Map, &'static str>;
