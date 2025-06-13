use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ValueError, ValueResult};

/// Color value supporting multiple formats and color spaces with rich
/// functionality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorValue {
    /// RGBA values (0-255 each)
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Custom serialization implementation
#[cfg(feature = "serde")]
impl Serialize for ColorValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.to_hex_lowercase().serialize(serializer)
    }
}

// Custom deserialization implementation
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ColorValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        ColorValue::parse(s).map_err(serde::de::Error::custom)
    }
}

impl ColorValue {
    // === Construction ===

    /// Creates a new color from RGBA values (0-255)
    #[inline]
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new color from RGB values with full opacity
    #[inline]
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// Creates a new color from RGBA values
    #[inline]
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(r, g, b, a)
    }

    /// Creates a color from normalized float values (0.0-1.0)
    pub fn from_floats(r: f32, g: f32, b: f32, a: f32) -> ValueResult<Self> {
        if ![r, g, b, a].iter().all(|&v| (0.0..=1.0).contains(&v)) {
            return Err(ValueError::custom("Color float values must be between 0.0 and 1.0"));
        }

        Ok(Self::new(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        ))
    }

    /// Creates a color from normalized float RGB with full opacity
    pub fn from_rgb_floats(r: f32, g: f32, b: f32) -> ValueResult<Self> {
        Self::from_floats(r, g, b, 1.0)
    }

    /// Creates a color from HSL values (H: 0-360, S: 0-100, L: 0-100)
    pub fn from_hsl(h: f32, s: f32, l: f32) -> ValueResult<Self> {
        Self::from_hsla(h, s, l, 1.0)
    }

    /// Creates a color from HSLA values (H: 0-360, S: 0-100, L: 0-100, A: 0-1)
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> ValueResult<Self> {
        if !(0.0..=360.0).contains(&h) {
            return Err(ValueError::custom("Hue must be between 0 and 360"));
        }
        if ![s, l].iter().all(|&v| (0.0..=100.0).contains(&v)) {
            return Err(ValueError::custom("Saturation and Lightness must be between 0 and 100"));
        }
        if !(0.0..=1.0).contains(&a) {
            return Err(ValueError::custom("Alpha must be between 0.0 and 1.0"));
        }

        let (r, g, b) = Self::hsl_to_rgb(h, s / 100.0, l / 100.0);
        Ok(Self::rgba(r, g, b, (a * 255.0).round() as u8))
    }

    /// Creates a color from HSV values (H: 0-360, S: 0-100, V: 0-100)
    pub fn from_hsv(h: f32, s: f32, v: f32) -> ValueResult<Self> {
        Self::from_hsva(h, s, v, 1.0)
    }

    /// Creates a color from HSVA values (H: 0-360, S: 0-100, V: 0-100, A: 0-1)
    pub fn from_hsva(h: f32, s: f32, v: f32, a: f32) -> ValueResult<Self> {
        if !(0.0..=360.0).contains(&h) {
            return Err(ValueError::custom("Hue must be between 0 and 360"));
        }
        if ![s, v].iter().all(|&v| (0.0..=100.0).contains(&v)) {
            return Err(ValueError::custom("Saturation and Value must be between 0 and 100"));
        }
        if !(0.0..=1.0).contains(&a) {
            return Err(ValueError::custom("Alpha must be between 0.0 and 1.0"));
        }

        let (r, g, b) = Self::hsv_to_rgb(h, s / 100.0, v / 100.0);
        Ok(Self::rgba(r, g, b, (a * 255.0).round() as u8))
    }

    /// Creates a color from a hex string (#RGB, #RGBA, #RRGGBB, #RRGGBBAA)
    pub fn from_hex(hex: impl AsRef<str>) -> ValueResult<Self> {
        let hex = hex.as_ref().trim();

        // Remove # prefix if present
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        match hex.len() {
            3 => {
                // #RGB -> #RRGGBB
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                Ok(Self::rgb(r, g, b))
            },
            4 => {
                // #RGBA -> #RRGGBBAA
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                Ok(Self::rgba(r, g, b, a))
            },
            6 => {
                // #RRGGBB
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                Ok(Self::rgb(r, g, b))
            },
            8 => {
                // #RRGGBBAA
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                let a = u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|_| ValueError::custom("Invalid hex color format"))?;
                Ok(Self::rgba(r, g, b, a))
            },
            _ => Err(ValueError::custom("Hex color must be 3, 4, 6, or eight characters long")),
        }
    }

    /// Parse color from various string formats
    pub fn parse(input: impl AsRef<str>) -> ValueResult<Self> {
        let input = input.as_ref().trim();

        // Try hex format first
        if input.starts_with('#') {
            return Self::from_hex(input);
        }

        // Try named colors
        if let Some(color) = Self::from_name(input) {
            return Ok(color);
        }

        Err(ValueError::custom(format!(
            "Unknown color format: '{}'. Supported formats: hex (#RGB, #RRGGBB, #RRGGBBAA) and named colors.",
            input
        )))
    }

    // === Named Colors ===

    /// Get color from standard color name
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "transparent" => Some(Self::rgba(0, 0, 0, 0)),
            "black" => Some(Self::rgb(0, 0, 0)),
            "white" => Some(Self::rgb(255, 255, 255)),
            "red" => Some(Self::rgb(255, 0, 0)),
            "green" => Some(Self::rgb(0, 128, 0)),
            "blue" => Some(Self::rgb(0, 0, 255)),
            "yellow" => Some(Self::rgb(255, 255, 0)),
            "cyan" => Some(Self::rgb(0, 255, 255)),
            "magenta" => Some(Self::rgb(255, 0, 255)),
            "orange" => Some(Self::rgb(255, 165, 0)),
            "purple" => Some(Self::rgb(128, 0, 128)),
            "pink" => Some(Self::rgb(255, 192, 203)),
            "brown" => Some(Self::rgb(165, 42, 42)),
            "gray" | "grey" => Some(Self::rgb(128, 128, 128)),
            "lightgray" | "lightgrey" => Some(Self::rgb(211, 211, 211)),
            "darkgray" | "darkgrey" => Some(Self::rgb(169, 169, 169)),
            "lime" => Some(Self::rgb(0, 255, 0)),
            "navy" => Some(Self::rgb(0, 0, 128)),
            "maroon" => Some(Self::rgb(128, 0, 0)),
            "olive" => Some(Self::rgb(128, 128, 0)),
            "silver" => Some(Self::rgb(192, 192, 192)),
            "teal" => Some(Self::rgb(0, 128, 128)),
            "aqua" => Some(Self::rgb(0, 255, 255)),
            "fuchsia" => Some(Self::rgb(255, 0, 255)),
            _ => None,
        }
    }

    // === Common Color Constants ===

    /// Transparent color (0, 0, 0, 0)
    #[must_use]
    pub const fn transparent() -> Self {
        Self::rgba(0, 0, 0, 0)
    }

    /// Black color
    #[must_use]
    pub const fn black() -> Self {
        Self::rgb(0, 0, 0)
    }

    /// White color
    #[must_use]
    pub const fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    /// Red color
    #[must_use]
    pub const fn red() -> Self {
        Self::rgb(255, 0, 0)
    }

    /// Green color
    #[must_use]
    pub const fn green() -> Self {
        Self::rgb(0, 128, 0)
    }

    /// Blue color
    #[must_use]
    pub const fn blue() -> Self {
        Self::rgb(0, 0, 255)
    }

    /// Yellow color
    #[must_use]
    pub const fn yellow() -> Self {
        Self::rgb(255, 255, 0)
    }

    /// Cyan color
    #[must_use]
    pub const fn cyan() -> Self {
        Self::rgb(0, 255, 255)
    }

    /// Magenta color
    #[must_use]
    pub const fn magenta() -> Self {
        Self::rgb(255, 0, 255)
    }

    // === Color Information ===

    /// Returns true if the color is fully transparent
    #[inline]
    #[must_use]
    pub const fn is_transparent(&self) -> bool {
        self.a == 0
    }

    /// Returns true if the color is fully opaque
    #[inline]
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        self.a == 255
    }

    /// Returns true if the color is grayscale
    #[inline]
    #[must_use]
    pub const fn is_grayscale(&self) -> bool {
        self.r == self.g && self.g == self.b
    }

    /// Calculate the relative luminance (0.0-1.0)
    #[must_use]
    pub fn luminance(&self) -> f32 {
        let r = (self.r as f32 / 255.0).powf(2.2);
        let g = (self.g as f32 / 255.0).powf(2.2);
        let b = (self.b as f32 / 255.0).powf(2.2);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Check if the color is considered "dark" (luminance < 0.5)
    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.luminance() < 0.5
    }

    /// Check if the color is considered "light" (luminance >= 0.5)
    #[must_use]
    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }

    // === Color Conversion ===

    /// Convert to normalized float values (0.0-1.0)
    #[must_use]
    pub fn to_floats(&self) -> (f32, f32, f32, f32) {
        (self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0)
    }

    /// Convert to RGB float values (0.0-1.0), ignoring alpha
    #[must_use]
    pub fn to_rgb_floats(&self) -> (f32, f32, f32) {
        (self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0)
    }

    /// Convert to HSL values (H: 0-360, S: 0-100, L: 0-100)
    #[must_use]
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        Self::rgb_to_hsl(self.r, self.g, self.b)
    }

    /// Convert to HSLA values (H: 0-360, S: 0-100, L: 0-100, A: 0-1)
    #[must_use]
    pub fn to_hsla(&self) -> (f32, f32, f32, f32) {
        let (h, s, l) = self.to_hsl();
        (h, s, l, self.a as f32 / 255.0)
    }

    /// Convert to HSV values (H: 0-360, S: 0-100, V: 0-100)
    #[must_use]
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        Self::rgb_to_hsv(self.r, self.g, self.b)
    }

    /// Convert to HSVA values (H: 0-360, S: 0-100, V: 0-100, A: 0-1)
    #[must_use]
    pub fn to_hsva(&self) -> (f32, f32, f32, f32) {
        let (h, s, v) = self.to_hsv();
        (h, s, v, self.a as f32 / 255.0)
    }

    // === Formatting ===

    /// Convert to hex string (#RRGGBB or #RRGGBBAA if alpha < 255)
    #[must_use]
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        }
    }

    /// Convert to lowercase hex string
    #[must_use]
    pub fn to_hex_lowercase(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }

    // === Color Manipulation ===

    /// Create a new color with modified alpha
    #[inline]
    #[must_use]
    pub const fn with_alpha(&self, alpha: u8) -> Self {
        Self::rgba(self.r, self.g, self.b, alpha)
    }

    /// Create a new color with modified alpha (0.0-1.0)
    #[must_use]
    pub fn with_alpha_f32(&self, alpha: f32) -> Self {
        self.with_alpha((alpha.clamp(0.0, 1.0) * 255.0).round() as u8)
    }

    /// Lighten the color by a percentage (0.0-1.0)
    #[must_use]
    pub fn lighten(&self, amount: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l + amount * 100.0).clamp(0.0, 100.0);
        Self::from_hsla(h, s, new_l, self.a as f32 / 255.0).unwrap_or(*self)
    }

    /// Darken the color by a percentage (0.0-1.0)
    #[must_use]
    pub fn darken(&self, amount: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l - amount * 100.0).clamp(0.0, 100.0);
        Self::from_hsla(h, s, new_l, self.a as f32 / 255.0).unwrap_or(*self)
    }

    /// Saturate the color by a percentage (0.0-1.0)
    #[must_use]
    pub fn saturate(&self, amount: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_s = (s + amount * 100.0).clamp(0.0, 100.0);
        Self::from_hsla(h, new_s, l, self.a as f32 / 255.0).unwrap_or(*self)
    }

    /// Desaturate the color by a percentage (0.0-1.0)
    #[must_use]
    pub fn desaturate(&self, amount: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_s = (s - amount * 100.0).clamp(0.0, 100.0);
        Self::from_hsla(h, new_s, l, self.a as f32 / 255.0).unwrap_or(*self)
    }

    /// Convert to grayscale
    #[must_use]
    pub fn grayscale(&self) -> Self {
        let gray =
            (0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32).round() as u8;
        Self::rgba(gray, gray, gray, self.a)
    }

    /// Invert the color (keeping alpha)
    #[must_use]
    pub const fn invert(&self) -> Self {
        Self::rgba(255 - self.r, 255 - self.g, 255 - self.b, self.a)
    }

    /// Mix with another color
    #[must_use]
    pub fn mix(&self, other: &Self, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let inv_ratio = 1.0 - ratio;

        Self::rgba(
            (self.r as f32 * inv_ratio + other.r as f32 * ratio).round() as u8,
            (self.g as f32 * inv_ratio + other.g as f32 * ratio).round() as u8,
            (self.b as f32 * inv_ratio + other.b as f32 * ratio).round() as u8,
            (self.a as f32 * inv_ratio + other.a as f32 * ratio).round() as u8,
        )
    }

    // === Color Harmony ===

    /// Get the complementary color (opposite on color wheel)
    #[must_use]
    pub fn complement(&self) -> Self {
        let (h, s, l) = self.to_hsl();
        let comp_h = (h + 180.0) % 360.0;
        Self::from_hsla(comp_h, s, l, self.a as f32 / 255.0).unwrap_or(*self)
    }

    /// Get triadic colors (120° apart on color wheel)
    #[must_use]
    pub fn triadic(&self) -> (Self, Self) {
        let (h, s, l) = self.to_hsl();
        let alpha = self.a as f32 / 255.0;
        let color1 = Self::from_hsla((h + 120.0) % 360.0, s, l, alpha).unwrap_or(*self);
        let color2 = Self::from_hsla((h + 240.0) % 360.0, s, l, alpha).unwrap_or(*self);
        (color1, color2)
    }

    /// Get analogous colors (±30° on color wheel)
    #[must_use]
    pub fn analogous(&self) -> (Self, Self) {
        let (h, s, l) = self.to_hsl();
        let alpha = self.a as f32 / 255.0;
        let color1 = Self::from_hsla((h + 30.0) % 360.0, s, l, alpha).unwrap_or(*self);
        let color2 = Self::from_hsla((h - 30.0 + 360.0) % 360.0, s, l, alpha).unwrap_or(*self);
        (color1, color2)
    }

    // === Validation Methods ===

    /// Check if the color is suitable for text (good contrast)
    #[must_use]
    pub fn is_suitable_for_text(&self, background: &Self) -> bool {
        self.contrast_ratio(background) >= 4.5 // WCAG AA standard
    }

    /// Calculate contrast ratio with another color (1.0-21.0)
    #[must_use]
    pub fn contrast_ratio(&self, other: &Self) -> f32 {
        let l1 = self.luminance();
        let l2 = other.luminance();
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Check if color is web-safe (216 web-safe colors)
    #[must_use]
    pub fn is_web_safe(&self) -> bool {
        [self.r, self.g, self.b].iter().all(|&c| c % 51 == 0)
    }

    // === Helper Methods for Color Space Conversion ===

    /// Convert hue sector and chroma values to RGB components
    fn hue_to_rgb_components(h_sector: f32, c: f32, x: f32) -> (f32, f32, f32) {
        match h_sector.floor() as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        }
    }

    /// Convert float RGB components to u8 values
    fn rgb_floats_to_u8(r: f32, g: f32, b: f32) -> (u8, u8, u8) {
        ((r * 255.0).round() as u8, (g * 255.0).round() as u8, (b * 255.0).round() as u8)
    }

    fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
        let h_sector = h / 60.0;
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - (h_sector % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = Self::hue_to_rgb_components(h_sector, c, x);
        Self::rgb_floats_to_u8(r + m, g + m, b + m)
    }

    fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let l = (max + min) / 2.0;

        if delta == 0.0 {
            return (0.0, 0.0, l * 100.0);
        }

        let s = if l < 0.5 { delta / (max + min) } else { delta / (2.0 - max - min) };

        let h = Self::calculate_hue(r, g, b, max, delta);

        (h, s * 100.0, l * 100.0)
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        let h_sector = h / 60.0;
        let c = v * s;
        let x = c * (1.0 - (h_sector % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = Self::hue_to_rgb_components(h_sector, c, x);
        Self::rgb_floats_to_u8(r + m, g + m, b + m)
    }

    fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let v = max;

        if delta == 0.0 {
            return (0.0, 0.0, v * 100.0);
        }

        let s = delta / max;
        let h = Self::calculate_hue(r, g, b, max, delta);

        (h, s * 100.0, v * 100.0)
    }

    /// Calculate a hue component (shared between HSL and HSV conversion)
    fn calculate_hue(r: f32, g: f32, b: f32, max: f32, delta: f32) -> f32 {
        if max == r {
            ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
        } else if max == g {
            ((b - r) / delta + 2.0) * 60.0
        } else {
            ((r - g) / delta + 4.0) * 60.0
        }
    }
}

