# Transform Utilities (Tailwind CSS v4)

## Architecture Note

Tailwind CSS v4 uses **native CSS transform properties** instead of the `transform:` function:

| v3 Output | v4 Output |
|-----------|-----------|
| `transform: scale(.5)` | `scale: 50%` |
| `transform: rotate(45deg)` | `rotate: 45deg` |
| `transform: translateX(1rem)` | `translate: <value> var(--tw-translate-y)` |

This allows transforms to be composed more naturally and leverages modern CSS features.

## Scale

Uses the native CSS `scale` property.

### Uniform Scale

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `scale-0` | `scale` | `0%` |
| `scale-50` | `scale` | `50%` |
| `scale-75` | `scale` | `75%` |
| `scale-90` | `scale` | `90%` |
| `scale-95` | `scale` | `95%` |
| `scale-100` | `scale` | `100%` |
| `scale-105` | `scale` | `105%` |
| `scale-110` | `scale` | `110%` |
| `scale-125` | `scale` | `125%` |
| `scale-150` | `scale` | `150%` |
| `scale-none` | `scale` | `none` |
| `-scale-50` | `scale` | `calc(50% * -1)` |

### Axis-Specific Scale

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `scale-x-50` | `scale` | `50% var(--tw-scale-y)` |
| `scale-x-100` | `scale` | `100% var(--tw-scale-y)` |
| `scale-y-50` | `scale` | `var(--tw-scale-x) 50%` |
| `scale-y-100` | `scale` | `var(--tw-scale-x) 100%` |

### 3D Scale

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `scale-3d` | `scale` | `var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z)` |
| `scale-z-50` | Sets `--tw-scale-z` | `50%` |

## Rotate

Uses the native CSS `rotate` property.

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `rotate-0` | `rotate` | `0deg` |
| `rotate-1` | `rotate` | `1deg` |
| `rotate-2` | `rotate` | `2deg` |
| `rotate-3` | `rotate` | `3deg` |
| `rotate-6` | `rotate` | `6deg` |
| `rotate-12` | `rotate` | `12deg` |
| `rotate-45` | `rotate` | `45deg` |
| `rotate-90` | `rotate` | `90deg` |
| `rotate-180` | `rotate` | `180deg` |
| `rotate-none` | `rotate` | `none` |
| `-rotate-1` | `rotate` | `calc(1deg * -1)` |
| `-rotate-45` | `rotate` | `calc(45deg * -1)` |
| `-rotate-90` | `rotate` | `calc(90deg * -1)` |

### 3D Rotation

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `rotate-x-0` | `rotate` | `x 0deg` |
| `rotate-x-45` | `rotate` | `x 45deg` |
| `rotate-x-90` | `rotate` | `x 90deg` |
| `rotate-x-180` | `rotate` | `x 180deg` |
| `rotate-y-0` | `rotate` | `y 0deg` |
| `rotate-y-45` | `rotate` | `y 45deg` |
| `rotate-y-90` | `rotate` | `y 90deg` |
| `rotate-y-180` | `rotate` | `y 180deg` |

## Translate

Uses the native CSS `translate` property with CSS variable composition.

### X-Axis

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `translate-x-0` | `translate` | `0px var(--tw-translate-y)` |
| `translate-x-px` | `translate` | `1px var(--tw-translate-y)` |
| `translate-x-1` | `translate` | `calc(var(--spacing) * 1) var(--tw-translate-y)` |
| `translate-x-4` | `translate` | `calc(var(--spacing) * 4) var(--tw-translate-y)` |
| `translate-x-1/2` | `translate` | `50% var(--tw-translate-y)` |
| `translate-x-full` | `translate` | `100% var(--tw-translate-y)` |
| `-translate-x-1` | `translate` | `calc(var(--spacing) * -1) var(--tw-translate-y)` |
| `-translate-x-1/2` | `translate` | `-50% var(--tw-translate-y)` |
| `-translate-x-full` | `translate` | `-100% var(--tw-translate-y)` |

### Y-Axis

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `translate-y-0` | `translate` | `var(--tw-translate-x) 0px` |
| `translate-y-px` | `translate` | `var(--tw-translate-x) 1px` |
| `translate-y-1` | `translate` | `var(--tw-translate-x) calc(var(--spacing) * 1)` |
| `translate-y-4` | `translate` | `var(--tw-translate-x) calc(var(--spacing) * 4)` |
| `translate-y-1/2` | `translate` | `var(--tw-translate-x) 50%` |
| `translate-y-full` | `translate` | `var(--tw-translate-x) 100%` |
| `-translate-y-1/2` | `translate` | `var(--tw-translate-x) -50%` |
| `-translate-y-full` | `translate` | `var(--tw-translate-x) -100%` |

### Special Values

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `translate-none` | `translate` | `none` |
| `translate-full` | `translate` | `100% 100%` |
| `-translate-full` | `translate` | `-100% -100%` |
| `translate-1/2` | `translate` | `50% 50%` |

### Z-Axis (3D)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `translate-z-0` | `translate` | `... 0px` |
| `translate-z-1` | `translate` | `... calc(var(--spacing) * 1)` |
| `translate-z-4` | `translate` | `... calc(var(--spacing) * 4)` |
| `translate-3d` | `translate` | `var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z)` |

