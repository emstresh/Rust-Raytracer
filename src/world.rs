use cgmath::Vector3;

use crate::hitable::Geometry;
use crate::material::Material;
use crate::texture::Texture;


pub fn cornell_box<'material>(
    green_material: &'material Material,
    red_material: &'material Material,
    white_material: &'material Material,
    emissive_material: &'material Material) -> Vec<Geometry<'material>> {
    let mut world = Vec::with_capacity(9);

    // green left
    world.push(Geometry::mesh(
        vec![
            555.0, 0.0, 0.0,
            555.0, 0.0, 555.0,
            555.0, 555.0, 0.0,
            555.0, 555.0, 555.0
        ],
        vec![ 0, 1, 2, 3, 2, 1 ],
        green_material
    ));

    // red right
    world.push(Geometry::mesh(
        vec![
            0.0, 0.0, 0.0,
            0.0, 555.0, 0.0,
            0.0, 0.0, 555.0,
            0.0, 555.0, 555.0
        ],
        vec![ 0, 1, 2, 3, 2, 1 ],
        red_material
    ));

    // white bottom
    world.push(Geometry::mesh(
        vec![
            0.0, 0.0, 0.0,
            0.0, 0.0, 555.0,
            555.0, 0.0, 0.0,
            555.0, 0.0, 555.0
        ],
        vec![ 0, 1, 2, 3, 2, 1 ],
        white_material
    ));

    // white top
    world.push(Geometry::mesh(
        vec![
            0.0, 555.0, 0.0,
            555.0, 555.0, 0.0,
            0.0, 555.0, 555.0,
            555.0, 555.0, 555.0
        ],
        vec![ 0, 1, 2, 3, 2, 1 ],
        white_material
    ));

    // white back
    world.push(Geometry::mesh(
        vec![
            0.0, 0.0, 555.0,
            0.0, 555.0, 555.0,
            555.0, 0.0, 555.0,
            555.0, 555.0, 555.0
        ],
        vec![ 0, 1, 2, 3, 2, 1 ],
        white_material
    ));

    // light
    world.push(Geometry::mesh(
        vec![
            213.0, 554.0, 227.0,
            213.0, 554.0, 332.0,
            343.0, 554.0, 227.0,
            343.0, 554.0, 332.0
        ],
        vec![ 0, 1, 2, 3, 2, 1 ],
        emissive_material
    ));

    world
}
