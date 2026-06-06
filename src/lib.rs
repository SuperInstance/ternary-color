//! # ternary-color
//!
//! Ternary color theory: RGB mapped to the ternary set {-1, 0, +1}.
//!
//! Each color channel is a **trit** — not a byte, not a float, but one of three
//! values: dark (-1), mid (0), or bright (+1). With three channels (R, G, B) that
//! gives us 3³ = 27 possible colors — a complete, finite, mathematically elegant
//! palette.
//!
//! This crate makes ternary mathematics **visible**. Students can *see* {-1, 0, +1}
//! rendered as actual colors, making abstract ternary algebra tangible and intuitive.

use std::fmt;

/// A trit value: one of three ternary states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trit {
    /// Dark / low / negative
    NegOne,
    /// Mid / neutral / zero
    Zero,
    /// Bright / high / positive
    PosOne,
}

impl Trit {
    /// Create a Trit from an integer (-1, 0, or 1).
    ///
    /// # Panics
    /// Panics if `n` is not -1, 0, or 1.
    pub fn from_int(n: i8) -> Self {
        match n {
            -1 => Trit::NegOne,
            0 => Trit::Zero,
            1 => Trit::PosOne,
            _ => panic!("Trit value must be -1, 0, or 1, got {}", n),
        }
    }

    /// Convert to integer representation.
    pub fn to_int(self) -> i8 {
        match self {
            Trit::NegOne => -1,
            Trit::Zero => 0,
            Trit::PosOne => 1,
        }
    }

    /// Ternary addition (mod 3 arithmetic in balanced form).
    /// -1 + 1 = 0, 0 + 0 = 0, 1 + 1 = -1 (wraps), etc.
    pub fn add(self, other: Trit) -> Trit {
        Trit::from_int(((self.to_int() + other.to_int() + 4) % 3) - 1)
    }

    /// Ternary negation: flip the trit.
    pub fn negate(self) -> Trit {
        match self {
            Trit::NegOne => Trit::PosOne,
            Trit::Zero => Trit::Zero,
            Trit::PosOne => Trit::NegOne,
        }
    }

    /// All possible trit values.
    pub fn all() -> [Trit; 3] {
        [Trit::NegOne, Trit::Zero, Trit::PosOne]
    }

    /// Map trit to a 0-255 range for display.
    pub fn to_u8(self) -> u8 {
        match self {
            Trit::NegOne => 0,
            Trit::Zero => 128,
            Trit::PosOne => 255,
        }
    }

    /// Create a Trit from a 0-255 value, thresholding into three buckets.
    pub fn from_u8(v: u8) -> Trit {
        if v < 85 {
            Trit::NegOne
        } else if v < 170 {
            Trit::Zero
        } else {
            Trit::PosOne
        }
    }
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_int())
    }
}

/// A ternary color: RGB where each channel is a Trit {-1, 0, +1}.
///
/// This gives exactly 27 possible colors, forming a complete ternary color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TritColor {
    pub r: Trit,
    pub g: Trit,
    pub b: Trit,
}

impl TritColor {
    /// Create a new TritColor from three Trits.
    pub fn new(r: Trit, g: Trit, b: Trit) -> Self {
        TritColor { r, g, b }
    }

    /// Create from three integers (-1, 0, or 1 each).
    pub fn from_ints(r: i8, g: i8, b: i8) -> Self {
        TritColor {
            r: Trit::from_int(r),
            g: Trit::from_int(g),
            b: Trit::from_int(b),
        }
    }

    /// Black: (-1, -1, -1)
    pub const fn black() -> Self {
        TritColor { r: Trit::NegOne, g: Trit::NegOne, b: Trit::NegOne }
    }

    /// White: (+1, +1, +1)
    pub const fn white() -> Self {
        TritColor { r: Trit::PosOne, g: Trit::PosOne, b: Trit::PosOne }
    }

    /// Red: (+1, -1, -1)
    pub const fn red() -> Self {
        TritColor { r: Trit::PosOne, g: Trit::NegOne, b: Trit::NegOne }
    }

    /// Green: (-1, +1, -1)
    pub const fn green() -> Self {
        TritColor { r: Trit::NegOne, g: Trit::PosOne, b: Trit::NegOne }
    }

