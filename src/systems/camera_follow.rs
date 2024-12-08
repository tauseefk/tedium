use crate::prelude::*;

pub fn camera_follow_system(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut intermediate_camera_query: Query<&mut IntermediateCamera>,
) {
    if player_query.get_single().is_err() {
        return;
    }

    let player_transform = player_query.single();

    intermediate_camera_query
        .iter_mut()
        .for_each(|mut intermediate_camera| {
            if (intermediate_camera.transform.translation.x - player_transform.translation.x).abs()
                > CAMERA_BB_WIDTH / 2.0
            {
                let sign = match (intermediate_camera.transform.translation.x
                    - player_transform.translation.x)
                    .is_sign_positive()
                {
                    false => -1.0,
                    _ => 1.0,
                };

                intermediate_camera.transform.translation.x = asymptotic_avg(
                    intermediate_camera.transform.translation.x,
                    player_transform.translation.x + (sign * CAMERA_BB_WIDTH / 2.0),
                    CAMERA_FOLLOW_SPEED,
                );
            }

            if (intermediate_camera.transform.translation.y - player_transform.translation.y).abs()
                > CAMERA_BB_HEIGHT / 2.0
            {
                let sign = match (intermediate_camera.transform.translation.y
                    - player_transform.translation.y)
                    .is_sign_positive()
                {
                    false => -1.0,
                    _ => 1.0,
                };

                intermediate_camera.transform.translation.y = asymptotic_avg(
                    intermediate_camera.transform.translation.y,
                    player_transform.translation.y + (sign * CAMERA_BB_HEIGHT / 2.0),
                    CAMERA_FOLLOW_SPEED,
                );
            }
        });
}

/// The system is responsible for updating camera position
pub fn camera_transform_system(
    intermediate_camera_query: Query<&IntermediateCamera>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if intermediate_camera_query.get_single().is_err() || camera_query.get_single().is_err() {
        return;
    }

    let intermediate_camera_transform = intermediate_camera_query.single().transform;
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation = Vec3 {
        x: intermediate_camera_transform.translation.x,
        y: intermediate_camera_transform.translation.y,
        z: camera_transform.translation.z,
    };
}
