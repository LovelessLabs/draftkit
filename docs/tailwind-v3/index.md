# Tailwind CSS v3 Utility Reference

Quick reference for Tailwind CSS v3.x utility classes and their CSS equivalents.

## Topics

### Layout
- [Display](display.md) - block, flex, grid, hidden, visibility
- [Position](position.md) - static, relative, absolute, fixed, sticky, z-index, inset
- [Flexbox](flexbox.md) - flex direction, wrap, grow, shrink, basis, justify, align
- [Grid](grid.md) - template columns/rows, spans, auto-flow, gap, place items

### Spacing & Sizing
- [Spacing](spacing.md) - padding, margin, space-between (0-96 scale)
- [Sizing](sizing.md) - width, height, min/max, size utility

### Typography
- [Typography](typography.md) - font family, size, weight, line-height, text alignment, decoration
- [Colors](colors.md) - color palette, opacity modifiers, text/bg/border colors

### Backgrounds & Borders
- [Backgrounds](backgrounds.md) - gradients, images, size, position, blend modes
- [Borders](borders.md) - width, color, radius, divide, outline, ring

### Effects & Transforms
- [Effects](effects.md) - shadows, opacity, blend modes
- [Filters](filters.md) - blur, brightness, contrast, grayscale, backdrop filters
- [Transforms](transforms.md) - scale, rotate, translate, skew, origin

### Animation & Interactivity
- [Transitions](transitions.md) - duration, timing, delay, built-in animations
- [Interactivity](interactivity.md) - cursor, pointer-events, scroll, touch, user-select

### Responsive & Theming
- [Responsive](responsive.md) - breakpoints, mobile-first patterns
- [Dark Mode](dark-mode.md) - class and media strategies, color patterns
- [States](states.md) - hover, focus, group, peer, has, variants

### Accessibility & SVG
- [Accessibility](accessibility.md) - sr-only, focus indicators, reduced motion
- [SVG](svg.md) - fill, stroke, icon patterns

---

## Quick Reference

### Breakpoints
```
sm:   640px+
md:   768px+
lg:   1024px+
xl:   1280px+
2xl:  1536px+
```

### Spacing Scale
```
0    → 0px
px   → 1px
0.5  → 0.125rem (2px)
1    → 0.25rem (4px)
2    → 0.5rem (8px)
3    → 0.75rem (12px)
4    → 1rem (16px)
5    → 1.25rem (20px)
6    → 1.5rem (24px)
8    → 2rem (32px)
10   → 2.5rem (40px)
12   → 3rem (48px)
16   → 4rem (64px)
20   → 5rem (80px)
24   → 6rem (96px)
32   → 8rem (128px)
40   → 10rem (160px)
48   → 12rem (192px)
56   → 14rem (224px)
64   → 16rem (256px)
72   → 18rem (288px)
80   → 20rem (320px)
96   → 24rem (384px)
```

### Color Shades
```
50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950
```

### Common Patterns

#### Centered Container
```html
<div class="container mx-auto px-4">...</div>
```

#### Flex Center
```html
<div class="flex items-center justify-center">...</div>
```

#### Responsive Grid
```html
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">...</div>
```

#### Card
```html
<div class="bg-white rounded-lg shadow-md p-6">...</div>
```

#### Button
```html
<button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600
               focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
  Button
</button>
```

#### Input
```html
<input class="w-full px-3 py-2 border border-gray-300 rounded-md
              focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
```

#### Absolute Centering
```html
<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">...</div>
```

#### Truncate Text
```html
<p class="truncate">...</p>
<!-- or -->
<p class="line-clamp-2">...</p>
```

#### Aspect Ratio
```html
<div class="aspect-video">...</div>
<div class="aspect-square">...</div>
```

#### Dark Mode
```html
<div class="bg-white dark:bg-gray-900 text-black dark:text-white">...</div>
```