    /// Blue: (-1, -1, +1)
    pub const fn blue() -> Self {
        TritColor { r: Trit::NegOne, g: Trit::NegOne, b: Trit::PosOne }
    }

    /// Yellow: (+1, +1, -1)
    pub const fn yellow() -> Self {
        TritColor { r: Trit::PosOne, g: Trit::PosOne, b: Trit::NegOne }
    }

    /// Cyan: (-1, +1, +1)
    pub const fn cyan() -> Self {
        TritColor { r: Trit::NegOne, g: Trit::PosOne, b: Trit::PosOne }
    }

    /// Magenta: (+1, -1, +1)
    pub const fn magenta() -> Self {
        TritColor { r: Trit::PosOne, g: Trit::NegOne, b: Trit::PosOne }
    }

    /// Gray / mid: (0, 0, 0)
    pub const fn gray() -> Self {
        TritColor { r: Trit::Zero, g: Trit::Zero, b: Trit::Zero }
    }

    /// Ternary color mixing: Z₃ addition per channel.
    ///
    /// This is the fundamental ternary operation on colors — each channel
    /// is independently added using balanced ternary arithmetic.
    pub fn mix(self, other: TritColor) -> TritColor {
        TritColor {
            r: self.r.add(other.r),
            g: self.g.add(other.g),
            b: self.b.add(other.b),
        }
    }

    /// Complementary color: negate each channel.
    ///
    /// In ternary, the complement of -1 is +1, the complement of +1 is -1,
    /// and 0 stays 0. This creates a natural "opposite" that's more nuanced
    /// than binary inversion.
    pub fn complement(self) -> TritColor {
        TritColor {
            r: self.r.negate(),
            g: self.g.negate(),
            b: self.b.negate(),
        }
    }

    /// Triadic harmony: rotate the channels cyclically.
    ///
    /// Returns two additional colors that form a triadic color scheme
    /// — equally spaced around the ternary color wheel.
    pub fn triadic(self) -> (TritColor, TritColor) {
        let second = TritColor {
            r: self.b,
            g: self.r,
            b: self.g,
        };
        let third = TritColor {
            r: self.g,
            g: self.b,
            b: self.r,
        };
        (second, third)
    }

    /// Ternary color distance: sum of absolute differences per channel.
    ///
    /// Returns a value from 0 (identical) to 6 (maximum difference, e.g., black↔white).
    pub fn distance(self, other: TritColor) -> u8 {
        let rd = (self.r.to_int() - other.r.to_int()).abs() as u8;
        let gd = (self.g.to_int() - other.g.to_int()).abs() as u8;
        let bd = (self.b.to_int() - other.b.to_int()).abs() as u8;
        rd + gd + bd
    }

    /// Convert to (u8, u8, u8) for display purposes.
    pub fn to_rgb_u8(self) -> (u8, u8, u8) {
        (self.r.to_u8(), self.g.to_u8(), self.b.to_u8())
    }

    /// Convert from (u8, u8, u8), thresholding each channel.
    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        TritColor {
            r: Trit::from_u8(r),
            g: Trit::from_u8(g),
            b: Trit::from_u8(b),
        }
    }

    /// All 27 possible ternary colors.
    pub fn all_colors() -> Vec<TritColor> {
        let mut colors = Vec::with_capacity(27);
        for &r in &Trit::all() {
            for &g in &Trit::all() {
                for &b in &Trit::all() {
                    colors.push(TritColor::new(r, g, b));
                }
            }
        }
        colors
    }

    /// Generate an analogous palette: colors near this one in ternary space.
    ///
    /// Analogous colors share similar channel values, differing by at most
    /// one trit in each channel.
    pub fn analogous_palette(self) -> Vec<TritColor> {
        let mut palette = Vec::new();
        for color in TritColor::all_colors() {
            if self.distance(color) <= 2 && self != color {
                palette.push(color);
            }
        }
        palette
    }

    /// Generate a complementary palette: this color + complement + neighbors.
    pub fn complementary_palette(self) -> Vec<TritColor> {
        let comp = self.complement();
        let mut palette = vec![self, comp];
        for color in TritColor::all_colors() {
            if color != self && color != comp
                && (self.distance(color) == 1 || comp.distance(color) == 1)
            {
                palette.push(color);
            }
        }
        palette
    }

    /// Generate a triadic palette: this color + its two triadic partners.
    pub fn triadic_palette(self) -> Vec<TritColor> {
        let (t2, t3) = self.triadic();
        vec![self, t2, t3]
    }
}