// === Default Implementation ===

impl Default for ColorValue {
    #[inline]
    fn default() -> Self {
        Self::transparent()
    }
}

// === Display Implementation ===

impl fmt::Display for ColorValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Pretty format with RGBA notation
            if self.a == 255 {
                write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
            } else {
                write!(f, "rgba({}, {}, {}, {:.3})", self.r, self.g, self.b, self.a as f32 / 255.0)
            }
        } else {
            // Hex format
            write!(f, "{}", self.to_hex_lowercase())
        }
    }
}

// === FromStr Implementation ===

impl FromStr for ColorValue {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

// === TryFrom Implementations ===

impl TryFrom<&str> for ColorValue {
    type Error = ValueError;

    fn try_from(color: &str) -> Result<Self, Self::Error> {
        Self::parse(color)
    }
}

impl TryFrom<String> for ColorValue {
    type Error = ValueError;

    fn try_from(color: String) -> Result<Self, Self::Error> {
        Self::parse(color)
    }
}

// === JSON Conversion ===
#[cfg(feature = "json")]
impl From<ColorValue> for serde_json::Value {
    fn from(color: ColorValue) -> Self {
        serde_json::Value::String(color.to_hex_lowercase())
    }
}
#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for ColorValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::String(color_str) => Self::parse(color_str),
            serde_json::Value::Array(arr) if arr.len() == 3 || arr.len() == 4 => {
                let r = arr[0]
                    .as_u64()
                    .and_then(|v| if v <= 255 { Some(v as u8) } else { None })
                    .ok_or_else(|| ValueError::custom("Invalid red value in a color array"))?;
                let g = arr[1]
                    .as_u64()
                    .and_then(|v| if v <= 255 { Some(v as u8) } else { None })
                    .ok_or_else(|| {
                    ValueError::custom("Invalid green value in a color array")
                })?;
                let b = arr[2]
                    .as_u64()
                    .and_then(|v| if v <= 255 { Some(v as u8) } else { None })
                    .ok_or_else(|| ValueError::custom("Invalid blue value in a color array"))?;

                let a = if arr.len() == 4 {
                    arr[3]
                        .as_u64()
                        .and_then(|v| if v <= 255 { Some(v as u8) } else { None })
                        .or_else(|| {
                            arr[3].as_f64().map(|f| (f.clamp(0.0, 1.0) * 255.0).round() as u8)
                        })
                        .ok_or_else(|| ValueError::custom("Invalid alpha value in color array"))?
                } else {
                    255
                };

                Ok(ColorValue::rgba(r, g, b, a))
            },
            other => Err(ValueError::type_conversion_with_value(
                format!("{:?}", other),
                "ColorValue",
                "expected string, or RGB/RGBA array".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let color = ColorValue::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);

        let transparent = ColorValue::transparent();
        assert!(transparent.is_transparent());

        let white = ColorValue::white();
        assert!(white.is_opaque());
    }

