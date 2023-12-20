mod point;
mod vector;
mod bounds;

pub use self::point::Point;
pub use self::vector::Vector;
pub use self::bounds::Bounds;

fn truncate(value: f32, decimal_places: u32) -> u64 {
    if decimal_places == 0 {
        return value as u64;
    }
    ((value * 10.0_f32.powi(decimal_places as i32 - 1)).round() * 10.0) as u64
}