impl fmt::Display for TritColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

/// A small ternary image: a 2D grid of TritColors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TritImage {
    width: usize,
    height: usize,
    pixels: Vec<TritColor>,
}

impl TritImage {
    /// Create a new TritImage filled with black.
    pub fn new(width: usize, height: usize) -> Self {
        TritImage {
            width,
            height,
            pixels: vec![TritColor::black(); width * height],
        }
    }

    /// Create from a vector of TritColors.
    pub fn from_pixels(width: usize, height: usize, pixels: Vec<TritColor>) -> Option<Self> {
        if pixels.len() != width * height {
            return None;
        }
        Some(TritImage { width, height, pixels })
    }

    /// Get the pixel at (x, y).
    pub fn get(&self, x: usize, y: usize) -> Option<TritColor> {
        if x < self.width && y < self.height {
            Some(self.pixels[y * self.width + x])
        } else {
            None
        }
    }

    /// Set the pixel at (x, y).
    pub fn set(&mut self, x: usize, y: usize, color: TritColor) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    /// Width of the image.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Height of the image.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Blend (ternary mix) this image with another of the same size.
    ///
    /// Each pixel is mixed using Z₃ addition per channel.
    pub fn blend(&self, other: &TritImage) -> Option<TritImage> {
        if self.width != other.width || self.height != other.height {
            return None;
        }
        let pixels: Vec<TritColor> = self.pixels.iter()
            .zip(other.pixels.iter())
            .map(|(&a, &b)| a.mix(b))
            .collect();
        Some(TritImage { width: self.width, height: self.height, pixels })
    }

    /// Invert the image: complement every pixel.
    pub fn invert(&self) -> TritImage {
        let pixels: Vec<TritColor> = self.pixels.iter().map(|&c| c.complement()).collect();
        TritImage { width: self.width, height: self.height, pixels }
    }

    /// Count how many pixels match a given color.
    pub fn count_color(&self, color: TritColor) -> usize {
        self.pixels.iter().filter(|&&c| c == color).count()
    }

