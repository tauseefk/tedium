use crate::prelude::*;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Hidden {
    pub visibility: u8,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

#[derive(Clone, Default, Bundle)]
pub struct ChestBundle {
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    pub point_of_interest: PointOfInterest,
}

// As we're using instance field `active` to determine the first active point of interest
// we need to implement LdtkEntity by hand.
// TODO: Just using macros was a lot easier, I wonder if there's a way to still map fields using macros
impl LdtkEntity for ChestBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        _: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> ChestBundle {
        let sprite_sheet_bundle = bevy_ecs_ldtk::utils::sprite_sheet_bundle_from_entity_info(
            entity_instance,
            tileset,
            tileset_definition,
            texture_atlases,
            false,
        );

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
                    sprite_sheet_bundle,
                    point_of_interest,
                }
            }
            None => ChestBundle {
                sprite_sheet_bundle,
                point_of_interest: PointOfInterest { active: false },
            },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PointOfInterest {
    pub active: bool,
}

#[derive(Component)]
pub struct Path;

#[derive(Component)]
pub struct PlaySpeed {
    pub multiplier: f32,
}

#[derive(Component)]
pub struct IntermediateCamera {
    pub transform: Transform,
}