    #[test]
    fn test_hex_parsing() {
        assert_eq!(ColorValue::from_hex("#FF0000").unwrap(), ColorValue::red());
        assert_eq!(ColorValue::from_hex("00FF00").unwrap(), ColorValue::rgb(0, 255, 0));
        assert_eq!(ColorValue::from_hex("#F0A").unwrap(), ColorValue::rgb(0xFF, 0x00, 0xAA)); // #F0A -> #FF00AA

        let color = ColorValue::from_hex("#FF8040").unwrap();
        assert_eq!(color.to_hex_lowercase(), "#ff8040");
    }

    #[test]
    fn test_named_colors() {
        assert_eq!(ColorValue::from_name("red"), Some(ColorValue::red()));
        assert_eq!(ColorValue::from_name("transparent"), Some(ColorValue::transparent()));
        assert_eq!(ColorValue::from_name("nonexistent"), None);
    }

    #[test]
    fn test_color_manipulation() {
        let red = ColorValue::red();
        let darker_red = red.darken(0.2);
        assert!(darker_red.is_dark());

        let lighter_red = red.lighten(0.5);

        assert!(lighter_red.luminance() > red.luminance());

        let gray = red.grayscale();

        assert!(gray.is_grayscale());

        let inverted = ColorValue::black().invert();
        assert_eq!(inverted, ColorValue::white());
    }

