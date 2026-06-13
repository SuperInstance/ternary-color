//! # ternary-color
//!
//! Color theory and perception with ternary classification.
//! TernaryColor (warm/neutral/cool), ColorHarmony rules, palette generation
//! with ternary balance, perceptual distance, accessibility contrast checking.

#![forbid(unsafe_code)]

/// Ternary color temperature classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryColor {
    Warm,
    Neutral,
    Cool,
}

impl TernaryColor {
    /// Classify a hue (0–360°) into warm/neutral/cool.
    pub fn from_hue(hue: f64) -> Self {
        // Warm: reds, oranges, yellows (~0-60, 300-360)
        // Neutral: greens, purples near boundary (~60-90, 270-300)
        // Cool: blues, teals, blue-greens (~90-270)
        let h = hue % 360.0;
        if h < 60.0 || h >= 330.0 {
            TernaryColor::Warm
        } else if h < 90.0 || h >= 270.0 {
            TernaryColor::Neutral
        } else {
            TernaryColor::Cool
        }
    }

    /// Convert to ternary digit: Warm=+1, Neutral=0, Cool=-1.
    pub fn to_ternary(self) -> i8 {
        match self {
            TernaryColor::Warm => 1,
            TernaryColor::Neutral => 0,
            TernaryColor::Cool => -1,
        }
    }

    /// From ternary digit.
    pub fn from_ternary(v: i8) -> Option<Self> {
        match v {
            1 => Some(TernaryColor::Warm),
            0 => Some(TernaryColor::Neutral),
            -1 => Some(TernaryColor::Cool),
            _ => None,
        }
    }
}

/// RGB color in 0.0–1.0 float range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Convert to HSL. Returns (hue, saturation, lightness).
    pub fn to_hsl(&self) -> (f64, f64, f64) {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let l = (max + min) / 2.0;
        if (max - min).abs() < 1e-10 {
            return (0.0, 0.0, l);
        }
        let d = max - min;
        let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
        let h = if (self.r - max).abs() < 1e-10 {
            ((self.g - self.b) / d) % 6.0
        } else if (self.g - max).abs() < 1e-10 {
            (self.b - self.r) / d + 2.0
        } else {
            (self.r - self.g) / d + 4.0
        };
        let h = (h * 60.0 + 360.0) % 360.0;
        (h, s, l)
    }

    /// Perceptual luminance (ITU-R BT.709).
    pub fn luminance(&self) -> f64 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    /// Classify this color's temperature.
    pub fn temperature(&self) -> TernaryColor {
        let (h, _, _) = self.to_hsl();
        TernaryColor::from_hue(h)
    }
}

/// Color harmony types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorHarmony {
    Complementary,
    Analogous,
    Triadic,
    SplitComplementary,
    Tetradic,
}

impl ColorHarmony {
    /// Return the hue offsets for this harmony (in degrees).
    pub fn offsets(&self) -> &'static [f64] {
        match self {
            ColorHarmony::Complementary => &[180.0],
            ColorHarmony::Analogous => &[-30.0, 30.0],
            ColorHarmony::Triadic => &[120.0, 240.0],
            ColorHarmony::SplitComplementary => &[150.0, 210.0],
            ColorHarmony::Tetradic => &[90.0, 180.0, 270.0],
        }
    }

    /// Generate a palette from a base hue and this harmony rule.
    pub fn palette(&self, base_hue: f64, saturation: f64, lightness: f64) -> Vec<Rgb> {
        let mut colors = vec![hsl_to_rgb(base_hue, saturation, lightness)];
        for &offset in self.offsets() {
            colors.push(hsl_to_rgb((base_hue + offset) % 360.0, saturation, lightness));
        }
        colors
    }
}

