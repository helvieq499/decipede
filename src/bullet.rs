use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Component)]
pub struct Bullet;

pub fn update_movement(mut query: Query<&mut Transform, With<Bullet>>) {
    query.for_each_mut(|mut transform| {
        transform.translation.y += 10.;
    });
}

pub fn update_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    snakes: Query<(Entity, &Transform, &crate::snake::Snake), Without<Bullet>>,
    junk: Query<(Entity, &Transform), (With<crate::junk::tile::JunkTile>, Without<Bullet>, Without<crate::snake::Snake>)>,
    mut grid: ResMut<crate::junk::grid::JunkGrid>,
) {
    for (bullet_entity, bullet_transform) in bullets.iter() {
        for (snake_entity, snake_transform, snake) in snakes.iter() {
            if collide(
                bullet_transform.translation,
                vec3_to_vec2(bullet_transform.scale),
                snake_transform.translation,
                vec3_to_vec2(snake_transform.scale),
            )
            .is_some()
            {
                commands.entity(bullet_entity).despawn();
                commands.entity(snake_entity).despawn();

                crate::junk::tile::create_junk_tile(&mut commands, grid.as_mut(), snake.x, snake.y);
            }
        }

        for (junk_entity, junk_transform) in junk.iter() {
            if collide(
                bullet_transform.translation,
                vec3_to_vec2(bullet_transform.scale),
                junk_transform.translation,
                vec3_to_vec2(junk_transform.scale)
            ).is_some() {
                commands.entity(bullet_entity).despawn();

                // todo: damage instead of destroy
                commands.entity(junk_entity).despawn();

                let (x, y) = crate::utils::pixels_to_grid(junk_transform.translation.x, junk_transform.translation.y);
                grid.0[x][y] = false;
            }
        }
    }
}

pub fn vec3_to_vec2(some: Vec3) -> Vec2 {
    Vec2::new(some.x, some.y)
}
