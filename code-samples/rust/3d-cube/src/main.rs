use image::{ImageBuffer, Rgb};
use cgmath::{Matrix4, Point3, Rad, Vector4};
use std::{fs::File, path::Path};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BACKGROUND_COLOR: Rgb<u8> = Rgb([255, 255, 255]);
const CUBE_COLOR: Rgb<u8> = Rgb([0, 0, 0]);

fn main() {
    // Пример углов поворота в радианах
    let angle_x = Rad(10.0_f32.to_radians());
    let angle_y = Rad(10.0_f32.to_radians());
    let angle_z = Rad(0.0_f32.to_radians());

    // Определяем вершины куба
    let vertices = [
        Point3::new(-1.0, -1.0, -1.0),
        Point3::new(1.0, -1.0, -1.0),
        Point3::new(1.0, 1.0, -1.0),
        Point3::new(-1.0, 1.0, -1.0),
        Point3::new(-1.0, -1.0, 1.0),
        Point3::new(1.0, -1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(-1.0, 1.0, 1.0),
    ];

    // Определяем связи между вершинами
    let lines = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    // Определяем матрицы поворота
    let rotation_x = Matrix4::from_angle_x(angle_x);
    let rotation_y = Matrix4::from_angle_y(angle_y);
    let rotation_z = Matrix4::from_angle_z(angle_z);

    // Итоговая матрица поворота
    let rotation = rotation_z * rotation_y * rotation_x;

    // Центральная точка изображения
    let center = Point3::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 0.0);

    // Создаем изображение
    let mut img = ImageBuffer::from_pixel(WIDTH, HEIGHT, BACKGROUND_COLOR);

    // Проецируем вершины куба и рисуем линии
    for &(start, end) in &lines {
        let p0 = transform_point(&rotation, vertices[start]);
        let p1 = transform_point(&rotation, vertices[end]);

        draw_line(
            &mut img,
            project_point(p0, center),
            project_point(p1, center),
            CUBE_COLOR,
        );
    }

    // Сохраняем изображение как JPEG
    img.save(Path::new("cube.jpg")).unwrap();
}

// Преобразование точки через матрицу
fn transform_point(matrix: &Matrix4<f32>, point: Point3<f32>) -> Point3<f32> {
    let vector = Vector4::new(point.x, point.y, point.z, 1.0);
    let transformed = matrix * vector;
    Point3::new(transformed.x, transformed.y, transformed.z)
}

fn project_point(p: Point3<f32>, center: Point3<f32>) -> Point3<f32> {
    // Простая перспективная проекция
    Point3::new(center.x + p.x * 100.0, center.y - p.y * 100.0, p.z)
}

fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p0: Point3<f32>, p1: Point3<f32>, color: Rgb<u8>) {
    let (x0, y0) = (p0.x as i32, p0.y as i32);
    let (x1, y1) = (p1.x as i32, p1.y as i32);

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);
    let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });

    loop {
        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
