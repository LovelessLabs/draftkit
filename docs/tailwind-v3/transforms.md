# Transform Utilities (Tailwind CSS v3)

## Transform Origin
```
origin-center       → transform-origin: center
origin-top          → transform-origin: top
origin-top-right    → transform-origin: top right
origin-right        → transform-origin: right
origin-bottom-right → transform-origin: bottom right
origin-bottom       → transform-origin: bottom
origin-bottom-left  → transform-origin: bottom left
origin-left         → transform-origin: left
origin-top-left     → transform-origin: top left
```

## Scale

### Uniform Scale
```
scale-0   → transform: scale(0)
scale-50  → transform: scale(.5)
scale-75  → transform: scale(.75)
scale-90  → transform: scale(.9)
scale-95  → transform: scale(.95)
scale-100 → transform: scale(1)
scale-105 → transform: scale(1.05)
scale-110 → transform: scale(1.1)
scale-125 → transform: scale(1.25)
scale-150 → transform: scale(1.5)
```

### Axis-specific Scale
```
scale-x-0, scale-x-50, scale-x-75, scale-x-90, scale-x-95
scale-x-100, scale-x-105, scale-x-110, scale-x-125, scale-x-150

scale-y-0, scale-y-50, scale-y-75, scale-y-90, scale-y-95
scale-y-100, scale-y-105, scale-y-110, scale-y-125, scale-y-150
```

## Rotate
```
rotate-0   → transform: rotate(0deg)
rotate-1   → transform: rotate(1deg)
rotate-2   → transform: rotate(2deg)
rotate-3   → transform: rotate(3deg)
rotate-6   → transform: rotate(6deg)
rotate-12  → transform: rotate(12deg)
rotate-45  → transform: rotate(45deg)
rotate-90  → transform: rotate(90deg)
rotate-180 → transform: rotate(180deg)

# Negative rotation
-rotate-1   → transform: rotate(-1deg)
-rotate-2   → transform: rotate(-2deg)
-rotate-3   → transform: rotate(-3deg)
-rotate-6   → transform: rotate(-6deg)
-rotate-12  → transform: rotate(-12deg)
-rotate-45  → transform: rotate(-45deg)
-rotate-90  → transform: rotate(-90deg)
-rotate-180 → transform: rotate(-180deg)
```

## Translate

### Horizontal Translation
```
translate-x-0    → transform: translateX(0px)
translate-x-px   → transform: translateX(1px)
translate-x-0.5  → transform: translateX(0.125rem)
translate-x-1    → transform: translateX(0.25rem)
translate-x-2    → transform: translateX(0.5rem)
translate-x-4    → transform: translateX(1rem)
translate-x-8    → transform: translateX(2rem)
translate-x-full → transform: translateX(100%)
translate-x-1/2  → transform: translateX(50%)
translate-x-1/3  → transform: translateX(33.333%)
translate-x-2/3  → transform: translateX(66.667%)
translate-x-1/4  → transform: translateX(25%)
translate-x-3/4  → transform: translateX(75%)
```

### Vertical Translation
```
translate-y-0    → transform: translateY(0px)
translate-y-px   → transform: translateY(1px)
translate-y-1    → transform: translateY(0.25rem)
translate-y-2    → transform: translateY(0.5rem)
translate-y-4    → transform: translateY(1rem)
translate-y-full → transform: translateY(100%)
translate-y-1/2  → transform: translateY(50%)
```

### Negative Translation
```
-translate-x-4    → transform: translateX(-1rem)
-translate-x-full → transform: translateX(-100%)
-translate-x-1/2  → transform: translateX(-50%)
-translate-y-4    → transform: translateY(-1rem)
-translate-y-full → transform: translateY(-100%)
-translate-y-1/2  → transform: translateY(-50%)
```

## Skew
```
skew-x-0  → transform: skewX(0deg)
skew-x-1  → transform: skewX(1deg)
skew-x-2  → transform: skewX(2deg)
skew-x-3  → transform: skewX(3deg)
skew-x-6  → transform: skewX(6deg)
skew-x-12 → transform: skewX(12deg)

skew-y-0  → transform: skewY(0deg)
skew-y-1  → transform: skewY(1deg)
skew-y-2  → transform: skewY(2deg)
skew-y-3  → transform: skewY(3deg)
skew-y-6  → transform: skewY(6deg)
skew-y-12 → transform: skewY(12deg)

# Negative skew
-skew-x-1, -skew-x-2, -skew-x-3, -skew-x-6, -skew-x-12
-skew-y-1, -skew-y-2, -skew-y-3, -skew-y-6, -skew-y-12
```

## GPU Acceleration
```
transform-cpu → transform: translate(var(--tw-translate-x), var(--tw-translate-y)) ...
transform-gpu → transform: translate3d(...) (enables GPU acceleration)
transform-none → transform: none
```

## Common Patterns

### Hover Scale Effect
```html
<button class="transform hover:scale-105 transition-transform">
  Grow on hover
</button>
```

### Centered Absolute Element
```html
<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
  Perfectly centered
</div>
```

### Rotation Animation
```html
<svg class="animate-spin">
  <!-- Spinning loader -->
</svg>
```

### Flip on Hover
```html
<div class="hover:rotate-180 transition-transform duration-500">
  Flip me
</div>
```

### Card Tilt Effect
```html
<div class="hover:-rotate-2 hover:scale-105 transition-transform">
  Tilted card
</div>
```

### Slide In from Side
```html
<!-- Start position -->
<div class="translate-x-full">

<!-- End position -->
<div class="translate-x-0">
```

### Mirror/Flip Image
```html
<img class="scale-x-[-1]" src="image.jpg" alt="">
<!-- Or using negative scale -->
<img class="-scale-x-100" src="image.jpg" alt="">
```

### Skewed Background
```html
<div class="relative">
  <div class="absolute inset-0 bg-blue-500 -skew-y-3"></div>
  <div class="relative">Content on top</div>
</div>
```

### 3D Card Rotation
```html
<div class="perspective-1000">
  <div class="transform-gpu hover:rotate-y-12 transition-transform">
    3D rotating card
  </div>
</div>
```

### Scale from Corner
```html
<div class="origin-top-left hover:scale-110 transition-transform">
  Scales from top-left corner
</div>
```

### Combining Transforms
```html
<div class="translate-x-4 rotate-45 scale-110">
  Multiple transforms applied
</div>
```
