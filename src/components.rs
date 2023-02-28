use crate::prelude::*;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub player: Player,
    pub animation_state: PlayerAnimationState,
    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

#[derive(Clone, Default, Bundle)]
pub struct ChestBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub point_of_interest: PointOfInterest,
}

// As we're using instance field `active` to determine the first active point of interest
// we need to implement LdtkEntity by hand.
// TODO: Just using macros was a lot easier, I wonder if there's a way to still map fields using macros
impl LdtkEntity for ChestBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> ChestBundle {
        let texture_handle = asset_server.load("chest.PNG");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::splat(GRID_BLOCK_SIZE as f32), 1, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        match entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"active")
        {
            Some(active) => {
                let point_of_interest = match active.value {
                    FieldValue::Bool(active) => PointOfInterest { active },
                    _ => PointOfInterest { active: false },
                };

                ChestBundle {
                    sprite_sheet_bundle: SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        ..Default::default()
                    },
                    point_of_interest,
                }
            }
            None => ChestBundle {
                sprite_sheet_bundle: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    ..Default::default()
                },
                point_of_interest: PointOfInterest { active: false },
            },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PointOfInterest {
    pub active: bool,
}

#[derive(Component, Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub const fn try_new(x: i32, y: i32) -> Option<Self> {
        if x <= 0 || y <= 0 || x > GRID_SIZE as i32 || y > GRID_SIZE as i32 {
            None
        } else {
            Some(Self {
                x: x as i32,
                y: y as i32,
            })
        }
    }

    pub const fn min(self) -> bool {
        self.x == 1 && self.y == 1
    }

    pub const fn max(self) -> bool {
        self.x == GRID_SIZE && self.y == GRID_SIZE
    }
}

#[derive(Component)]
pub struct Path;

#[derive(Component)]
pub struct PlaySpeed {
    pub multiplier: f32,
}
