# Future Integration: ternary-color

## Current State
Provides ternary color classification (warm/neutral/cool), RGB/HSL conversion, color harmony rules, palette generation with ternary balance, perceptual distance, and accessibility contrast checking.

## Integration Opportunities

### With ternary-visualization / open-tui
Room state visualization needs color mapping. `TernaryColor::from_hue()` classifies any state dimension into warm/neutral/cool — perfect for heatmaps of room energy, agent activity, or cell grid state. The `to_ternary()` method maps directly to the three-state visualization that `ternary-visualization` uses. `ColorHarmony` rules ensure that multi-room dashboard views use harmonious palettes rather than random color assignments.

### With ternary-science
Scientific visualization of experimental results (GPU benchmarks, conservation law verification) needs perceptually uniform color scales. `perceptual_distance()` ensures that equal data differences map to equal perceived color differences.

### With ternary-music
Synesthesia mapping: chord tension maps to warm colors, resolution to cool. Interval consonance maps to color harmony. This enables multi-modal room representations — hear the room state AND see it.

## Potential in Mature Systems
In room-as-codespace, each room gets a color identity. The ternary color classification determines the room's visual representation in the campus map. Palette generation ensures the campus doesn't look like a rainbow explosion. Accessibility contrast checking ensures the dashboard is usable by humans with color vision deficiency.

## Cross-Pollination Ideas
- Color temperature as room type indicator: warm rooms are high-activity, cool rooms are stable/monitoring
- Palette generation for agent identity — each agent gets a visually distinct but harmonious color
- Perceptual distance as a clustering metric for room similarity visualization

## Dependencies for Next Steps
- ternary-visualization needs to use TernaryColor instead of hardcoded hex strings
- open-tui integration for terminal-based room dashboards
- ternary-room needs a color field for visual identity
