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

#[derive(Debug, Clone)]
struct SlopeRange {
    s_min: f32,
    s_max: f32,
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
    pub observer: GridPosition,
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

    pub fn drain_visible_tiles(&mut self) {
        self.visible_tiles.drain();
    }

    pub fn _is_tile_visible(
        &self,
        world: &World,
        observer_coords: &GridPosition,
        tile_coords: &GridPosition,
    ) -> bool {
        // TODO: this should prob happen at the world construction
        if world.tiles.is_empty() {
            panic!("World is too small.");
        }

        if world.tiles.len() != (world.width * world.height) as usize {
            panic!("World size is inconsistent");
        }

        if self.max_visible_distance < 1 {
            panic!("Can't see shit!");
        }

        if !is_in_bounds(observer_coords, world.width, world.height)
            && !is_in_bounds(tile_coords, world.width, world.height)
        {
            panic!("Coordinate out of bounds!");
        }

        world.tiles.len() == 1 || *observer_coords == *tile_coords || self.is_omniscient
    }

    fn get_tile_type(&self, world: &World, tile_coords: &GridPosition) -> TileType {
        let idx = grid_pos_to_idx(tile_coords, world.width, world.height);
        world.tiles[idx].clone()
    }

    pub fn slope(&self, tile: &GridPosition, pivot: Pivot) -> f32 {
        let target = pivot.abs_coords(tile);

        (target.y - self.observer.y as f32) / (target.x - self.observer.x as f32)
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
    ///
    /// it's impossible for the the slope to be infinite as each octant is calculated separately
    fn grid_point_on_scan_line(&self, depth: i32, slope: f32) -> GridPosition {
        let x = depth;
        let y = x as f32 * slope;

        GridPosition {
            x: x + self.observer.x,
            y: y as i32 + self.observer.y,
        }
    }

    pub fn compute_visible_tiles(&mut self, world: &World) -> HashSet<GridPosition> {
        // self.compute_visible_tiles_iterative(world);
        self.compute_visible_tiles_in_octant(world, 1, 0., 1.);
        self.visible_tiles.clone()
    }

    fn compute_visible_tiles_in_octant(
        &mut self,
        world: &World,
        current_depth: i32,
        mut min_slope: f32,
        max_slope: f32,
    ) {
        if current_depth > self.max_visible_distance
            || self.observer.x + current_depth >= GRID_CELL_COUNT
        {
            return;
        }

        let mut is_first = true;
        let mut previous = self.grid_point_on_scan_line(current_depth, min_slope);
        let mut current = self.grid_point_on_scan_line(current_depth, min_slope);
        let end = self.grid_point_on_scan_line(current_depth, max_slope);

        if !is_in_bounds(&previous, world.width, world.height)
            || !is_in_bounds(&current, world.width, world.height)
            || !is_in_bounds(&end, world.width, world.height)
        {
            return;
        }

        while current.y < end.y && end.y < world.height {
            self.visible_tiles.insert(current);

            match is_first {
                false => {
                    match (
                        self.get_tile_type(world, &previous),
                        self.get_tile_type(world, &current),
                    ) {
                        // first opaque cell after at least one transparent
                        (TileType::Transparent, TileType::Opaque) => {
                            let next_max_slope = self.slope(&current, Pivot::BottomRight);

                            self.compute_visible_tiles_in_octant(
                                world,
                                current_depth + 1,
                                min_slope,
                                next_max_slope,
                            );
                        }
                        // first transparent cell after at least one opaque
                        (TileType::Opaque, TileType::Transparent) => {
                            min_slope = self.slope(&current, Pivot::BottomLeft);
                        }
                        (_, _) => {}
                    }
                }
                true => {
                    is_first = false;
                }
            }
            previous = current;
            current.y += 1;
        }

        // TODO: uncomment after encountering the edge case
        // see through last group of transparent cells in a row
        if self.get_tile_type(world, &previous) == TileType::Transparent {
            self.compute_visible_tiles_in_octant(world, current_depth + 1, min_slope, max_slope);
        }
    }

    pub fn _compute_visible_tiles_iterative(&mut self, world: &World) {
        // update all visible tiles on the hash set

        let slope_range = SlopeRange {
            s_min: 0.,
            s_max: 1.,
        };
        let mut slopes_next: Vec<SlopeRange> = vec![slope_range];
        let mut depth_curr = 0;
        let depth_max = world.width - self.observer.x;
        while depth_curr < depth_max {
            let mut slopes_local: Vec<SlopeRange> = vec![];
            for slope in slopes_next.clone() {
                let mut previous = self.grid_point_on_scan_line(depth_curr, slope.s_min);
                let mut current = self.grid_point_on_scan_line(depth_curr, slope.s_min);
                let end = self.grid_point_on_scan_line(depth_curr, slope.s_max);
                let mut slope_min_local = slope.s_min;

                while current.y < end.y {
                    self.visible_tiles.insert(current);

                    match (
                        self.get_tile_type(world, &previous),
                        self.get_tile_type(world, &current),
                    ) {
                        (TileType::Transparent, TileType::Opaque) => {
                            slopes_local.push(SlopeRange {
                                s_min: slope_min_local,
                                s_max: self.slope(&current, Pivot::Center),
                            });
                        }
                        (TileType::Opaque, TileType::Transparent) => {
                            slope_min_local = self.slope(&current, Pivot::Center);
                        }
                        (_, _) => {}
                    }
                    previous = current;
                    current = GridPosition {
                        x: current.x,
                        y: max(current.y + 1, world.height - self.observer.y),
                    };
                }
            }
            if slopes_local.len() > 0 {
                slopes_next = slopes_local;
            }
            depth_curr += 1;
        }
    }
}
