use crate::prelude::*;

/// Cycle through points of interest in order
pub fn cycle_point_of_interest(
    time: Res<Time>,
    mut cycle_timer: ResMut<CycleTimer>,
    mut my_events: EventReader<CyclePOIEvent>,
    mut poi_query: Query<&mut PointOfInterest>,
) {
    for _ in my_events.iter() {
        let active_idx = poi_query
            .iter()
            .enumerate()
            .find_map(|(idx, point_of_interest)| {
                if point_of_interest.active {
                    Some(idx)
                } else {
                    None
                }
            });

        if let Some(active_idx) = active_idx {
            let mut active_poi = poi_query.iter_mut().nth(active_idx).unwrap();
            active_poi.active = false;

            let next_idx = (active_idx + 1) % poi_query.iter().len();
            let mut next_poi = poi_query.iter_mut().nth(next_idx).unwrap();
            next_poi.active = true;
        }
    }

    cycle_timer.0.tick(time.delta());
    if cycle_timer.0.finished() {
        let active_idx = poi_query
            .iter()
            .enumerate()
            .find_map(|(idx, point_of_interest)| {
                if point_of_interest.active {
                    Some(idx)
                } else {
                    None
                }
            });

        if let Some(active_idx) = active_idx {
            let mut active_poi = poi_query.iter_mut().nth(active_idx).unwrap();
            active_poi.active = false;

            let next_idx = (active_idx + 1) % poi_query.iter().len();
            let mut next_poi = poi_query.iter_mut().nth(next_idx).unwrap();
            next_poi.active = true;
        }
    }
}
