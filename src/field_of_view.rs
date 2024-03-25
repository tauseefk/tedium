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

/// Pivots are oriented along the game world, which also aligns with Octant::NorthOfEast
pub enum Pivot {
    _Center,
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl Pivot {
    pub fn abs_coords(&self, tile_coords: &GridPosition) -> Vec3 {
        let translation = grid_to_translation(*tile_coords);

        match self {
            Pivot::_Center => translation,
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

    pub fn flip_x(&self) -> Pivot {
        match self {
            Pivot::_Center => Pivot::_Center,
            Pivot::TopRight => Pivot::TopLeft,
            Pivot::BottomRight => Pivot::BottomLeft,
            Pivot::BottomLeft => Pivot::BottomRight,
            Pivot::TopLeft => Pivot::TopRight,
        }
    }

    pub fn flip_y(&self) -> Pivot {
        match self {
            Pivot::_Center => Pivot::_Center,
            Pivot::TopRight => Pivot::BottomRight,
            Pivot::BottomRight => Pivot::TopRight,
            Pivot::BottomLeft => Pivot::TopLeft,
            Pivot::TopLeft => Pivot::BottomLeft,
        }
    }
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
#[derive(Copy, Clone)]
pub enum Octant {
    NorthOfEast,
    EastOfNorth,
    WestOfNorth,
    NorthOfWest,
    SouthOfWest,
    WestOfSouth,
    EastOfSouth,
    SouthOfEast,
}

impl Octant {
    /// Get the grid point on scan line per quadrant
    pub fn grid_point_on_scan_line(
        &self,
        observer: GridPosition,
        depth: i32,
        slope: f32,
    ) -> GridPosition {
        match self {
            Self::NorthOfEast => {
                let x = depth;
                let y = (x as f32 * slope) as i32;

                GridPosition {
                    x: x + observer.x - 1,
                    y: y + observer.y,
                }
            }
            Self::EastOfNorth => {
                let y = depth;
                let x = (y as f32 * slope) as i32;

                GridPosition {
                    x: x + observer.x,
                    y: y + observer.y - 1,
                }
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn get_adjusted_pivot(&self, pivot: Pivot) -> Pivot {
        match self {
            Octant::NorthOfEast => pivot,
            Octant::EastOfNorth => pivot.flip_x().flip_y(),
            Octant::WestOfNorth => todo!(),
            Octant::NorthOfWest => todo!(),
            Octant::SouthOfWest => todo!(),
            Octant::WestOfSouth => todo!(),
            Octant::EastOfSouth => todo!(),
            Octant::SouthOfEast => todo!(),
        }
    }

    /// Returns the absolute slope for an octant
    /// Octants (1, 4, 5, 8) and (2, 3, 6, 7) should have the same absolute slope
    pub fn slope(&self, observer: &GridPosition, tile: &GridPosition, pivot: Pivot) -> f32 {
        let pivot = self.get_adjusted_pivot(pivot);
        let target = pivot.abs_coords(tile);

        match self {
            Octant::NorthOfEast
            | Octant::NorthOfWest
            | Octant::SouthOfWest
            | Octant::SouthOfEast => {
                ((target.y - observer.y as f32) / (target.x - observer.x as f32)).abs()
            }
            Octant::EastOfNorth
            | Octant::WestOfNorth
            | Octant::WestOfSouth
            | Octant::EastOfSouth => {
                ((target.x - observer.x as f32) / (target.y - observer.y as f32)).abs()
            }
        }
    }

    pub fn get_next_tile_on_scanline(&self, tile: &GridPosition) -> GridPosition {
        match self {
            Octant::NorthOfEast => GridPosition {
                x: tile.x,
                y: tile.y + 1,
            },
            Octant::EastOfNorth => GridPosition {
                x: tile.x + 1,
                y: tile.y,
            },
            Octant::WestOfNorth => todo!(),
            Octant::NorthOfWest => todo!(),
            Octant::SouthOfWest => todo!(),
            Octant::WestOfSouth => todo!(),
            Octant::EastOfSouth => todo!(),
            Octant::SouthOfEast => todo!(),
        }
    }
}

pub struct Visibility {
    _is_omniscient: bool,
    max_visible_distance: i32,
    visible_tiles: HashSet<GridPosition>,
    pub observer: GridPosition,
}

impl Visibility {
    pub fn new(is_omniscient: bool, max_visible_distance: i32) -> Self {
        Self {
            _is_omniscient: is_omniscient,
            max_visible_distance,
            visible_tiles: HashSet::new(),
            observer: GridPosition { x: 0, y: 0 },
        }
    }

    pub fn drain_visible_tiles(&mut self) {
        self.visible_tiles.drain();
    }

    fn get_tile_type(&self, world: &World, tile_coords: &GridPosition) -> TileType {
        let idx = grid_pos_to_idx(tile_coords, world.width, world.height);
        world.tiles[idx].clone()
    }

    pub fn compute_visible_tiles(&mut self, world: &World) -> HashSet<GridPosition> {
        self.compute_visible_tiles_in_octant(world, Octant::NorthOfEast, 1, 0., 1.);
        self.compute_visible_tiles_in_octant(world, Octant::EastOfNorth, 1, 0., 1.);
        self.visible_tiles.clone()
    }

    // pub fn is_tile_in_bounds(&self, octant: Octant, tile: &GridPosition) -> bool {}
    fn compute_visible_tiles_in_octant(
        &mut self,
        world: &World,
        octant: Octant,
        current_depth: i32,
        mut min_slope: f32,
        max_slope: f32,
    ) {
        let mut is_first = true;
        let mut previous = octant.grid_point_on_scan_line(self.observer, current_depth, min_slope);
        let mut current = octant.grid_point_on_scan_line(self.observer, current_depth, min_slope);
        let end = octant.grid_point_on_scan_line(self.observer, current_depth, max_slope);

        if !is_in_bounds(&previous, world.width, world.height)
            || !is_in_bounds(&current, world.width, world.height)
            || !is_in_bounds(&end, world.width, world.height)
        {
            return;
        }

        while self.observer.square_distance(current) < self.max_visible_distance.pow(2) {
            self.visible_tiles.insert(current);

            match is_first {
                false => {
                    match (
                        self.get_tile_type(world, &previous),
                        self.get_tile_type(world, &current),
                    ) {
                        // first opaque cell after at least one transparent
                        (TileType::Transparent, TileType::Opaque) => {
                            let next_max_slope =
                                octant.slope(&self.observer, &current, Pivot::BottomRight);

                            self.compute_visible_tiles_in_octant(
                                world,
                                octant,
                                current_depth + 1,
                                min_slope,
                                next_max_slope,
                            );
                        }
                        // first transparent cell after at least one opaque
                        (TileType::Opaque, TileType::Transparent) => {
                            min_slope = octant.slope(&self.observer, &current, Pivot::BottomLeft);
                        }
                        (_, _) => {}
                    }
                }
                true => {
                    is_first = false;
                }
            }
            previous = current;
            current = octant.get_next_tile_on_scanline(&current);

            // if the slope of the current cell exceeds max_slope, we can stop calculating
            if octant.slope(&self.observer, &current, Pivot::BottomRight) > max_slope {
                break;
            }
        }

        // see through last group of transparent cells in a row
        if self.get_tile_type(world, &previous) == TileType::Transparent {
            self.compute_visible_tiles_in_octant(
                world,
                octant,
                current_depth + 1,
                min_slope,
                max_slope,
            );
        }
    }
}