/// Convert HSL to RGB.
pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Rgb {
    if s.abs() < 1e-10 {
        return Rgb::new(l, l, l);
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let hue_to_rgb = |t: f64| -> f64 {
        let mut t = t;
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
        if t < 1.0 / 2.0 { return q; }
        if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
        p
    };
    Rgb::new(
        hue_to_rgb((h / 360.0) + 1.0 / 3.0),
        hue_to_rgb(h / 360.0),
        hue_to_rgb((h / 360.0) - 1.0 / 3.0),
    )
}

/// Palette with ternary balance analysis.
#[derive(Debug, Clone)]
pub struct TernaryPalette {
    pub colors: Vec<Rgb>,
}

impl TernaryPalette {
    pub fn new(colors: Vec<Rgb>) -> Self {
        Self { colors }
    }

    /// Count colors of each temperature class.
    pub fn ternary_counts(&self) -> (usize, usize, usize) {
        let (mut warm, mut neutral, mut cool) = (0, 0, 0);
        for c in &self.colors {
            match c.temperature() {
                TernaryColor::Warm => warm += 1,
                TernaryColor::Neutral => neutral += 1,
                TernaryColor::Cool => cool += 1,
            }
        }
        (warm, neutral, cool)
    }

    /// Check if palette is balanced (no class dominates by more than ratio).
    pub fn is_balanced(&self, max_ratio: f64) -> bool {
        let (w, n, c) = self.ternary_counts();
        let total = w + n + c;
        if total == 0 { return true; }
        let max_count = w.max(n).max(c) as f64;
        max_count / total as f64 <= max_ratio
    }

    /// Generate a balanced palette with specified count.
    pub fn generate_balanced(count: usize, harmony: ColorHarmony) -> Self {
        let mut colors = Vec::new();
        // Pick base hues spaced to cover temperature classes
        let base_hues = [0.0, 30.0, 60.0, 120.0, 180.0, 210.0, 240.0, 300.0, 330.0];
        let mut i = 0;
        while colors.len() < count {
            let base = base_hues[i % base_hues.len()];
            let palette = harmony.palette(base, 0.7, 0.5);
            for c in palette {
                if colors.len() < count {
                    colors.push(c);
                }
            }
            i += 1;
        }
        Self { colors }
    }
}

/// Perceptual distance between two colors (CIE76 simplified).
pub fn perceptual_distance(a: &Rgb, b: &Rgb) -> f64 {
    let dr = a.r - b.r;
    let dg = a.g - b.g;
    let db = a.b - b.b;
    (dr * dr + dg * dg + db * db).sqrt()
}

/// WCAG relative luminance from sRGB component.
fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Relative luminance per WCAG 2.1.
pub fn relative_luminance(color: &Rgb) -> f64 {
    let r = srgb_to_linear(color.r);
    let g = srgb_to_linear(color.g);
    let b = srgb_to_linear(color.b);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// WCAG 2.1 contrast ratio between two colors.
pub fn contrast_ratio(fg: &Rgb, bg: &Rgb) -> f64 {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Accessibility contrast verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastVerdict {
    /// < 3.0 — fail
    Fail,
    /// 3.0–4.5 — AA large text only
    AALarge,
    /// 4.5–7.0 — AA
    AA,
    /// >= 7.0 — AAA
    AAA,
}

impl ContrastVerdict {
    /// Classify a contrast ratio.
    pub fn from_ratio(ratio: f64) -> Self {
        if ratio >= 7.0 { ContrastVerdict::AAA }
        else if ratio >= 4.5 { ContrastVerdict::AA }
        else if ratio >= 3.0 { ContrastVerdict::AALarge }
        else { ContrastVerdict::Fail }
    }

    /// Check accessibility of a foreground/background pair.
    pub fn check(fg: &Rgb, bg: &Rgb) -> Self {
        Self::from_ratio(contrast_ratio(fg, bg))
    }

    /// To ternary: Fail=-1, AALarge/AA=0, AAA=+1.
    pub fn to_ternary(self) -> i8 {
        match self {
            ContrastVerdict::Fail => -1,
            ContrastVerdict::AALarge | ContrastVerdict::AA => 0,
            ContrastVerdict::AAA => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_color_from_hue_warm() {
        assert_eq!(TernaryColor::from_hue(0.0), TernaryColor::Warm);
        assert_eq!(TernaryColor::from_hue(30.0), TernaryColor::Warm);
        assert_eq!(TernaryColor::from_hue(350.0), TernaryColor::Warm);
    }

    #[test]
    fn test_ternary_color_from_hue_cool() {
        assert_eq!(TernaryColor::from_hue(180.0), TernaryColor::Cool);
        assert_eq!(TernaryColor::from_hue(240.0), TernaryColor::Cool);
        assert_eq!(TernaryColor::from_hue(150.0), TernaryColor::Cool);
    }

    #[test]
    fn test_ternary_color_from_hue_neutral() {
        assert_eq!(TernaryColor::from_hue(75.0), TernaryColor::Neutral);
        assert_eq!(TernaryColor::from_hue(280.0), TernaryColor::Neutral);
    }

    #[test]
    fn test_ternary_roundtrip() {
        for v in [-1i8, 0, 1] {
            assert_eq!(TernaryColor::from_ternary(v).unwrap().to_ternary(), v);
        }
        assert!(TernaryColor::from_ternary(2).is_none());
    }

    #[test]
    fn test_rgb_to_hsl_pure_red() {
        let (h, s, l) = Rgb::new(1.0, 0.0, 0.0).to_hsl();
        assert!((h - 0.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((l - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_rgb_to_hsl_white() {
        let (h, s, l) = Rgb::new(1.0, 1.0, 1.0).to_hsl();
        assert!((l - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_luminance() {
        let white = Rgb::new(1.0, 1.0, 1.0);
        assert!((white.luminance() - 1.0).abs() < 0.01);
        let black = Rgb::new(0.0, 0.0, 0.0);
        assert!(black.luminance().abs() < 0.01);
    }

    #[test]
    fn test_color_temperature_red() {
        assert_eq!(Rgb::new(1.0, 0.0, 0.0).temperature(), TernaryColor::Warm);
    }

    #[test]
    fn test_color_temperature_blue() {
        assert_eq!(Rgb::new(0.0, 0.0, 1.0).temperature(), TernaryColor::Cool);
    }

    #[test]
    fn test_harmony_complementary_palette() {
        let palette = ColorHarmony::Complementary.palette(0.0, 0.7, 0.5);
        assert_eq!(palette.len(), 2);
    }

    #[test]
    fn test_harmony_triadic_palette() {
        let palette = ColorHarmony::Triadic.palette(0.0, 0.7, 0.5);
        assert_eq!(palette.len(), 3);
    }

    #[test]
    fn test_harmony_tetradic_offsets() {
        let offsets = ColorHarmony::Tetradic.offsets();
        assert_eq!(offsets.len(), 3);
    }

    #[test]
    fn test_hsl_to_rgb_roundtrip() {
        let rgb = hsl_to_rgb(120.0, 0.5, 0.5);
        let (h, s, l) = rgb.to_hsl();
        assert!((h - 120.0).abs() < 1.0);
        assert!((s - 0.5).abs() < 0.05);
        assert!((l - 0.5).abs() < 0.05);
    }

    #[test]
    fn test_perceptual_distance_identity() {
        let c = Rgb::new(0.5, 0.3, 0.8);
        assert!(perceptual_distance(&c, &c).abs() < 1e-10);
    }

    #[test]
    fn test_perceptual_distance_black_white() {
        let d = perceptual_distance(&Rgb::new(0.0, 0.0, 0.0), &Rgb::new(1.0, 1.0, 1.0));
        assert!((d - 3.0f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_contrast_ratio_black_white() {
        let ratio = contrast_ratio(&Rgb::new(0.0, 0.0, 0.0), &Rgb::new(1.0, 1.0, 1.0));
        assert!(ratio >= 20.0);
    }

    #[test]
    fn test_contrast_ratio_same_color() {
        let ratio = contrast_ratio(&Rgb::new(0.5, 0.5, 0.5), &Rgb::new(0.5, 0.5, 0.5));
        assert!((ratio - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_contrast_verdict_aaa() {
        let v = ContrastVerdict::check(&Rgb::new(0.0, 0.0, 0.0), &Rgb::new(1.0, 1.0, 1.0));
        assert_eq!(v, ContrastVerdict::AAA);
    }

    #[test]
    fn test_contrast_verdict_fail() {
        let v = ContrastVerdict::check(&Rgb::new(0.5, 0.5, 0.5), &Rgb::new(0.5, 0.5, 0.5));
        assert_eq!(v, ContrastVerdict::Fail);
    }

    #[test]
    fn test_verdict_ternary_mapping() {
        assert_eq!(ContrastVerdict::AAA.to_ternary(), 1);
        assert_eq!(ContrastVerdict::AA.to_ternary(), 0);
        assert_eq!(ContrastVerdict::Fail.to_ternary(), -1);
    }

    #[test]
    fn test_palette_balance() {
        let colors = vec![
            Rgb::new(1.0, 0.0, 0.0), // warm
            Rgb::new(0.0, 0.0, 1.0), // cool
            Rgb::new(0.5, 0.8, 0.5), // neutral-ish
        ];
        let palette = TernaryPalette::new(colors);
        assert!(palette.is_balanced(0.8));
    }

    #[test]
    fn test_palette_generate_balanced() {
        let palette = TernaryPalette::generate_balanced(6, ColorHarmony::Triadic);
        assert_eq!(palette.colors.len(), 6);
    }

    #[test]
    fn test_ternary_counts() {
        let colors = vec![
            Rgb::new(1.0, 0.0, 0.0), // warm
            Rgb::new(0.0, 0.0, 1.0), // cool
            Rgb::new(1.0, 0.0, 0.0), // warm
        ];
        let palette = TernaryPalette::new(colors);
        let (w, n, c) = palette.ternary_counts();
        assert_eq!(w, 2);
        assert_eq!(c, 1);
        assert_eq!(n, 0);
    }

    #[test]
    fn test_verdict_from_ratio() {
        assert_eq!(ContrastVerdict::from_ratio(7.5), ContrastVerdict::AAA);
        assert_eq!(ContrastVerdict::from_ratio(5.0), ContrastVerdict::AA);
        assert_eq!(ContrastVerdict::from_ratio(3.5), ContrastVerdict::AALarge);
        assert_eq!(ContrastVerdict::from_ratio(2.0), ContrastVerdict::Fail);
    }

    #[test]
    fn test_relative_luminance_green() {
        let green = Rgb::new(0.0, 1.0, 0.0);
        let lum = relative_luminance(&green);
        assert!(lum > 0.6 && lum < 0.8);
    }
}
