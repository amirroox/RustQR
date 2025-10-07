use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use qrcode::{EcLevel, QrCode, Version};
use std::path::PathBuf;
use base64::{Engine as _, engine::general_purpose};

mod styles;
use styles::{DotStyle, EyeStyle, apply_dot_style, apply_eye_style, parse_gradient};

#[derive(Parser, Debug)]
#[command(name = "qrcode")]
#[command(about = "Generate QR codes with custom styling", long_about = None)]
struct Args {
    /// Text or URL to encode
    #[arg(short = 'd', long)]
    data: Option<String>,

    /// Output file path
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Output format (png, jpg)
    #[arg(short = 'f', long, default_value = "png")]
    format: String,

    /// Background color (hex format: #ffffff)
    #[arg(long, default_value = "#ffffff")]
    bg_color: String,

    /// Foreground color (hex format: #000000)
    #[arg(long, default_value = "#000000")]
    fg_color: String,

    /// Gradient colors (format: #ff0000,#0000ff)
    #[arg(short = 'g', long)]
    gradient: Option<String>,

    /// Dot style (square, circle, rounded)
    #[arg(long, default_value = "square")]
    dot_style: String,

    /// Eye style (square, circle, frame)
    #[arg(long, default_value = "square")]
    eye_style: String,

    /// Logo file path
    #[arg(short = 'l', long)]
    logo: Option<PathBuf>,

    /// Logo size ratio (0.0 to 1.0)
    #[arg(long, default_value = "0.2")]
    logo_size: f32,

    /// Error correction level (L, M, Q, H)
    #[arg(short, long, default_value = "M")]
    error: String,

    /// QR code size in pixels
    #[arg(short = 's', long, default_value = "300")]
    size: u32,

    /// Border size (quiet zone)
    #[arg(short = 'b', long, default_value = "4")]
    border: u32,

    /// Show QR in terminal
    #[arg(long)]
    show: bool,

    /// Copy to clipboard
    #[arg(long)]
    copy: bool,

    /// Base64 encode data before generating QR
    #[arg(long)]
    encode: bool,

    /// QR version (1-40)
    #[arg(short = 'v', long)]
    version: Option<i16>,

    /// Interactive mode
    #[arg(short = 'i', long)]
    interactive: bool,
}

fn main() -> Result<()> {
    let mut args = Args::parse();

    if args.interactive {
        run_interactive_mode(&mut args)?;
    }

    // Validate required data
    let data = args.data.as_ref().context("Data is required. Use --data or --interactive")?.clone();

    // Encode data if requested
    let final_data = if args.encode {
        general_purpose::STANDARD.encode(&data)
    } else {
        data
    };

    // Parse error correction level
    let ec_level = match args.error.to_uppercase().as_str() {
        "L" => EcLevel::L,
        "M" => EcLevel::M,
        "Q" => EcLevel::Q,
        "H" => EcLevel::H,
        _ => EcLevel::M,
    };

    // Create QR code
    let qr = if let Some(v) = args.version {
        QrCode::with_version(&final_data, Version::Normal(v), ec_level)
            .context("Failed to create QR code with specified version")?
    } else {
        QrCode::with_error_correction_level(&final_data, ec_level)
            .context("Failed to create QR code")?
    };

    // Show in terminal if requested
    if args.show {
        print_qr_terminal(&qr);
    }

    // Generate image
    let img = generate_qr_image(&qr, &args)?;

    // Save to file
    let output_path = args.output.unwrap_or_else(|| PathBuf::from("qrcode.png"));
    img.save(&output_path)
        .context("Failed to save QR code image")?;

    println!("✓ QR code saved to: {}", output_path.display());

    // Copy to clipboard if requested
    if args.copy {
        match cli_clipboard::set_contents(output_path.to_string_lossy().to_string()) {
            Ok(_) => println!("✓ Path copied to clipboard"),
            Err(e) => eprintln!("⚠ Failed to copy to clipboard: {}", e),
        }
    }

    Ok(())
}

fn run_interactive_mode(args: &mut Args) -> Result<()> {
    let theme = ColorfulTheme::default();

    // Get data
    if args.data.is_none() {
        args.data = Some(Input::with_theme(&theme)
            .with_prompt("Enter text or URL")
            .interact_text()?);
    }

    // Get foreground color
    let fg: String = Input::with_theme(&theme)
        .with_prompt("Foreground color (hex)")
        .default("#000000".to_string())
        .interact_text()?;
    args.fg_color = fg;

    // Get background color
    let bg: String = Input::with_theme(&theme)
        .with_prompt("Background color (hex)")
        .default("#ffffff".to_string())
        .interact_text()?;
    args.bg_color = bg;

    // Gradient option
    if Confirm::with_theme(&theme)
        .with_prompt("Use gradient?")
        .default(false)
        .interact()?
    {
        let gradient: String = Input::with_theme(&theme)
            .with_prompt("Gradient colors (format: #ff0000,#0000ff)")
            .interact_text()?;
        args.gradient = Some(gradient);
    }

    // Dot style
    let dot_styles = vec!["square", "circle", "rounded"];
    let dot_idx = Select::with_theme(&theme)
        .with_prompt("Dot style")
        .default(0)
        .items(&dot_styles)
        .interact()?;
    args.dot_style = dot_styles[dot_idx].to_string();

    // Eye style
    let eye_styles = vec!["square", "circle", "frame"];
    let eye_idx = Select::with_theme(&theme)
        .with_prompt("Eye style")
        .default(0)
        .items(&eye_styles)
        .interact()?;
    args.eye_style = eye_styles[eye_idx].to_string();

    // Logo
    if Confirm::with_theme(&theme)
        .with_prompt("Add logo?")
        .default(false)
        .interact()?
    {
        let logo_path: String = Input::with_theme(&theme)
            .with_prompt("Logo path")
            .interact_text()?;
        args.logo = Some(PathBuf::from(logo_path));

        let logo_size: f32 = Input::with_theme(&theme)
            .with_prompt("Logo size ratio (0.1 to 0.3)")
            .default(0.2)
            .interact_text()?;
        args.logo_size = logo_size;
    }

    // Error correction
    let ec_levels = vec!["L", "M", "Q", "H"];
    let ec_idx = Select::with_theme(&theme)
        .with_prompt("Error correction level")
        .default(1)
        .items(&ec_levels)
        .interact()?;
    args.error = ec_levels[ec_idx].to_string();

    // Size
    let size: u32 = Input::with_theme(&theme)
        .with_prompt("Image size (pixels)")
        .default(300)
        .interact_text()?;
    args.size = size;

    // Output path
    let output: String = Input::with_theme(&theme)
        .with_prompt("Output file path")
        .default("qrcode.png".to_string())
        .interact_text()?;
    args.output = Some(PathBuf::from(output));

    Ok(())
}

