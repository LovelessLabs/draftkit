# Sizing Utilities (Tailwind CSS v3)

## Width

### Fixed Width (Spacing Scale)
```
w-0    → width: 0px
w-px   → width: 1px
w-0.5  → width: 0.125rem (2px)
w-1    → width: 0.25rem (4px)
w-2    → width: 0.5rem (8px)
w-4    → width: 1rem (16px)
w-8    → width: 2rem (32px)
w-16   → width: 4rem (64px)
w-32   → width: 8rem (128px)
w-64   → width: 16rem (256px)
w-96   → width: 24rem (384px)
```

### Percentage Width
```
w-1/2   → width: 50%
w-1/3   → width: 33.333333%
w-2/3   → width: 66.666667%
w-1/4   → width: 25%
w-3/4   → width: 75%
w-1/5   → width: 20%
w-2/5   → width: 40%
w-3/5   → width: 60%
w-4/5   → width: 80%
w-1/6   → width: 16.666667%
w-5/6   → width: 83.333333%
w-1/12  → width: 8.333333%
w-11/12 → width: 91.666667%
w-full  → width: 100%
```

### Viewport Width
```
w-screen → width: 100vw
```

### Special Values
```
w-auto   → width: auto
w-min    → width: min-content
w-max    → width: max-content
w-fit    → width: fit-content
```

## Height

### Fixed Height (Spacing Scale)
```
h-0    → height: 0px
h-px   → height: 1px
h-0.5  → height: 0.125rem (2px)
h-1    → height: 0.25rem (4px)
h-2    → height: 0.5rem (8px)
h-4    → height: 1rem (16px)
h-8    → height: 2rem (32px)
h-16   → height: 4rem (64px)
h-32   → height: 8rem (128px)
h-64   → height: 16rem (256px)
h-96   → height: 24rem (384px)
```

### Percentage Height
```
h-1/2   → height: 50%
h-1/3   → height: 33.333333%
h-2/3   → height: 66.666667%
h-1/4   → height: 25%
h-3/4   → height: 75%
h-1/5   → height: 20%
h-2/5   → height: 40%
h-3/5   → height: 60%
h-4/5   → height: 80%
h-1/6   → height: 16.666667%
h-5/6   → height: 83.333333%
h-full  → height: 100%
```

### Viewport Height
```
h-screen → height: 100vh
h-svh    → height: 100svh (small viewport)
h-lvh    → height: 100lvh (large viewport)
h-dvh    → height: 100dvh (dynamic viewport)
```

### Special Values
```
h-auto   → height: auto
h-min    → height: min-content
h-max    → height: max-content
h-fit    → height: fit-content
```

## Size (Width + Height)

Sets both width and height simultaneously:
```
size-0    → width: 0px; height: 0px
size-px   → width: 1px; height: 1px
size-1    → width: 0.25rem; height: 0.25rem
size-4    → width: 1rem; height: 1rem
size-8    → width: 2rem; height: 2rem
size-16   → width: 4rem; height: 4rem
size-full → width: 100%; height: 100%
```

## Min-Width
```
min-w-0    → min-width: 0px
min-w-full → min-width: 100%
min-w-min  → min-width: min-content
min-w-max  → min-width: max-content
min-w-fit  → min-width: fit-content
```

## Max-Width
```
max-w-0     → max-width: 0rem
max-w-none  → max-width: none
max-w-xs    → max-width: 20rem (320px)
max-w-sm    → max-width: 24rem (384px)
max-w-md    → max-width: 28rem (448px)
max-w-lg    → max-width: 32rem (512px)
max-w-xl    → max-width: 36rem (576px)
max-w-2xl   → max-width: 42rem (672px)
max-w-3xl   → max-width: 48rem (768px)
max-w-4xl   → max-width: 56rem (896px)
max-w-5xl   → max-width: 64rem (1024px)
max-w-6xl   → max-width: 72rem (1152px)
max-w-7xl   → max-width: 80rem (1280px)
max-w-full  → max-width: 100%
max-w-min   → max-width: min-content
max-w-max   → max-width: max-content
max-w-fit   → max-width: fit-content
max-w-prose → max-width: 65ch
max-w-screen-sm → max-width: 640px
max-w-screen-md → max-width: 768px
max-w-screen-lg → max-width: 1024px
max-w-screen-xl → max-width: 1280px
max-w-screen-2xl → max-width: 1536px
```

## Min-Height
```
min-h-0      → min-height: 0px
min-h-full   → min-height: 100%
min-h-screen → min-height: 100vh
min-h-svh    → min-height: 100svh
min-h-lvh    → min-height: 100lvh
min-h-dvh    → min-height: 100dvh
min-h-min    → min-height: min-content
min-h-max    → min-height: max-content
min-h-fit    → min-height: fit-content
```

## Max-Height
```
max-h-0      → max-height: 0px
max-h-px     → max-height: 1px
max-h-1      → max-height: 0.25rem
max-h-full   → max-height: 100%
max-h-screen → max-height: 100vh
max-h-svh    → max-height: 100svh
max-h-lvh    → max-height: 100lvh
max-h-dvh    → max-height: 100dvh
max-h-min    → max-height: min-content
max-h-max    → max-height: max-content
max-h-fit    → max-height: fit-content
max-h-none   → max-height: none
```

## Common Patterns

### Centered Container
```html
<div class="max-w-7xl mx-auto px-4">
  <!-- Content constrained to 1280px, centered -->
</div>
```

### Full-Height Layout
```html
<div class="min-h-screen flex flex-col">
  <header>Header</header>
  <main class="flex-1">Content</main>
  <footer>Footer</footer>
</div>
```

### Square Avatar
```html
<img class="size-12 rounded-full" src="avatar.jpg" alt="">
```

### Responsive Width
```html
<div class="w-full md:w-1/2 lg:w-1/3">
  <!-- Full width on mobile, half on medium, third on large -->
</div>
```

### Prose Container
```html
<article class="max-w-prose mx-auto">
  <!-- Optimal reading width (~65 characters) -->
</article>
```

### Fixed Sidebar
```html
<aside class="w-64 shrink-0">Sidebar</aside>
<main class="flex-1 min-w-0">Content</main>
```

### Aspect Ratio with Size
```html
<div class="w-full aspect-video">
  <!-- 16:9 aspect ratio -->
</div>
```
