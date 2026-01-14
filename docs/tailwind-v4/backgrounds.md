# Background Utilities (Tailwind CSS v4)

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

## Background Opacity

Using the `/` modifier:
```
bg-black/50   → background-color: rgb(0 0 0 / 0.5)
bg-white/75   → background-color: rgb(255 255 255 / 0.75)
bg-blue-500/25 → background-color: rgb(59 130 246 / 0.25)
```

## Background Image

### Gradients
```
bg-none           → background-image: none
bg-gradient-to-t  → background-image: linear-gradient(to top, ...)
bg-gradient-to-tr → background-image: linear-gradient(to top right, ...)
bg-gradient-to-r  → background-image: linear-gradient(to right, ...)
bg-gradient-to-br → background-image: linear-gradient(to bottom right, ...)
bg-gradient-to-b  → background-image: linear-gradient(to bottom, ...)
bg-gradient-to-bl → background-image: linear-gradient(to bottom left, ...)
bg-gradient-to-l  → background-image: linear-gradient(to left, ...)
bg-gradient-to-tl → background-image: linear-gradient(to top left, ...)
```

### Gradient Color Stops
```
from-{color}    → --tw-gradient-from: {color}
via-{color}     → --tw-gradient-via: {color}
to-{color}      → --tw-gradient-to: {color}

from-transparent → starting color transparent
from-current     → starting color currentColor
from-blue-500    → starting color blue-500
```

### Gradient Stop Positions (v4)
```
from-0%   → gradient starts at 0%
from-5%   → gradient starts at 5%
via-50%   → middle color at 50%
to-100%   → gradient ends at 100%
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

## Background Clip
```
bg-clip-border  → background-clip: border-box
bg-clip-padding → background-clip: padding-box
bg-clip-content → background-clip: content-box
bg-clip-text    → background-clip: text (for gradient text)
```

## Background Origin
```
bg-origin-border  → background-origin: border-box
bg-origin-padding → background-origin: padding-box
bg-origin-content → background-origin: content-box
```

## Arbitrary Values
```
bg-[url('/img/hero.jpg')] → background-image: url('/img/hero.jpg')
bg-[length:200px_100px]   → background-size: 200px 100px
bg-[position:center_top]  → background-position: center top
```

## Common Patterns

### Simple Gradient
```html
<div class="bg-gradient-to-r from-blue-500 to-purple-500">
  <!-- Horizontal gradient -->
</div>
```

### Three-Color Gradient
```html
<div class="bg-gradient-to-r from-green-400 via-blue-500 to-purple-600">
  <!-- Green → Blue → Purple -->
</div>
```

### Gradient Text
```html
<h1 class="bg-gradient-to-r from-pink-500 to-violet-500 bg-clip-text text-transparent">
  Gradient Heading
</h1>
```

### Hero with Background Image
```html
<div class="bg-[url('/hero.jpg')] bg-cover bg-center bg-no-repeat">
  <div class="bg-black/50 p-8">
    <!-- Content with overlay -->
  </div>
</div>
```

### Radial Gradient (arbitrary)
```html
<div class="bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-blue-500 to-transparent">
  <!-- Radial gradient effect -->
</div>
```

### Subtle Gradient Background
```html
<div class="bg-gradient-to-b from-white to-gray-50">
  <!-- Subtle vertical gradient -->
</div>
```

### Fixed Background (Parallax Effect)
```html
<div class="bg-fixed bg-cover bg-center" style="background-image: url('/bg.jpg')">
  <!-- Background stays fixed on scroll -->
</div>
```
