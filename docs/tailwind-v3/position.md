# Position Utilities (Tailwind CSS v3)

## Position
```
static   → position: static
fixed    → position: fixed
absolute → position: absolute
relative → position: relative
sticky   → position: sticky
```

## Inset (Top/Right/Bottom/Left)

### All Sides
```
inset-0    → inset: 0px (top, right, bottom, left: 0)
inset-px   → inset: 1px
inset-0.5  → inset: 0.125rem
inset-1    → inset: 0.25rem
inset-2    → inset: 0.5rem
inset-4    → inset: 1rem
inset-8    → inset: 2rem
inset-auto → inset: auto
inset-1/2  → inset: 50%
inset-full → inset: 100%
```

### Horizontal (X-axis)
```
inset-x-0    → left: 0px; right: 0px
inset-x-1    → left: 0.25rem; right: 0.25rem
inset-x-auto → left: auto; right: auto
inset-x-1/2  → left: 50%; right: 50%
```

### Vertical (Y-axis)
```
inset-y-0    → top: 0px; bottom: 0px
inset-y-1    → top: 0.25rem; bottom: 0.25rem
inset-y-auto → top: auto; bottom: auto
```

### Individual Sides
```
top-0, top-1, top-2, top-4, top-8, top-auto, top-1/2, top-full
right-0, right-1, right-2, right-4, right-8, right-auto, right-1/2, right-full
bottom-0, bottom-1, bottom-2, bottom-4, bottom-8, bottom-auto, bottom-1/2, bottom-full
left-0, left-1, left-2, left-4, left-8, left-auto, left-1/2, left-full
```

### Logical Properties (RTL-aware)
```
start-0, start-1, start-2, start-4, start-auto, start-1/2
end-0, end-1, end-2, end-4, end-auto, end-1/2
```

### Negative Values
```
-inset-1   → inset: -0.25rem
-inset-x-4 → left: -1rem; right: -1rem
-top-2     → top: -0.5rem
-left-4    → left: -1rem
```

## Z-Index
```
z-0    → z-index: 0
z-10   → z-index: 10
z-20   → z-index: 20
z-30   → z-index: 30
z-40   → z-index: 40
z-50   → z-index: 50
z-auto → z-index: auto

# Negative
-z-10  → z-index: -10
```

## Float
```
float-start → float: inline-start
float-end   → float: inline-end
float-right → float: right
float-left  → float: left
float-none  → float: none
```

## Clear
```
clear-start → clear: inline-start
clear-end   → clear: inline-end
clear-left  → clear: left
clear-right → clear: right
clear-both  → clear: both
clear-none  → clear: none
```

## Isolation
```
isolate      → isolation: isolate
isolation-auto → isolation: auto
```

## Object Fit
```
object-contain    → object-fit: contain
object-cover      → object-fit: cover
object-fill       → object-fit: fill
object-none       → object-fit: none
object-scale-down → object-fit: scale-down
```

## Object Position
```
object-bottom       → object-position: bottom
object-center       → object-position: center
object-left         → object-position: left
object-left-bottom  → object-position: left bottom
object-left-top     → object-position: left top
object-right        → object-position: right
object-right-bottom → object-position: right bottom
object-right-top    → object-position: right top
object-top          → object-position: top
```

## Overflow
```
overflow-auto    → overflow: auto
overflow-hidden  → overflow: hidden
overflow-clip    → overflow: clip
overflow-visible → overflow: visible
overflow-scroll  → overflow: scroll

overflow-x-auto, overflow-x-hidden, overflow-x-clip, overflow-x-visible, overflow-x-scroll
overflow-y-auto, overflow-y-hidden, overflow-y-clip, overflow-y-visible, overflow-y-scroll
```

## Overscroll Behavior
```
overscroll-auto    → overscroll-behavior: auto
overscroll-contain → overscroll-behavior: contain
overscroll-none    → overscroll-behavior: none

overscroll-x-auto, overscroll-x-contain, overscroll-x-none
overscroll-y-auto, overscroll-y-contain, overscroll-y-none
```

## Common Patterns

### Centered Absolute Element
```html
<div class="relative">
  <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
    Perfectly centered
  </div>
</div>
```

### Full-Screen Overlay
```html
<div class="fixed inset-0 bg-black/50 z-50">
  Modal backdrop
</div>
```

### Sticky Header
```html
<header class="sticky top-0 z-40 bg-white shadow">
  Navigation
</header>
```

### Pinned to Corners
```html
<!-- Top-left -->
<div class="absolute top-0 left-0">Badge</div>

<!-- Top-right -->
<div class="absolute top-0 right-0">Close</div>

<!-- Bottom-right -->
<div class="absolute bottom-4 right-4">FAB</div>
```

### Stretch to Fill
```html
<div class="absolute inset-0">
  Fills parent completely
</div>
```

### Cover Image
```html
<img class="object-cover w-full h-64" src="photo.jpg" alt="">
```

### Contain Image
```html
<img class="object-contain w-full h-64" src="logo.svg" alt="">
```

### Scrollable Container
```html
<div class="overflow-y-auto h-96">
  Long scrollable content...
</div>
```

### Hidden Scrollbar
```html
<div class="overflow-hidden">
  Content is clipped
</div>
```

### Prevent Scroll Chaining
```html
<div class="overscroll-contain overflow-y-auto">
  Scroll stops at container edges
</div>
```

### Float with Clearfix
```html
<div class="flow-root">
  <img class="float-left mr-4" src="photo.jpg" alt="">
  <p>Text wraps around floated image...</p>
</div>
```

### Stacking Context
```html
<div class="isolate">
  <!-- Creates new stacking context -->
  <div class="relative z-10">Above</div>
  <div class="relative z-0">Below</div>
</div>
```
