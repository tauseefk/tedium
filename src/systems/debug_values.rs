use crate::prelude::*;

/// Toggle the wall block at the location described in the event
pub fn debug_values(
    mut debug_values: EventReader<UpdateDebugValuesEvent>,
    player_query: Query<&Transform, With<Player>>,
    visibility: Res<crate::field_of_view::Visibility>,
) {
    if player_query.get_single().is_err() {
        return;
    }

    let player_translation = player_query.get_single().unwrap().translation;

    for event in debug_values.iter() {
        let event: &UpdateDebugValuesEvent = event;

        let player_grid_pos = translation_to_grid_pos(player_translation);
        let vis_observer_grid_pos = visibility.observer;
        let tile = translation_to_grid_pos(event.translation);

        if tile.is_none() || player_grid_pos.is_none() {
            break;
        }
        let tile = tile.unwrap();
        let player_grid_pos = player_grid_pos.unwrap();

        let octant_1_slope =
            Octant::NorthOfEast.slope_abs(&visibility.observer, &tile, Pivot::_Center);
        let octant_2_slope =
            Octant::EastOfNorth.slope_abs(&visibility.observer, &tile, Pivot::_Center);

        let octant_3_slope =
            Octant::WestOfNorth.slope_abs(&visibility.observer, &tile, Pivot::_Center);

        let octant_4_slope =
            Octant::NorthOfWest.slope_abs(&visibility.observer, &tile, Pivot::_Center);

        println!("{player_grid_pos:?}");
        println!("{vis_observer_grid_pos:?}");
        println!("{tile:?}");

        println!("_\\{octant_3_slope:1.3}|{octant_2_slope:1.3}/1");
        println!("__\\____|____/__");
        println!("___\\___|___/___");
        println!("____\\__|__/____");
        println!("{octant_4_slope:1.3}\\_|_/{octant_1_slope:1.3}");
        println!("------@_@------");
        println!("55555/_|_\\88888");
        println!("____/__|__\\____");
        println!("___/___|___\\___");
        println!("__/____|____\\__");
        println!("_/66666|77777\\_");
    }
}
