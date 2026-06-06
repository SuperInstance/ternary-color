# ternary-color

**Ternary color theory: RGB mapped to the ternary set {-1, 0, +1}.**

[![Tests](https://img.shields.io/badge/tests-28%20passing-brightgreen)]()

## Why?

Ternary mathematics is powerful but abstract. Colors are *immediate* — everyone
can see red, blue, yellow. By mapping ternary trits to actual color channels,
students can **see** {-1, 0, +1} rendered as dark, mid, and bright values.

This crate provides a complete ternary color algebra that's mathematically sound
and visually intuitive. With three channels (R, G, B) each taking three values,
we get exactly **27 colors** — a complete, finite, elegant palette.

## Core Concepts

### The Trit

A **trit** is to ternary what a bit is to binary — a single digit with three
possible values:

| Trit | Meaning | u8 Mapping |
|------|---------|------------|
| -1   | Dark    | 0          |
| 0    | Mid     | 128        |
| +1   | Bright  | 255        |

### TritColor

A color with three trit channels (R, G, B), giving 3³ = 27 unique colors:

```rust
use ternary_color::{TritColor, Trit};

let red = TritColor::red();       // (+1, -1, -1)
let blue = TritColor::blue();     // (-1, -1, +1)
let white = TritColor::white();   // (+1, +1, +1)
let black = TritColor::black();   // (-1, -1, -1)
let gray = TritColor::gray();     // (0, 0, 0)
```

### Color Mixing (Z₃ Addition)

Colors are mixed using **balanced ternary addition per channel** — the
fundamental operation of Z₃ arithmetic:

```rust
// Mixing black + white = gray (each channel: -1 + 1 = 0)
let mixed = TritColor::black().mix(TritColor::white());
assert_eq!(mixed, TritColor::gray());

// Red + Green: (1,-1,-1) + (-1,1,-1) = (0,0,1) = blue-ish
let result = TritColor::red().mix(TritColor::green());
```

### Complementary Colors

Ternary complementation negates each channel — more nuanced than binary inversion:

```rust
assert_eq!(TritColor::red().complement(), TritColor::cyan());
assert_eq!(TritColor::black().complement(), TritColor::white());
// Gray is self-complementary!
assert_eq!(TritColor::gray().complement(), TritColor::gray());
```

### Triadic Harmony

Rotate the channels cyclically to create triadic color schemes:

```rust
let (second, third) = TritColor::red().triadic();
// Red → Cyan-shifted → Blue-shifted
```

### Color Distance

Manhattan distance in ternary space (0–6 range):

```rust
assert_eq!(TritColor::black().distance(TritColor::white()), 6); // maximum
assert_eq!(TritColor::red().distance(TritColor::red()), 0);     // identical
```

### Palette Generation

```rust
let analogous = TritColor::red().analogous_palette();     // nearby colors
let complementary = TritColor::red().complementary_palette(); // + complement
let triadic = TritColor::red().triadic_palette();          // 3 equally spaced
```

### TritImage

A small ternary image — a 2D grid of TritColors with blending, inversion, and
rendering operations:

```rust
use ternary_color::TritImage;

let mut img = TritImage::new(8, 8);
img.set(0, 0, TritColor::red());
img.fill(TritColor::blue());

let blended = img.blend(&other_image);
let inverted = img.invert();
println!("{}", img.render()); // Unicode block art
```

## The 27 Colors

The complete ternary color space:

```
(-1,-1,-1) Black     (0,-1,-1)            (1,-1,-1) Red
(-1,-1, 0)           (0,-1, 0)            (1,-1, 0)
(-1,-1, 1) Blue      (0,-1, 1)            (1,-1, 1) Magenta
(-1, 0,-1)           (0, 0,-1)            (1, 0,-1)
(-1, 0, 0)           (0, 0, 0) Gray       (1, 0, 0)
(-1, 0, 1)           (0, 0, 1)            (1, 0, 1)
(-1, 1,-1) Green     (0, 1,-1)            (1, 1,-1) Yellow
(-1, 1, 0)           (0, 1, 0)            (1, 1, 0)
(-1, 1, 1) Cyan      (0, 1, 1)            (1, 1, 1) White
```

## Educational Value

This crate is designed for the **Loom** educational platform, which makes agent
coordination concepts accessible through hands-on exploration. Ternary color
theory serves as a visual gateway to:

1. **Balanced ternary arithmetic** — students can *see* addition wrap around
2. **Finite fields** — Z₃ operations become color mixing
3. **Distance metrics** — color proximity is Manhattan distance
4. **Group theory** — complements, rotations, symmetries become visible patterns

## API Overview

| Type | Description |
|------|-------------|
| `Trit` | Single ternary digit (-1, 0, +1) |
| `TritColor` | RGB color with ternary channels |
| `TritImage` | 2D grid of TritColors |

## License

MIT
