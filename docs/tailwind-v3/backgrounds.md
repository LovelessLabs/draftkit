# Background Utilities (Tailwind CSS v3)

## Background Color

See colors.md for the full color palette.

```
bg-inherit     → background-color: inherit
bg-current     → background-color: currentColor
bg-transparent → background-color: transparent
bg-black       → background-color: #000
bg-white       → background-color: #fff
bg-{color}-{shade} → background-color: {color value}
```

### With Opacity
```
bg-black/50    → background-color: rgb(0 0 0 / 0.5)
bg-white/75    → background-color: rgb(255 255 255 / 0.75)
bg-blue-500/25 → background-color: rgb(59 130 246 / 0.25)
```

## Background Gradient

### Direction
```
bg-gradient-to-t  → linear-gradient(to top, var(--tw-gradient-stops))
bg-gradient-to-tr → linear-gradient(to top right, var(--tw-gradient-stops))
bg-gradient-to-r  → linear-gradient(to right, var(--tw-gradient-stops))
bg-gradient-to-br → linear-gradient(to bottom right, var(--tw-gradient-stops))
bg-gradient-to-b  → linear-gradient(to bottom, var(--tw-gradient-stops))
bg-gradient-to-bl → linear-gradient(to bottom left, var(--tw-gradient-stops))
bg-gradient-to-l  → linear-gradient(to left, var(--tw-gradient-stops))
bg-gradient-to-tl → linear-gradient(to top left, var(--tw-gradient-stops))
bg-none           → background-image: none
```

### Gradient Stops
```
from-{color}  → --tw-gradient-from: {color}
via-{color}   → --tw-gradient-stops: ... {color} ... (middle)
to-{color}    → --tw-gradient-to: {color}
```

### Gradient Stop Positions (v3.4+)
```
from-0%   → --tw-gradient-from-position: 0%
from-5%   → --tw-gradient-from-position: 5%
from-10%  → --tw-gradient-from-position: 10%
...
from-100% → --tw-gradient-from-position: 100%

via-0%, via-10%, via-20%, ... via-100%
to-0%, to-10%, to-20%, ... to-100%
```

## Background Size
```
bg-auto    → background-size: auto
bg-cover   → background-size: cover
bg-contain → background-size: contain
```

## Background Position
```
bg-bottom       → background-position: bottom
bg-center       → background-position: center
bg-left         → background-position: left
bg-left-bottom  → background-position: left bottom
bg-left-top     → background-position: left top
bg-right        → background-position: right
bg-right-bottom → background-position: right bottom
bg-right-top    → background-position: right top
bg-top          → background-position: top
```

## Background Repeat
```
bg-repeat       → background-repeat: repeat
bg-no-repeat    → background-repeat: no-repeat
bg-repeat-x     → background-repeat: repeat-x
bg-repeat-y     → background-repeat: repeat-y
bg-repeat-round → background-repeat: round
bg-repeat-space → background-repeat: space
```

## Background Attachment
```
bg-fixed  → background-attachment: fixed
bg-local  → background-attachment: local
bg-scroll → background-attachment: scroll
```

## Background Origin
```
bg-origin-border  → background-origin: border-box
bg-origin-padding → background-origin: padding-box
bg-origin-content → background-origin: content-box
```

## Background Clip
```
bg-clip-border  → background-clip: border-box
bg-clip-padding → background-clip: padding-box
bg-clip-content → background-clip: content-box
bg-clip-text    → background-clip: text
```

## Background Blend Mode
```
bg-blend-normal      → background-blend-mode: normal
bg-blend-multiply    → background-blend-mode: multiply
bg-blend-screen      → background-blend-mode: screen
bg-blend-overlay     → background-blend-mode: overlay
bg-blend-darken      → background-blend-mode: darken
bg-blend-lighten     → background-blend-mode: lighten
bg-blend-color-dodge → background-blend-mode: color-dodge
bg-blend-color-burn  → background-blend-mode: color-burn
bg-blend-hard-light  → background-blend-mode: hard-light
bg-blend-soft-light  → background-blend-mode: soft-light
bg-blend-difference  → background-blend-mode: difference
bg-blend-exclusion   → background-blend-mode: exclusion
bg-blend-hue         → background-blend-mode: hue
bg-blend-saturation  → background-blend-mode: saturation
bg-blend-color       → background-blend-mode: color
bg-blend-luminosity  → background-blend-mode: luminosity
```

## Common Patterns

### Simple Gradient
```html
<div class="bg-gradient-to-r from-blue-500 to-purple-500">
  Blue to purple gradient
</div>
```

### Three-Color Gradient
```html
<div class="bg-gradient-to-r from-green-400 via-blue-500 to-purple-600">
  Green → Blue → Purple
</div>
```

### Gradient with Positions
```html
<div class="bg-gradient-to-r from-blue-500 from-10% via-sky-500 via-30% to-emerald-500 to-90%">
  Gradient with custom stop positions
</div>
```

### Hero Image
```html
<div class="bg-cover bg-center bg-no-repeat"
     style="background-image: url('hero.jpg')">
  <!-- Content -->
</div>
```

### Fixed Background (Parallax Effect)
```html
<div class="bg-fixed bg-cover bg-center min-h-screen"
     style="background-image: url('background.jpg')">
  <!-- Scrolling content -->
</div>
```

### Gradient Text
```html
<h1 class="bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
  Gradient Text
</h1>
```

### Semi-transparent Overlay
```html
<div class="relative">
  <img src="photo.jpg" alt="">
  <div class="absolute inset-0 bg-black/50">
    <!-- Overlay content -->
  </div>
</div>
```

### Gradient Border Effect
```html
<div class="bg-gradient-to-r from-pink-500 to-violet-500 p-1 rounded-lg">
  <div class="bg-white rounded-md p-4">
    Content with gradient border
  </div>
</div>
```

### Multiple Backgrounds
```html
<div class="bg-gradient-to-b from-transparent to-black/50 bg-cover"
     style="background-image: url('image.jpg')">
  <!-- Image with gradient overlay -->
</div>
```
