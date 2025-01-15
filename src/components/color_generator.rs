extern crate rgb; // Add this to your dependencies in Cargo.toml: rgb = "0.8"

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let diff = max - min;

    let h = if diff == 0.0 {
        0.0
    } else if max == r {
        (g - b) / diff % 6.0
    } else if max == g {
        (b - r) / diff + 2.0
    } else {
        (r - g) / diff + 4.0
    };

    let h = 60.0 * h;
    let l = (max + min) / 2.0;
    let s = if diff == 0.0 {
        0.0
    } else {
        diff / (1.0 - (2.0 * l - 1.0).abs())
    };

    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

pub fn color_generator(color: &str) -> String {
    // Parse the input hex color
    let hex = &color[1..];
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    // Convert RGB to HSL
    let (h, s, l) = rgb_to_hsl(r, g, b);

    // Adjust the HSL values to generate the target color's HSL (hue stays, but lightness and saturation increase)
    let new_h = h; // Keep the hue same
    let new_s = s * 1.1; // Slightly increase saturation (can adjust further if needed)
    let new_l = l + 0.13; // Increase lightness

    // Convert the adjusted HSL back to RGB
    let (new_r, new_g, new_b) = hsl_to_rgb(new_h, new_s, new_l);

    // Return the new hex color
    format!("#{:02x}{:02x}{:02x}", new_r, new_g, new_b)
}
