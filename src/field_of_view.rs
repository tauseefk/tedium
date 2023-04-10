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

pub enum Pivot {
    Center,
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl Pivot {
    pub fn abs_coords(&self, tile_coords: &GridPosition) -> Vec3 {
        let translation = grid_to_translation(*tile_coords);

        match self {
            Pivot::Center => translation,
            Pivot::TopRight => Vec3::new(
                translation.x + (GRID_BLOCK_SIZE / 2) as f32,
                translation.y + (GRID_BLOCK_SIZE / 2) as f32,
                translation.z,
            ),
            Pivot::BottomRight => Vec3::new(
                translation.x + (GRID_BLOCK_SIZE / 2) as f32,
                translation.y - (GRID_BLOCK_SIZE / 2) as f32,
                translation.z,
            ),
            Pivot::BottomLeft => Vec3::new(
                translation.x - (GRID_BLOCK_SIZE / 2) as f32,
                translation.y - (GRID_BLOCK_SIZE / 2) as f32,
                translation.z,
            ),
            Pivot::TopLeft => Vec3::new(
                translation.x - (GRID_BLOCK_SIZE / 2) as f32,
                translation.y + (GRID_BLOCK_SIZE / 2) as f32,
                translation.z,
            ),
        }
    }
}

pub struct Visibility {
    is_omniscient: bool,
    max_visible_distance: i32,
    visible_tiles: HashSet<GridPosition>,
    observer: GridCoords,
}

impl Visibility {
    pub fn new(is_omniscient: bool, max_visible_distance: i32) -> Self {
        Self {
            is_omniscient,
            max_visible_distance,
            visible_tiles: HashSet::new(),
            observer: GridCoords { x: 0, y: 0 },
        }
    }

    pub fn is_tile_visible(
        &self,
        world: &World,
        observer_coords: &GridPosition,
        tile_coords: &GridPosition,
    ) -> bool {
        // TODO: this should prob happen at the world construction
        if world.tiles.len() < 1 {
            panic!("World is too small.");
        }

        if world.tiles.len() != (world.width * world.height) as usize {
            panic!("World size is inconsistent");
        }

        if self.max_visible_distance < 1 {
            panic!("Can't see shit!");
        }

        if !self.is_in_bounds(world, observer_coords) && !self.is_in_bounds(world, tile_coords) {
            panic!("Coordinate out of bounds!");
        }

        if world.tiles.len() == 1 || *observer_coords == *tile_coords || self.is_omniscient {
            true
        } else {
            false
        }
    }

    fn get_tile_type(&self, world: &World, tile_coords: &GridPosition) -> TileType {
        let idx = self.grid_coord_to_idx(world, tile_coords);
        world.tiles[idx].clone()
    }

    pub fn slope(&self, tile: &GridPosition, pivot: Pivot) -> f32 {
        let target = pivot.abs_coords(tile);
        let slope =
            (target.y as f32 - self.observer.y as f32) / (target.x as f32 - self.observer.x as f32);
        return slope;
    }

    /// 4\33333|22222/1
    /// 44\3333|2222/11
    /// 444\333|222/111
    /// 4444\33|22/1111
    /// 44444\3|2/11111
    /// ------@_@------
    /// 55555/6|7\88888
    /// 5555/66|77\8888
    /// 555/666|777\888
    /// 55/6666|7777\88
    /// 5/66666|77777\8
    /// assuming we're only concerned with the octant 1
    /// scan lines are vertical so y = mx
    fn point_on_scan_line(&self, depth: i32, slope: f32) -> GridPosition {
        if slope.is_infinite() {
            GridPosition {
                x: self.observer.x,
                y: (self.observer.y + depth).min(self.max_visible_distance),
            }
        } else {
            let x = (self.observer.x + depth).min(self.max_visible_distance);
            let y = (x as f32 * slope).min(self.max_visible_distance as f32);

            GridPosition { x, y: y as i32 }
        }
    }

    pub fn compute_visible_tiles(&mut self, world: &World) -> HashSet<GridPosition> {
        self.compute_visible_tiles_in_octant(world, 1, 0., 1.);
        self.visible_tiles.clone()
    }

    fn compute_visible_tiles_in_octant(
        &mut self,
        world: &World,
        // observer: &GridCoords, this doesn't change during each call
        current_depth: i32,
        mut min_slope: f32,
        max_slope: f32,
    ) {
        if current_depth > self.max_visible_distance {
            return;
        }

        let mut is_first = true;
        let mut previous = self.point_on_scan_line(current_depth, min_slope);
        let mut current = self.point_on_scan_line(current_depth, min_slope);
        let end = self.point_on_scan_line(current_depth, max_slope);

        while current.y < end.y {
            self.visible_tiles.insert(current.clone());

            match (
                is_first,
                self.get_tile_type(world, &previous),
                self.get_tile_type(world, &current),
            ) {
                // first opaque cell after at least one transparent
                (false, TileType::Transparent, TileType::Opaque) => {
                    let next_max_slope = self.slope(&current, Pivot::BottomRight);
                    self.compute_visible_tiles_in_octant(
                        world,
                        current_depth + 1,
                        min_slope,
                        next_max_slope,
                    );
                }
                // first transparent cell after at least one opaque
                (false, TileType::Opaque, TileType::Transparent) => {
                    min_slope = self.slope(&previous, Pivot::TopLeft);
                }
                // do nothing
                (false, TileType::Transparent, TileType::Transparent) => {}
                (false, TileType::Opaque, TileType::Opaque) => {}
                (true, _, _) => {
                    is_first = false;
                }
            };
            previous = current.clone();
            current.y += 1;
        }
        // TODO: uncomment after encountering the edge case
        // see through last group of transparent cells in a row
        if self.get_tile_type(world, &previous) == TileType::Transparent {
            self.compute_visible_tiles_in_octant(world, current_depth + 1, min_slope, max_slope);
        }
    }

    fn grid_coord_to_idx(&self, world: &World, tile_coords: &GridPosition) -> usize {
        if !self.is_in_bounds(world, tile_coords) {
            panic!("Tile not in bounds");
        }

        let w = world.width;

        (tile_coords.x * w + tile_coords.y) as usize
    }

    fn is_in_bounds(&self, world: &World, tile_coords: &GridPosition) -> bool {
        let x = tile_coords.x;
        let y = tile_coords.y;

        x >= 0 && y >= 0 && x < world.width && y < world.height
    }
}