## Skew

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `skew-x-0` | `transform` | `skewX(0deg)` |
| `skew-x-1` | `transform` | `skewX(1deg)` |
| `skew-x-2` | `transform` | `skewX(2deg)` |
| `skew-x-3` | `transform` | `skewX(3deg)` |
| `skew-x-6` | `transform` | `skewX(6deg)` |
| `skew-x-12` | `transform` | `skewX(12deg)` |
| `skew-y-0` | `transform` | `skewY(0deg)` |
| `skew-y-1` | `transform` | `skewY(1deg)` |
| `skew-y-2` | `transform` | `skewY(2deg)` |
| `skew-y-3` | `transform` | `skewY(3deg)` |
| `skew-y-6` | `transform` | `skewY(6deg)` |
| `skew-y-12` | `transform` | `skewY(12deg)` |
| `-skew-x-6` | `transform` | `skewX(-6deg)` |
| `-skew-y-6` | `transform` | `skewY(-6deg)` |

**Note**: Skew still uses `transform:` as there's no native CSS `skew` property.

## Transform Origin

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `origin-center` | `transform-origin` | `center` |
| `origin-top` | `transform-origin` | `top` |
| `origin-top-right` | `transform-origin` | `top right` |
| `origin-right` | `transform-origin` | `right` |
| `origin-bottom-right` | `transform-origin` | `bottom right` |
| `origin-bottom` | `transform-origin` | `bottom` |
| `origin-bottom-left` | `transform-origin` | `bottom left` |
| `origin-left` | `transform-origin` | `left` |
| `origin-top-left` | `transform-origin` | `top left` |

## 3D Transform Support

### Perspective

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `perspective-none` | `perspective` | `none` |
| `perspective-dramatic` | `perspective` | `var(--perspective-dramatic)` (100px) |
| `perspective-near` | `perspective` | `var(--perspective-near)` (300px) |
| `perspective-normal` | `perspective` | `var(--perspective-normal)` (500px) |
| `perspective-midrange` | `perspective` | `var(--perspective-midrange)` (800px) |
| `perspective-distant` | `perspective` | `var(--perspective-distant)` (1200px) |

### Perspective Origin

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `perspective-origin-center` | `perspective-origin` | `center` |
| `perspective-origin-top` | `perspective-origin` | `top` |
| `perspective-origin-top-right` | `perspective-origin` | `top right` |
| `perspective-origin-right` | `perspective-origin` | `right` |
| `perspective-origin-bottom-right` | `perspective-origin` | `bottom right` |
| `perspective-origin-bottom` | `perspective-origin` | `bottom` |
| `perspective-origin-bottom-left` | `perspective-origin` | `bottom left` |
| `perspective-origin-left` | `perspective-origin` | `left` |
| `perspective-origin-top-left` | `perspective-origin` | `top left` |

### Transform Style

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `transform-flat` | `transform-style` | `flat` |
| `transform-3d` | `transform-style` | `preserve-3d` |

### Backface Visibility

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `backface-visible` | `backface-visibility` | `visible` |
| `backface-hidden` | `backface-visibility` | `hidden` |

## Arbitrary Values

| Utility | CSS Output |
|---------|------------|
| `scale-[1.75]` | `scale: 175%` |
| `rotate-[17deg]` | `rotate: 17deg` |
| `translate-x-[17rem]` | `translate: 17rem var(--tw-translate-y)` |
| `skew-y-[17deg]` | `transform: skewY(17deg)` |
| `origin-[33%_75%]` | `transform-origin: 33% 75%` |
| `perspective-[500px]` | `perspective: 500px` |

## Common Patterns

### Hover Scale
```html
<div class="transition hover:scale-105">
  Scales up on hover
</div>
```

### Button Press Effect
```html
<button class="transition active:scale-95">
  Press me
</button>
```

### Centered Absolute Element
```html
<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
  <!-- Perfectly centered -->
</div>
```

### Rotating Icon
```html
<svg class="transition group-hover:rotate-180">
  <!-- Arrow that flips on hover -->
</svg>
```

### Slide In Animation
```html
<div class="translate-x-full transition group-hover:translate-x-0">
  <!-- Slides in from right -->
</div>
```

### Card Flip (3D)
```html
<div class="group perspective-normal">
  <div class="transform-3d transition duration-500 group-hover:rotate-y-180">
    <div class="backface-hidden">Front</div>
    <div class="absolute inset-0 rotate-y-180 backface-hidden">Back</div>
  </div>
</div>
```

### Tilt Effect
```html
<div class="transition hover:rotate-3 hover:scale-105">
  Tilts on hover
</div>
```

### Skewed Section
```html
<div class="-skew-y-3">
  <div class="skew-y-3">
    <!-- Content is straight, container is skewed -->
  </div>
</div>
```

## v3 to v4 Migration Notes

| v3 Output | v4 Output |
|-----------|-----------|
| `scale-50` → `transform: scale(.5)` | `scale-50` → `scale: 50%` |
| `rotate-45` → `transform: rotate(45deg)` | `rotate-45` → `rotate: 45deg` |
| `translate-x-4` → `transform: translateX(1rem)` | `translate-x-4` → `translate: calc(var(--spacing) * 4) var(--tw-translate-y)` |

### New v4 Utilities

- `scale-none`, `rotate-none`, `translate-none` - Reset utilities
- `scale-3d`, `translate-3d` - 3D composition helpers
- `translate-z-*` - Z-axis translation
- `scale-z-*` - Z-axis scaling