    #[test]
    fn test_color_harmony() {
        let red = ColorValue::red();
        let complement = red.complement();

        // Red's complement should be cyan-ish
        let (h, _, _) = complement.to_hsl();
        assert!((h - 180.0).abs() < 10.0); // Approximately 180° from red

        let (tri1, tri2) = red.triadic();
        let (h1, _, _) = tri1.to_hsl();
        let (h2, _, _) = tri2.to_hsl();

        // Should be approximately 120° apart
        assert!((h1 - 120.0).abs() < 10.0);
        assert!((h2 - 240.0).abs() < 10.0);
    }

    #[test]
    fn test_contrast_ratio() {
        let white = ColorValue::white();
        let black = ColorValue::black();

        let contrast = white.contrast_ratio(&black);
        assert!((contrast - 21.0).abs() < 0.1); // Maximum contrast ratio

        assert!(black.is_suitable_for_text(&white));
        assert!(white.is_suitable_for_text(&black));
    }

    #[test]
    fn test_formatting() {
        let color = ColorValue::rgba(255, 128, 64, 200);

        assert_eq!(format!("{}", color), "#ff8040c8");
        assert_eq!(format!("{:#}", color), "rgba(255, 128, 64, 0.784)");

        let opaque_color = ColorValue::rgb(255, 128, 64);
        assert_eq!(format!("{:#}", opaque_color), "rgb(255, 128, 64)");
    }
}
