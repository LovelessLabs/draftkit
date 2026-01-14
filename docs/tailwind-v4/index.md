# Tailwind CSS v4 Documentation Index

Quick reference documentation for Tailwind CSS v4 utilities.

## Layout
- **flexbox** - Flex container, direction, wrap, justify, align, grow, shrink, basis
- **grid** - Grid container, columns, rows, spans, auto-flow, gap, place utilities
- **position** - Static, relative, absolute, fixed, sticky, inset, z-index
- **display** - Block, inline, flex, grid, hidden, visibility, overflow

## Spacing
- **spacing** - Padding, margin, space-between, negative margins
- **sizing** - Width, height, min/max width/height, size utility

## Typography
- **typography** - Font family, size, weight, line height, letter spacing, text alignment, decoration, transform, overflow

## Backgrounds & Borders
- **colors** - Color palette, text color, background color, opacity modifiers
- **backgrounds** - Background color, image, gradients, size, position, repeat
- **borders** - Border width, color, style, radius, divide, outline, ring

## Effects
- **effects** - Box shadow, shadow color, opacity, blend modes, drop shadow
- **filters** - Blur, brightness, contrast, grayscale, hue-rotate, backdrop filters

## Transforms & Animation
- **transforms** - Scale, rotate, translate, skew, origin, 3D transforms
- **transitions** - Transition property, duration, timing, delay, animations

## Interactivity
- **interactivity** - Cursor, pointer events, resize, scroll, touch, user select
- **forms** - Input styling, checkbox, radio, select, toggle, file input
- **states** - Hover, focus, active, disabled, checked, group, peer, has, not

## Responsive & Dark Mode
- **responsive** - Breakpoints, mobile-first, container queries, max-width variants
- **dark-mode** - Dark mode setup, class strategy, color recommendations

## Accessibility
- **accessibility** - Screen reader, focus styles, motion preferences, print styles

## SVG
- **svg** - Fill, stroke, stroke width, icon patterns

## Reference
- **v4-changes** - What's new in v4, migration from v3, configuration changes

---

## Quick Lookup

### Common Patterns
```html
<!-- Centered flex container -->
<div class="flex items-center justify-center">

<!-- Responsive grid -->
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">

<!-- Card with shadow -->
<div class="rounded-lg bg-white p-6 shadow-lg">

<!-- Button with states -->
<button class="bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white px-4 py-2 rounded">

<!-- Dark mode support -->
<div class="bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100">

<!-- Truncated text -->
<p class="truncate">Long text that gets cut off...</p>

<!-- Absolute centering -->
<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
```

### Spacing Scale
```
0    = 0px
1    = 0.25rem (4px)
2    = 0.5rem (8px)
3    = 0.75rem (12px)
4    = 1rem (16px)
5    = 1.25rem (20px)
6    = 1.5rem (24px)
8    = 2rem (32px)
10   = 2.5rem (40px)
12   = 3rem (48px)
16   = 4rem (64px)
20   = 5rem (80px)
24   = 6rem (96px)
```

### Breakpoints
```
sm   = 640px
md   = 768px
lg   = 1024px
xl   = 1280px
2xl  = 1536px
```