    /// Render the image as a string using block characters.
    /// Maps brightness to unicode block elements.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.pixels[y * self.width + x];
                let sum = c.r.to_int() + c.g.to_int() + c.b.to_int();
                let ch = match sum {
                    -3 => '░',
                    -2 => '▒',
                    -1 => '▓',
                    0 => '─',
                    1 => '▓',
                    2 => '▒',
                    3 => '█',
                    _ => '?',
                };
                out.push(ch);
            }
            out.push('\n');
        }
        out
    }

    /// Fill the entire image with one color.
    pub fn fill(&mut self, color: TritColor) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }

    /// Apply a function to every pixel.
    pub fn map_pixels<F>(&self, f: F) -> TritImage
    where
        F: Fn(TritColor) -> TritColor,
    {
        let pixels: Vec<TritColor> = self.pixels.iter().map(|&c| f(c)).collect();
        TritImage { width: self.width, height: self.height, pixels }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trit_from_to_int() {
        assert_eq!(Trit::from_int(-1).to_int(), -1);
        assert_eq!(Trit::from_int(0).to_int(), 0);
        assert_eq!(Trit::from_int(1).to_int(), 1);
    }

    #[test]
    #[should_panic]
    fn test_trit_invalid() {
        Trit::from_int(2);
    }

    #[test]
    fn test_trit_addition() {
        // Z₃ addition: balanced ternary
        assert_eq!(Trit::Zero.add(Trit::Zero), Trit::Zero);
        assert_eq!(Trit::NegOne.add(Trit::PosOne), Trit::Zero);
        assert_eq!(Trit::PosOne.add(Trit::PosOne), Trit::NegOne); // 1+1 = -1 (wraps)
        assert_eq!(Trit::NegOne.add(Trit::NegOne), Trit::PosOne); // -1+-1 = +1 (wraps)
        assert_eq!(Trit::NegOne.add(Trit::Zero), Trit::NegOne);
        assert_eq!(Trit::PosOne.add(Trit::Zero), Trit::PosOne);
    }

    #[test]
    fn test_trit_negation() {
        assert_eq!(Trit::NegOne.negate(), Trit::PosOne);
        assert_eq!(Trit::Zero.negate(), Trit::Zero);
        assert_eq!(Trit::PosOne.negate(), Trit::NegOne);
    }

    #[test]
    fn test_trit_u8_roundtrip() {
        for &t in &Trit::all() {
            assert_eq!(Trit::from_u8(t.to_u8()), t);
        }
    }

    #[test]
    fn test_color_mixing() {
        // Red + Green: red=(1,-1,-1), green=(-1,1,-1)
        // R: 1+(-1) = 0, G: -1+1 = 0, B: -1+(-1) = 1 (wraps in Z₃)
        let result = TritColor::red().mix(TritColor::green());
        assert_eq!(result, TritColor::from_ints(0, 0, 1));

        // Black + White = Gray
        let mixed = TritColor::black().mix(TritColor::white());
        assert_eq!(mixed, TritColor::gray());

        // Gray + Gray = Gray (0+0=0 for all channels)
        let double_gray = TritColor::gray().mix(TritColor::gray());
        assert_eq!(double_gray, TritColor::gray());
    }

    #[test]
    fn test_complement() {
        assert_eq!(TritColor::black().complement(), TritColor::white());
        assert_eq!(TritColor::white().complement(), TritColor::black());
        assert_eq!(TritColor::red().complement(), TritColor::cyan());
        assert_eq!(TritColor::green().complement(), TritColor::magenta());
        assert_eq!(TritColor::blue().complement(), TritColor::yellow());
        assert_eq!(TritColor::gray().complement(), TritColor::gray());
    }

    #[test]
    fn test_triadic() {
        let (t2, t3) = TritColor::red().triadic();
        // Red (1,-1,-1) → rotate: (-1,1,-1)=Green, (-1,-1,1)=Blue
        assert_eq!(t2, TritColor::from_ints(-1, 1, -1));
        assert_eq!(t3, TritColor::from_ints(-1, -1, 1));

        // Triadic of gray should be gray (all channels same)
        let (g2, g3) = TritColor::gray().triadic();
        assert_eq!(g2, TritColor::gray());
        assert_eq!(g3, TritColor::gray());
    }

    #[test]
    fn test_distance() {
        // Same color = distance 0
        assert_eq!(TritColor::red().distance(TritColor::red()), 0);

        // Black to white = 6 (max)
        assert_eq!(TritColor::black().distance(TritColor::white()), 6);

        // Black to gray = 3 (0-(-1)=1 per channel, times 3)
        assert_eq!(TritColor::black().distance(TritColor::gray()), 3);

        // Red to black: R: |1-(-1)|=2, G: 0, B: 0 → 2
        assert_eq!(TritColor::red().distance(TritColor::black()), 2);

        // Symmetry
        assert_eq!(
            TritColor::red().distance(TritColor::blue()),
            TritColor::blue().distance(TritColor::red())
        );
    }

    #[test]
    fn test_all_colors_count() {
        assert_eq!(TritColor::all_colors().len(), 27);
    }

    #[test]
    fn test_analogous_palette() {
        let palette = TritColor::red().analogous_palette();
        // Colors at distance 1 or 2 from red, excluding red itself
        for &color in &palette {
            let d = TritColor::red().distance(color);
            assert!(d > 0 && d <= 2);
        }
        assert!(!palette.is_empty());
    }

    #[test]
    fn test_complementary_palette() {
        let palette = TritColor::red().complementary_palette();
        assert!(palette.contains(&TritColor::red()));
        assert!(palette.contains(&TritColor::cyan())); // complement of red
        assert!(palette.len() >= 2);
    }

    #[test]
    fn test_triadic_palette() {
        let palette = TritColor::red().triadic_palette();
        assert_eq!(palette.len(), 3);
        assert_eq!(palette[0], TritColor::red());
    }

    #[test]
    fn test_image_new() {
        let img = TritImage::new(3, 3);
        assert_eq!(img.width(), 3);
        assert_eq!(img.height(), 3);
        // All pixels should be black
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(img.get(x, y), Some(TritColor::black()));
            }
        }
    }

    #[test]
    fn test_image_set_get() {
        let mut img = TritImage::new(2, 2);
        img.set(0, 0, TritColor::red());
        img.set(1, 1, TritColor::white());
        assert_eq!(img.get(0, 0), Some(TritColor::red()));
        assert_eq!(img.get(1, 1), Some(TritColor::white()));
        assert_eq!(img.get(0, 1), Some(TritColor::black()));
    }

    #[test]
    fn test_image_blend() {
        let mut img1 = TritImage::new(2, 2);
        img1.fill(TritColor::red());
        let mut img2 = TritImage::new(2, 2);
        img2.fill(TritColor::cyan());
        let blended = img1.blend(&img2).unwrap();
        // Red + Cyan = Gray (they're complements)
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(blended.get(x, y), Some(TritColor::gray()));
            }
        }
    }

    #[test]
    fn test_image_blend_size_mismatch() {
        let img1 = TritImage::new(2, 2);
        let img2 = TritImage::new(3, 3);
        assert!(img1.blend(&img2).is_none());
    }

    #[test]
    fn test_image_invert() {
        let mut img = TritImage::new(1, 1);
        img.set(0, 0, TritColor::red());
        let inverted = img.invert();
        assert_eq!(inverted.get(0, 0), Some(TritColor::cyan()));
    }

    #[test]
    fn test_image_count_color() {
        let mut img = TritImage::new(2, 2);
        img.set(0, 0, TritColor::red());
        img.set(1, 0, TritColor::red());
        img.set(0, 1, TritColor::white());
        assert_eq!(img.count_color(TritColor::red()), 2);
        assert_eq!(img.count_color(TritColor::white()), 1);
        assert_eq!(img.count_color(TritColor::black()), 1);
    }

    #[test]
    fn test_image_render() {
        let mut img = TritImage::new(3, 1);
        img.set(0, 0, TritColor::black());   // sum = -3
        img.set(1, 0, TritColor::gray());     // sum = 0
        img.set(2, 0, TritColor::white());    // sum = 3
        let rendered = img.render();
        assert!(rendered.contains('░'));  // darkest
        assert!(rendered.contains('─'));  // mid
        assert!(rendered.contains('█'));  // brightest
    }

    #[test]
    fn test_image_from_pixels() {
        let pixels = vec![TritColor::red(), TritColor::green(), TritColor::blue(), TritColor::white()];
        let img = TritImage::from_pixels(2, 2, pixels).unwrap();
        assert_eq!(img.get(0, 0), Some(TritColor::red()));
        assert_eq!(img.get(1, 1), Some(TritColor::white()));
    }

    #[test]
    fn test_image_from_pixels_wrong_size() {
        let pixels = vec![TritColor::red(), TritColor::green()];
        assert!(TritImage::from_pixels(2, 2, pixels).is_none());
    }

    #[test]
    fn test_image_map_pixels() {
        let mut img = TritImage::new(2, 1);
        img.set(0, 0, TritColor::red());
        img.set(1, 0, TritColor::blue());
        let mapped = img.map_pixels(|c| c.complement());
        assert_eq!(mapped.get(0, 0), Some(TritColor::cyan()));
        assert_eq!(mapped.get(1, 0), Some(TritColor::yellow()));
    }

    #[test]
    fn test_image_fill() {
        let mut img = TritImage::new(2, 2);
        img.fill(TritColor::white());
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(img.get(x, y), Some(TritColor::white()));
            }
        }
    }

    #[test]
    fn test_out_of_bounds() {
        let img = TritImage::new(2, 2);
        assert_eq!(img.get(5, 5), None);
    }

    #[test]
    fn test_rgb_u8_roundtrip() {
        let colors = [TritColor::black(), TritColor::gray(), TritColor::white(),
                      TritColor::red(), TritColor::green(), TritColor::blue()];
        for &c in &colors {
            let (r, g, b) = c.to_rgb_u8();
            assert_eq!(TritColor::from_rgb_u8(r, g, b), c);
        }
    }

    #[test]
    fn test_trit_display() {
        assert_eq!(format!("{}", Trit::NegOne), "-1");
        assert_eq!(format!("{}", Trit::Zero), "0");
        assert_eq!(format!("{}", Trit::PosOne), "1");
    }

    #[test]
    fn test_color_display() {
        let c = TritColor::red();
        assert_eq!(format!("{}", c), "(1, -1, -1)");
    }
}
