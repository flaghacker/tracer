use image::ImageBuffer;
use imgref::ImgRef;
use wavefront_obj::obj;
use wavefront_obj::obj::Primitive;

use crate::common::math::{Point3, Transform};
use crate::common::scene::{Color, Material, Object, Shape};

type Image = ImageBuffer<image::Rgb<u8>, Vec<u8>>;

/// Convert the given image to a format suitable for saving to a file.
/// The first return Image is the image itself, the second Image shows where values had to be clipped
/// to fit into the image format .
pub fn to_image(image: ImgRef<Color>) -> (Image, Image) {
    let mut result: Image = ImageBuffer::new(image.width() as u32, image.height() as u32);
    let mut clipped: Image = ImageBuffer::new(image.width() as u32, image.height() as u32);

    let max = palette::Srgb::new(1.0, 1.0, 1.0).into_linear();

    for (x, y, p) in result.enumerate_pixels_mut() {
        let linear: Color = image[(x, y)];

        let srgb = palette::Srgb::from_linear(linear);
        let data = srgb.into_format();

        *p = image::Rgb([data.red, data.green, data.blue]);
        clipped[(x, y)] = image::Rgb([
            if linear.red > max.red { 255 } else { 0 },
            if linear.green > max.green { 255 } else { 0 },
            if linear.blue > max.blue { 255 } else { 0 },
        ]);
    }

    return (result, clipped);
}

fn vertex_to_point(vertex: &obj::Vertex) -> Point3 {
    Point3::new(vertex.x as f32, vertex.y as f32, vertex.z as f32)
}

pub fn triangle_as_transform(a: Point3, b: Point3, c: Point3) -> Transform {
    println!("Triangle with points {:?}, {:?}, {:?}", a, b, c);

    let db = b - a;
    let dc = c - a;
    let _n = -db.cross(dc);

    todo!()

    /*let transform = Transform::from_matrix_unchecked(Matrix4::new(
        db.x, dc.x, n.x, a.x,
        db.y, dc.y, n.y, a.y,
        db.z, dc.z, n.z, a.z,
        0.0, 0.0, 0.0, 1.0,
    ));*/

    // transform
}

pub fn obj_to_triangles(obj: &obj::Object, material: Material, transform: Transform) -> impl Iterator<Item=Object> + '_ {
    obj.geometry.iter().flat_map(move |geometry|
        geometry.shapes.iter().filter_map(move |shape| {
            match shape.primitive {
                Primitive::Point(_) => None,
                Primitive::Line(_, _) => None,
                Primitive::Triangle((avi, _, _), (bvi, ..), (cvi, ..)) => {
                    let a = vertex_to_point(&obj.vertices[avi]);
                    let b = vertex_to_point(&obj.vertices[bvi]);
                    let c = vertex_to_point(&obj.vertices[cvi]);

                    let local_transform = triangle_as_transform(a, b, c);

                    Some(Object {
                        shape: Shape::Triangle,
                        material,
                        transform: transform * local_transform,
                    })
                }
            }
        })
    )
}