fn generate_qr_image(qr: &QrCode, args: &Args) -> Result<DynamicImage> {
    let qr_width = qr.width();
    let scale = args.size / (qr_width as u32 + 2 * args.border);
    let img_size = (qr_width as u32 + 2 * args.border) * scale;

    // Parse colors
    let bg_color = parse_color(&args.bg_color)?;
    let fg_color = parse_color(&args.fg_color)?;

    // Check for gradient
    let gradient_colors = if let Some(ref g) = args.gradient {
        Some(parse_gradient(g)?)
    } else {
        None
    };

    // Create image
    let mut img: RgbaImage = ImageBuffer::from_pixel(img_size, img_size, bg_color);

    // Parse styles
    let dot_style = DotStyle::from_str(&args.dot_style);
    let eye_style = EyeStyle::from_str(&args.eye_style);

    // Find eye positions (0,0), (qr_width-7, 0), (0, qr_width-7)
    let eye_positions = vec![
        (0, 0),
        (qr_width - 7, 0),
        (0, qr_width - 7),
    ];

    // Draw QR code with styles
    for y in 0..qr_width {
        for x in 0..qr_width {
            if qr[(x, y)] == qrcode::Color::Dark {
                let color = if let Some(ref grad) = gradient_colors {
                    interpolate_gradient(grad, x as f32 / qr_width as f32)
                } else {
                    fg_color
                };

                // Check if in eye area
                let in_eye = eye_positions.iter().any(|(ex, ey)| {
                    x >= *ex && x < ex + 7 && y >= *ey && y < ey + 7
                });

                let px = (x as u32 + args.border) * scale;
                let py = (y as u32 + args.border) * scale;

                if in_eye {
                    apply_eye_style(&mut img, px, py, scale, color, &eye_style);
                } else {
                    apply_dot_style(&mut img, px, py, scale, color, &dot_style);
                }
            }
        }
    }

    // Add logo if provided
    if let Some(ref logo_path) = args.logo {
        add_logo(&mut img, logo_path, args.logo_size)?;
    }

    Ok(DynamicImage::ImageRgba8(img))
}

fn parse_color(hex: &str) -> Result<Rgba<u8>> {
    let color = csscolorparser::parse(hex)
        .context("Invalid color format")?;
    Ok(Rgba([
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
        (color.a * 255.0) as u8,
    ]))
}

fn interpolate_gradient(colors: &(Rgba<u8>, Rgba<u8>), t: f32) -> Rgba<u8> {
    let (c1, c2) = colors;
    Rgba([
        (c1[0] as f32 + (c2[0] as f32 - c1[0] as f32) * t) as u8,
        (c1[1] as f32 + (c2[1] as f32 - c1[1] as f32) * t) as u8,
        (c1[2] as f32 + (c2[2] as f32 - c1[2] as f32) * t) as u8,
        255,
    ])
}

fn add_logo(img: &mut RgbaImage, logo_path: &PathBuf, size_ratio: f32) -> Result<()> {
    let logo = image::open(logo_path)
        .context("Failed to open logo file")?
        .to_rgba8();

    let img_size = img.width();
    let logo_size = (img_size as f32 * size_ratio.clamp(0.1, 0.4)) as u32;
    let logo = image::imageops::resize(&logo, logo_size, logo_size, image::imageops::FilterType::Lanczos3);

    let offset_x = (img_size - logo_size) / 2;
    let offset_y = (img_size - logo_size) / 2;

    // Add white background behind logo
    for y in 0..logo_size + 20 {
        for x in 0..logo_size + 20 {
            let px = offset_x as i32 + x as i32 - 10;
            let py = offset_y as i32 + y as i32 - 10;
            if px >= 0 && py >= 0 && px < img_size as i32 && py < img_size as i32 {
                img.put_pixel(px as u32, py as u32, Rgba([255, 255, 255, 255]));
            }
        }
    }

    image::imageops::overlay(img, &logo, offset_x as i64, offset_y as i64);
    Ok(())
}

fn print_qr_terminal(qr: &QrCode) {
    let width = qr.width();
    println!("\nQR Code:");
    for y in 0..width {
        for x in 0..width {
            let c = if qr[(x, y)] == qrcode::Color::Dark {
                "██"
            } else {
                "  "
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}