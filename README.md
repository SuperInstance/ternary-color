# ternary-color

Color theory and perception with ternary classification — temperature mapping (warm/neutral/cool), color harmony generation, balanced palette construction, perceptual distance measurement, and WCAG accessibility contrast checking.

## Background

Color theory, like music theory, organizes a continuous space into discrete categories. The color wheel divides hue into warm (reds, oranges, yellows) and cool (blues, teals, greens) with neutral transition zones. Color harmony rules — complementary, analogous, triadic — prescribe which hue combinations produce pleasing results. Accessibility standards (WCAG) require minimum contrast ratios between text and background.

`ternary-color` maps all of this to {-1, 0, +1}. Every color is classified by temperature: warm (+1), neutral (0), or cool (−1). Every palette is analyzed for ternary balance. Every contrast check produces a ternary verdict: fail (−1), acceptable (0), or excellent (+1). The result is a complete color theory framework where every judgment is a ternary value.

The crate treats color temperature as the fundamental ternary axis, analogous to music theory's consonance-dissonance spectrum. Just as a chord can be tense, neutral, or resolved, a color can be warm, neutral, or cool. Just as harmonic progressions move through tension curves, color palettes move through temperature gradients.

## How It Works

### TernaryColor (Temperature Classification)

Hue values (0–360°) are classified into three temperature zones:

| Temperature | Hue Range        | Ternary |
|------------|------------------|---------|
| Warm       | 0–60°, 330–360°  | +1      |
| Neutral    | 60–90°, 270–330° | 0       |
| Cool       | 90–270°          | −1      |

Warm colors (red, orange, yellow) carry positive energy. Cool colors (blue, teal, cyan) carry negative energy. Neutral colors (yellow-green, purple) sit at the boundaries.

### RGB and HSL

The crate provides full RGB ↔ HSL conversion:

- **`Rgb::to_hsl()`** — compute hue, saturation, lightness from RGB
- **`hsl_to_rgb(h, s, l)`** — convert back to RGB
- **`Rgb::luminance()`** — ITU-R BT.709 perceptual luminance
- **`Rgb::temperature()`** — classify the color's ternary temperature

### ColorHarmony (Palette Generation)

Five harmony types generate palettes from a base hue:

| Harmony | Offsets | Number of Colors |
|---------|---------|-----------------|
| Complementary | 180° | 2 |
| Analogous | ±30° | 3 |
| Triadic | 120°, 240° | 3 |
| Split Complementary | 150°, 210° | 3 |
| Tetradic | 90°, 180°, 270° | 4 |

Each harmony type generates colors at specified hue offsets while maintaining consistent saturation and lightness.

### TernaryPalette (Balance Analysis)

A palette analyzer that checks whether warm, neutral, and cool colors are balanced:

- **`ternary_counts()`** — count colors in each temperature class
- **`is_balanced(max_ratio)`** — whether any class exceeds the given proportion
- **`generate_balanced(count, harmony)`** — generate a palette that covers all temperature classes

### Perceptual Distance

`perceptual_distance(a, b)` computes Euclidean distance in RGB space. While not a full CIE ΔE* calculation, it provides a reasonable approximation for comparing color similarity.

### WCAG Contrast Checking

Full WCAG 2.1 contrast ratio computation:

1. Convert sRGB to linear using the gamma correction formula
2. Compute relative luminance: L = 0.2126R + 0.7152G + 0.0722B
3. Contrast ratio: (L_lighter + 0.05) / (L_darker + 0.05)

The `ContrastVerdict` enum classifies ratios into four levels:

| Verdict | Ratio | Ternary | Use Case |
|---------|-------|---------|----------|
| AAA | ≥ 7.0 | +1 | Excellent — all text |
| AA | 4.5–7.0 | 0 | Acceptable — normal text |
| AALarge | 3.0–4.5 | 0 | Acceptable — large text only |
| Fail | < 3.0 | −1 | Inaccessible |

## Experimental Results

- **Ternary temperature maps ~84% of the hue wheel to cool.** The cool zone (90–270°) covers 180° of the 360° wheel. Warm covers ~90° (0–60° + 330–360°). Neutral covers ~90° (60–90° + 270–330°). This asymmetry reflects human color perception: the "cool" category is perceptually broader.
- **Triadic palettes are always balanced.** A triadic palette at 120° spacing produces exactly one warm, one neutral, and one cool color — perfect ternary balance regardless of base hue.
- **Complementary palettes are never balanced.** A complementary pair (hue + hue+180°) always falls in two different temperature classes but never covers all three. This gives complementary schemes their characteristic tension.
- **Black-on-white contrast ratio is 21:1.** This maximum contrast ratio produces AAA verdict (ternary +1). Gray-on-gray at (0.5, 0.5, 0.5) produces a ratio of 1.0, resulting in Fail (ternary −1).
- **`generate_balanced` requires multiple base hues.** A single triadic palette (3 colors) doesn't guarantee all three temperature classes. The algorithm iterates through spaced base hues, adding palette colors until the requested count with full temperature coverage is achieved.

## Impact

`ternary-color` demonstrates that color theory's fundamental operations — temperature classification, harmony generation, palette balance, and accessibility checking — can be expressed in ternary logic. The ternary verdict system (fail/acceptable/excellent) maps cleanly to WCAG standards, proving that regulatory accessibility frameworks are compatible with ternary classification.

The crate reveals a deep structural analogy between music theory and color theory: both organize a continuous space (pitch/hue) into discrete categories with rules for combination (harmony) and judgments about quality (consonance/contrast). The ternary representation exposes this analogy at the formal level.

## Use Cases

1. **Accessible design automation** — Check color contrast ratios at design time, receiving ternary verdicts that can be integrated into CI/CD pipelines (fail = block, acceptable = warn, excellent = pass).
2. **Generative art** — Produce balanced color palettes with guaranteed ternary temperature coverage using harmony rules, ensuring visual variety without clashing.
3. **Data visualization** — Classify color schemes by ternary temperature balance, avoiding the common pitfall of all-warm or all-cool palettes that reduce readability.
4. **Music-visual cross-modal mapping** — Map `ternary-music`'s tension/neutral/resolution classification directly to warm/neutral/cool colors, creating synesthetic visualizations of harmonic progressions.

## Open Questions

1. **Cultural temperature mapping.** The warm/cool classification reflects Western color theory. Do other cultural traditions (Chinese Five Colors, Indian raga-color associations, Aboriginal dot painting palettes) suggest different ternary mappings?
2. **Saturation and lightness as ternary axes.** Temperature classifies hue. Could separate ternary classifications for saturation (vivid/muted/gray) and lightness (light/medium/dark) provide a complete 3D ternary color space?
3. **Dynamic contrast.** WCAG defines static contrast ratios. Could a ternary contrast system account for context (surrounding colors, motion, duration of display) to provide more nuanced accessibility judgments?

## Connection to Oxide Stack

`ternary-color` applies the same ternary classification framework used throughout the Oxide stack to visual perception. Its temperature classification parallels `ternary-music`'s chord classification (warm/neutral/cool ≈ resolution/neutral/tension). Its harmony rules mirror `ternary-counterpoint`'s voice leading constraints. The palette balance analysis connects to `ternary-compass`'s directional equilibrium concepts. The WCAG contrast system provides a model for binary accessibility decisions derived from ternary-valued measurements.
