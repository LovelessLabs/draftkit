# Filter Utilities (Tailwind CSS v4)

## Architecture Note

Tailwind CSS v4 uses **CSS custom properties** for filter values:

```css
--blur-xs: 4px;
--blur-sm: 8px;   /* Note: 8px in v4, was 4px in v3 */
--blur-md: 12px;
/* ... */
```

## Blur

| Utility | CSS Property | CSS Value | Computed |
|---------|--------------|-----------|----------|
| `blur-none` | `filter` | `blur(0)` | 0 |
| `blur-xs` | `filter` | `blur(var(--blur-xs))` | 4px |
| `blur-sm` | `filter` | `blur(var(--blur-sm))` | 8px |
| `blur-md` | `filter` | `blur(var(--blur-md))` | 12px |
| `blur-lg` | `filter` | `blur(var(--blur-lg))` | 16px |
| `blur-xl` | `filter` | `blur(var(--blur-xl))` | 24px |
| `blur-2xl` | `filter` | `blur(var(--blur-2xl))` | 40px |
| `blur-3xl` | `filter` | `blur(var(--blur-3xl))` | 64px |

**Note**: `blur-xs` is new in v4. `blur-sm` changed from 4px to 8px.

## Brightness

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `brightness-0` | `filter` | `brightness(0)` |
| `brightness-50` | `filter` | `brightness(.5)` |
| `brightness-75` | `filter` | `brightness(.75)` |
| `brightness-90` | `filter` | `brightness(.9)` |
| `brightness-95` | `filter` | `brightness(.95)` |
| `brightness-100` | `filter` | `brightness(1)` |
| `brightness-105` | `filter` | `brightness(1.05)` |
| `brightness-110` | `filter` | `brightness(1.1)` |
| `brightness-125` | `filter` | `brightness(1.25)` |
| `brightness-150` | `filter` | `brightness(1.5)` |
| `brightness-200` | `filter` | `brightness(2)` |

## Contrast

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `contrast-0` | `filter` | `contrast(0)` |
| `contrast-50` | `filter` | `contrast(.5)` |
| `contrast-75` | `filter` | `contrast(.75)` |
| `contrast-100` | `filter` | `contrast(1)` |
| `contrast-125` | `filter` | `contrast(1.25)` |
| `contrast-150` | `filter` | `contrast(1.5)` |
| `contrast-200` | `filter` | `contrast(2)` |

## Drop Shadow

Uses CSS variables for drop shadow values. Designed for elements with transparency:

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `drop-shadow-none` | `filter` | `none` |
| `drop-shadow-xs` | `filter` | `drop-shadow(var(--drop-shadow-xs))` |
| `drop-shadow-sm` | `filter` | `drop-shadow(var(--drop-shadow-sm))` |
| `drop-shadow-md` | `filter` | `drop-shadow(var(--drop-shadow-md))` |
| `drop-shadow-lg` | `filter` | `drop-shadow(var(--drop-shadow-lg))` |
| `drop-shadow-xl` | `filter` | `drop-shadow(var(--drop-shadow-xl))` |
| `drop-shadow-2xl` | `filter` | `drop-shadow(var(--drop-shadow-2xl))` |

### Default Drop Shadow Values (theme.css)

| Variable | Value |
|----------|-------|
| `--drop-shadow-xs` | `0 1px 1px rgb(0 0 0 / 0.05)` |
| `--drop-shadow-sm` | `0 1px 2px rgb(0 0 0 / 0.15)` |
| `--drop-shadow-md` | `0 3px 3px rgb(0 0 0 / 0.12)` |
| `--drop-shadow-lg` | `0 4px 4px rgb(0 0 0 / 0.15)` |
| `--drop-shadow-xl` | `0 9px 7px rgb(0 0 0 / 0.1)` |
| `--drop-shadow-2xl` | `0 25px 25px rgb(0 0 0 / 0.15)` |

**Note**: `drop-shadow-xs` is new in v4. Values differ from v3.

## Grayscale

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `grayscale-0` | `filter` | `grayscale(0)` |
| `grayscale` | `filter` | `grayscale(100%)` |

## Hue Rotate

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `hue-rotate-0` | `filter` | `hue-rotate(0deg)` |
| `hue-rotate-15` | `filter` | `hue-rotate(15deg)` |
| `hue-rotate-30` | `filter` | `hue-rotate(30deg)` |
| `hue-rotate-60` | `filter` | `hue-rotate(60deg)` |
| `hue-rotate-90` | `filter` | `hue-rotate(90deg)` |
| `hue-rotate-180` | `filter` | `hue-rotate(180deg)` |
| `-hue-rotate-15` | `filter` | `hue-rotate(-15deg)` |
| `-hue-rotate-180` | `filter` | `hue-rotate(-180deg)` |

## Invert

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `invert-0` | `filter` | `invert(0)` |
| `invert` | `filter` | `invert(100%)` |

## Saturate

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `saturate-0` | `filter` | `saturate(0)` |
| `saturate-50` | `filter` | `saturate(.5)` |
| `saturate-100` | `filter` | `saturate(1)` |
| `saturate-150` | `filter` | `saturate(1.5)` |
| `saturate-200` | `filter` | `saturate(2)` |

## Sepia

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `sepia-0` | `filter` | `sepia(0)` |
| `sepia` | `filter` | `sepia(100%)` |

## Backdrop Filters

Apply filters to the area behind an element.

### Backdrop Blur

