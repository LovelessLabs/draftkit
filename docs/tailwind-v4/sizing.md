# Sizing Utilities (Tailwind CSS v4)

## Width

### Fixed Widths
```
w-0    → width: 0px
w-px   → width: 1px
w-0.5  → width: 0.125rem (2px)
w-1    → width: 0.25rem (4px)
w-2    → width: 0.5rem (8px)
w-4    → width: 1rem (16px)
w-8    → width: 2rem (32px)
w-12   → width: 3rem (48px)
w-16   → width: 4rem (64px)
w-24   → width: 6rem (96px)
w-32   → width: 8rem (128px)
w-48   → width: 12rem (192px)
w-64   → width: 16rem (256px)
w-80   → width: 20rem (320px)
w-96   → width: 24rem (384px)
```

### Percentage Widths
```
w-1/2   → width: 50%
w-1/3   → width: 33.333%
w-2/3   → width: 66.667%
w-1/4   → width: 25%
w-3/4   → width: 75%
w-1/5   → width: 20%
w-2/5   → width: 40%
w-3/5   → width: 60%
w-4/5   → width: 80%
w-1/6   → width: 16.667%
w-5/6   → width: 83.333%
w-full  → width: 100%
```

### Special Widths
```
w-auto   → width: auto
w-screen → width: 100vw
w-svw    → width: 100svw (small viewport)
w-lvw    → width: 100lvw (large viewport)
w-dvw    → width: 100dvw (dynamic viewport)
w-min    → width: min-content
w-max    → width: max-content
w-fit    → width: fit-content
```

## Height

### Fixed Heights
```
h-0    → height: 0px
h-px   → height: 1px
h-1    → height: 0.25rem (4px)
h-4    → height: 1rem (16px)
h-8    → height: 2rem (32px)
h-12   → height: 3rem (48px)
h-16   → height: 4rem (64px)
h-24   → height: 6rem (96px)
h-32   → height: 8rem (128px)
h-64   → height: 16rem (256px)
h-96   → height: 24rem (384px)
```

### Percentage Heights
```
h-1/2   → height: 50%
h-1/3   → height: 33.333%
h-2/3   → height: 66.667%
h-1/4   → height: 25%
h-3/4   → height: 75%
h-full  → height: 100%
```

### Special Heights
```
h-auto   → height: auto
h-screen → height: 100vh
h-svh    → height: 100svh (small viewport)
h-lvh    → height: 100lvh (large viewport)
h-dvh    → height: 100dvh (dynamic viewport - iOS safe)
h-min    → height: min-content
h-max    → height: max-content
h-fit    → height: fit-content
```

## Min/Max Width

### Min Width
```
min-w-0     → min-width: 0px
min-w-full  → min-width: 100%
min-w-min   → min-width: min-content
min-w-max   → min-width: max-content
min-w-fit   → min-width: fit-content
```

### Max Width
```
max-w-0     → max-width: 0px
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
max-w-prose → max-width: 65ch (optimal reading width)
max-w-screen-sm → max-width: 640px
max-w-screen-md → max-width: 768px
max-w-screen-lg → max-width: 1024px
max-w-screen-xl → max-width: 1280px
max-w-screen-2xl → max-width: 1536px
```

## Min/Max Height

### Min Height
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

### Max Height
```
max-h-0      → max-height: 0px
max-h-none   → max-height: none
max-h-full   → max-height: 100%
max-h-screen → max-height: 100vh
max-h-svh    → max-height: 100svh
max-h-lvh    → max-height: 100lvh
max-h-dvh    → max-height: 100dvh
max-h-min    → max-height: min-content
max-h-max    → max-height: max-content
max-h-fit    → max-height: fit-content
```

## Size (Width + Height)

Sets both width and height simultaneously:
```
size-0    → width: 0px; height: 0px
size-1    → width: 0.25rem; height: 0.25rem
size-4    → width: 1rem; height: 1rem
size-8    → width: 2rem; height: 2rem
size-12   → width: 3rem; height: 3rem
size-16   → width: 4rem; height: 4rem
size-full → width: 100%; height: 100%
```

## Arbitrary Values
```
w-[200px]     → width: 200px
h-[calc(100vh-4rem)] → height: calc(100vh - 4rem)
max-w-[1400px] → max-width: 1400px
```

## Common Patterns

### Full-Screen Container
```html
<div class="min-h-screen w-full">
  <!-- At least full viewport height -->
</div>
```

### Centered Content Container
```html
<div class="mx-auto max-w-7xl px-4">
  <!-- Max 1280px, centered -->
</div>
```

### Square Avatar
```html
<img class="size-12 rounded-full" src="..." />
<!-- 48x48px circle -->
```

### Responsive Image
```html
<img class="w-full h-auto" src="..." />
<!-- Full width, maintain aspect ratio -->
```

### Fixed Sidebar Layout
```html
<div class="flex">
  <aside class="w-64 shrink-0">Sidebar</aside>
  <main class="flex-1 min-w-0">Content</main>
</div>
```

### Mobile-Safe Full Height
```html
<div class="min-h-dvh">
  <!-- Respects mobile browser chrome -->
</div>
```
