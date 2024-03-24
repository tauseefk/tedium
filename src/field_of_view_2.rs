pub use crate::prelude::*;

pub struct World {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TileType {
    Transparent,
    Opaque,
}

impl From<&char> for TileType {
    fn from(value: &char) -> Self {
        match value {
            'o' => TileType::Opaque,
            '_' => TileType::Transparent,
            _ => panic!("Encountered improbable tiletype"),
        }
    }
}

enum Pivot {
    Center,
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}
pub struct World {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

pub struct Visibility {
    is_omniscient: bool,
    max_visible_distance: i32,
    visible_tiles: HashSet<GridPosition>,
    pub observer: GridPosition,
}

struct SlopeRange {
    s_min: f32,
    s_max: f32,
}

impl Visibility {
    pub fn new(is_omniscient: bool, max_visible_distance: i32) -> Self {
        Self {
            is_omniscient,
            max_visible_distance,
            visible_tiles: HashSet::new(),
            observer: GridPosition { x: 0, y: 0 },
        }
    }

    fn grid_point_on_scan_line(&self, depth: i32, slope: f32) -> GridPosition {
        let x = depth;
        let y = x as f32 * slope;

        GridPosition {
            x: x + self.observer.x,
            y: y as i32 + self.observer.y,
        }
    }

    pub fn calculate_vis(&mut self, world: &World, observer: GridPosition) {
        // update all visible tiles on the hash set

        let slope_range = SlopeRange {
            s_min: 0.,
            s_max: 1.,
        };
        let mut slopes: Vec<SlopeRange> = vec![slope_range];
        let mut depth_curr = 0;
        let depth_max = world.width - observer.x;
        while depth_curr <= depth_max {
            // iter between slope_min -> slope_max

            let mut slopes_local: Vec<SlopeRange> = vec![];
            for slope in slopes {
                let mut current = self.grid_point_on_scan_line(depth_curr, slope.s_min);
                let end = self.grid_point_on_scan_line(depth_curr, slope.s_max);
            }
        }
    }
}
