use anyhow::{Context, Result};
use image::{Rgba, RgbaImage};

pub enum DotStyle {
    Square,
    Circle,
    Rounded,
}

pub enum EyeStyle {
    Square,
    Circle,
    Frame,
}

impl DotStyle {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "circle" => DotStyle::Circle,
            "rounded" => DotStyle::Rounded,
            _ => DotStyle::Square,
        }
    }
}

impl EyeStyle {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "circle" => EyeStyle::Circle,
            "frame" => EyeStyle::Frame,
            _ => EyeStyle::Square,
        }
    }
}

pub fn apply_dot_style(
    img: &mut RgbaImage,
    x: u32,
    y: u32,
    scale: u32,
    color: Rgba<u8>,
    style: &DotStyle,
) {
    match style {
        DotStyle::Square => draw_square(img, x, y, scale, color),
        DotStyle::Circle => draw_circle(img, x, y, scale, color),
        DotStyle::Rounded => draw_rounded_square(img, x, y, scale, color),
    }
}

pub fn apply_eye_style(
    img: &mut RgbaImage,
    x: u32,
    y: u32,
    scale: u32,
    color: Rgba<u8>,
    style: &EyeStyle,
) {
    match style {
        EyeStyle::Square => draw_square(img, x, y, scale, color),
        EyeStyle::Circle => draw_circle(img, x, y, scale, color),
        EyeStyle::Frame => draw_frame(img, x, y, scale, color),
    }
}

fn draw_square(img: &mut RgbaImage, x: u32, y: u32, scale: u32, color: Rgba<u8>) {
    for dy in 0..scale {
        for dx in 0..scale {
            let px = x + dx;
            let py = y + dy;
            if px < img.width() && py < img.height() {
                img.put_pixel(px, py, color);
            }
        }
    }
}

fn draw_circle(img: &mut RgbaImage, x: u32, y: u32, scale: u32, color: Rgba<u8>) {
    let center_x = x as f32 + scale as f32 / 2.0;
    let center_y = y as f32 + scale as f32 / 2.0;
    let radius = scale as f32 / 2.0;

    for dy in 0..scale {
        for dx in 0..scale {
            let px = x + dx;
            let py = y + dy;

            if px < img.width() && py < img.height() {
                let dist = ((px as f32 - center_x).powi(2) + (py as f32 - center_y).powi(2)).sqrt();
                if dist <= radius {
                    img.put_pixel(px, py, color);
                }
            }
        }
    }
}

fn draw_rounded_square(img: &mut RgbaImage, x: u32, y: u32, scale: u32, color: Rgba<u8>) {
    let corner_radius = scale as f32 * 0.3;

    for dy in 0..scale {
        for dx in 0..scale {
            let px = x + dx;
            let py = y + dy;

            if px < img.width() && py < img.height() {
                let in_corner = is_in_rounded_corner(dx, dy, scale, corner_radius);
                if in_corner {
                    img.put_pixel(px, py, color);
                }
            }
        }
    }
}

fn draw_frame(img: &mut RgbaImage, x: u32, y: u32, scale: u32, color: Rgba<u8>) {
    let thickness = (scale as f32 * 0.2).max(1.0) as u32;

    for dy in 0..scale {
        for dx in 0..scale {
            let px = x + dx;
            let py = y + dy;

            if px < img.width() && py < img.height() {
                if dx < thickness || dx >= scale - thickness || dy < thickness || dy >= scale - thickness {
                    img.put_pixel(px, py, color);
                }
            }
        }
    }
}

fn is_in_rounded_corner(dx: u32, dy: u32, scale: u32, radius: f32) -> bool {
    let corners = [
        (radius, radius),                           // top-left
        (scale as f32 - radius, radius),           // top-right
        (radius, scale as f32 - radius),           // bottom-left
        (scale as f32 - radius, scale as f32 - radius), // bottom-right
    ];

    let dx_f = dx as f32;
    let dy_f = dy as f32;

    // Check if in corner region
    let in_corner_region = (dx_f < radius && dy_f < radius)
        || (dx_f > scale as f32 - radius && dy_f < radius)
        || (dx_f < radius && dy_f > scale as f32 - radius)
        || (dx_f > scale as f32 - radius && dy_f > scale as f32 - radius);

    if !in_corner_region {
        return true;
    }

    // Check distance from nearest corner
    for (cx, cy) in &corners {
        let dist = ((dx_f - cx).powi(2) + (dy_f - cy).powi(2)).sqrt();
        if dist <= radius {
            return true;
        }
    }

    false
}

pub fn parse_gradient(gradient: &str) -> Result<(Rgba<u8>, Rgba<u8>)> {
    let parts: Vec<&str> = gradient.split(',').collect();
    if parts.len() != 2 {
        anyhow::bail!("Gradient must have exactly 2 colors");
    }

    let c1 = parse_color_from_hex(parts[0].trim())?;
    let c2 = parse_color_from_hex(parts[1].trim())?;

    Ok((c1, c2))
}

fn parse_color_from_hex(hex: &str) -> Result<Rgba<u8>> {
    let color = csscolorparser::parse(hex)
        .context("Invalid color format")?;
    Ok(Rgba([
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
        (color.a * 255.0) as u8,
    ]))
}