| Utility | CSS Property | CSS Value | Computed |
|---------|--------------|-----------|----------|
| `backdrop-blur-none` | `backdrop-filter` | `blur(0)` | 0 |
| `backdrop-blur-xs` | `backdrop-filter` | `blur(var(--blur-xs))` | 4px |
| `backdrop-blur-sm` | `backdrop-filter` | `blur(var(--blur-sm))` | 8px |
| `backdrop-blur-md` | `backdrop-filter` | `blur(var(--blur-md))` | 12px |
| `backdrop-blur-lg` | `backdrop-filter` | `blur(var(--blur-lg))` | 16px |
| `backdrop-blur-xl` | `backdrop-filter` | `blur(var(--blur-xl))` | 24px |
| `backdrop-blur-2xl` | `backdrop-filter` | `blur(var(--blur-2xl))` | 40px |
| `backdrop-blur-3xl` | `backdrop-filter` | `blur(var(--blur-3xl))` | 64px |

### Backdrop Brightness

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backdrop-brightness-0` | `backdrop-filter` | `brightness(0)` |
| `backdrop-brightness-50` | `backdrop-filter` | `brightness(.5)` |
| `backdrop-brightness-100` | `backdrop-filter` | `brightness(1)` |
| `backdrop-brightness-150` | `backdrop-filter` | `brightness(1.5)` |

### Backdrop Contrast

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backdrop-contrast-0` | `backdrop-filter` | `contrast(0)` |
| `backdrop-contrast-50` | `backdrop-filter` | `contrast(.5)` |
| `backdrop-contrast-100` | `backdrop-filter` | `contrast(1)` |
| `backdrop-contrast-150` | `backdrop-filter` | `contrast(1.5)` |

### Backdrop Grayscale

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backdrop-grayscale-0` | `backdrop-filter` | `grayscale(0)` |
| `backdrop-grayscale` | `backdrop-filter` | `grayscale(100%)` |

### Backdrop Hue Rotate

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backdrop-hue-rotate-0` | `backdrop-filter` | `hue-rotate(0deg)` |
| `backdrop-hue-rotate-90` | `backdrop-filter` | `hue-rotate(90deg)` |
| `backdrop-hue-rotate-180` | `backdrop-filter` | `hue-rotate(180deg)` |

### Backdrop Invert

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backdrop-invert-0` | `backdrop-filter` | `invert(0)` |
| `backdrop-invert` | `backdrop-filter` | `invert(100%)` |

### Backdrop Opacity

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backdrop-opacity-0` | `backdrop-filter` | `opacity(0)` |
| `backdrop-opacity-50` | `backdrop-filter` | `opacity(0.5)` |
| `backdrop-opacity-100` | `backdrop-filter` | `opacity(1)` |

### Backdrop Saturate

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backdrop-saturate-0` | `backdrop-filter` | `saturate(0)` |
| `backdrop-saturate-100` | `backdrop-filter` | `saturate(1)` |
| `backdrop-saturate-200` | `backdrop-filter` | `saturate(2)` |

### Backdrop Sepia

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backdrop-sepia-0` | `backdrop-filter` | `sepia(0)` |
| `backdrop-sepia` | `backdrop-filter` | `sepia(100%)` |

## Arbitrary Values

| Utility | CSS Output |
|---------|------------|
| `blur-[2px]` | `filter: blur(2px)` |
| `brightness-[1.75]` | `filter: brightness(1.75)` |
| `hue-rotate-[270deg]` | `filter: hue-rotate(270deg)` |
| `backdrop-blur-[10px]` | `backdrop-filter: blur(10px)` |
| `drop-shadow-[0_35px_35px_rgba(0,0,0,0.25)]` | Custom drop shadow |

## Common Patterns

### Frosted Glass Effect
```html
<div class="bg-white/30 backdrop-blur-md">
  <!-- Content behind is blurred -->
</div>
```

### Modal Backdrop
```html
<div class="fixed inset-0 bg-black/50 backdrop-blur-sm">
  <!-- Dimmed and slightly blurred backdrop -->
</div>
```

### Grayscale on Hover (Reverse)
```html
<img class="grayscale hover:grayscale-0 transition-all duration-300" src="..." />
<!-- Grayscale until hovered -->
```

### Disabled Image
```html
<img class="grayscale brightness-75" src="..." />
<!-- Muted, disabled appearance -->
```

### Bright on Hover
```html
<img class="brightness-90 hover:brightness-110 transition-all" src="..." />
<!-- Slightly dim, brightens on hover -->
```

### Image with Drop Shadow
```html
<img class="drop-shadow-xl" src="transparent-logo.png" />
<!-- Shadow respects image transparency -->
```

### Color Shift Effect
```html
<img class="hue-rotate-90" src="..." />
<!-- Shifts all colors by 90 degrees -->
```

### Vintage Photo Effect
```html
<img class="sepia contrast-125 brightness-90" src="..." />
<!-- Old photo look -->
```

### Frosted Glass Card
```html
<div class="bg-white/10 backdrop-blur-lg border border-white/20 rounded-xl p-6">
  <h2 class="text-white">Glass Card</h2>
</div>
```

### iOS-style Blur Header
```html
<header class="fixed top-0 w-full bg-white/80 backdrop-blur-md backdrop-saturate-150 border-b">
  <!-- Semi-transparent blurred header -->
</header>
```

### Dark Overlay with Blur
```html
<div class="relative">
  <img src="..." class="w-full" />
  <div class="absolute inset-0 bg-black/40 backdrop-blur-sm flex items-center justify-center">
    <span class="text-white text-xl">Overlay Text</span>
  </div>
</div>
```

## v3 to v4 Migration Notes

| Change | v3 | v4 |
|--------|----|----|
| `blur-sm` | 4px | 8px |
| `blur-xs` | N/A | 4px (new) |
| `drop-shadow-xs` | N/A | new |
| `backdrop-blur-xs` | N/A | new |

**Breaking change**: If you relied on `blur-sm` being 4px, use `blur-xs` in v4.
