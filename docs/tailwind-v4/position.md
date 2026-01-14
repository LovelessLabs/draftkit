# Position Utilities (Tailwind CSS v4)

## Position Type

```
static   → position: static
fixed    → position: fixed
absolute → position: absolute
relative → position: relative
sticky   → position: sticky
```

## Top / Right / Bottom / Left

### Fixed Values
```
inset-0    → top: 0; right: 0; bottom: 0; left: 0
inset-px   → top: 1px; right: 1px; bottom: 1px; left: 1px
inset-0.5  → top: 0.125rem; right: 0.125rem; bottom: 0.125rem; left: 0.125rem
inset-1    → top: 0.25rem; ...
inset-2    → top: 0.5rem; ...
inset-4    → top: 1rem; ...
inset-8    → top: 2rem; ...
inset-auto → top: auto; right: auto; bottom: auto; left: auto
```

### X/Y Axis
```
inset-x-0    → left: 0; right: 0
inset-x-4    → left: 1rem; right: 1rem
inset-x-auto → left: auto; right: auto

inset-y-0    → top: 0; bottom: 0
inset-y-4    → top: 1rem; bottom: 1rem
inset-y-auto → top: auto; bottom: auto
```

### Individual Sides
```
top-0    → top: 0px
top-px   → top: 1px
top-1    → top: 0.25rem
top-4    → top: 1rem
top-auto → top: auto

right-0    → right: 0px
right-4    → right: 1rem
right-auto → right: auto

bottom-0    → bottom: 0px
bottom-4    → bottom: 1rem
bottom-auto → bottom: auto

left-0    → left: 0px
left-4    → left: 1rem
left-auto → left: auto
```

### Percentage Values
```
inset-1/2  → top: 50%; right: 50%; bottom: 50%; left: 50%
inset-1/3  → top: 33.333%; ...
inset-full → top: 100%; ...

top-1/2    → top: 50%
top-1/3    → top: 33.333%
top-full   → top: 100%

left-1/2   → left: 50%
left-full  → left: 100%
```

### Negative Values
```
-inset-1   → top: -0.25rem; right: -0.25rem; bottom: -0.25rem; left: -0.25rem
-top-4     → top: -1rem
-right-4   → right: -1rem
-bottom-4  → bottom: -1rem
-left-4    → left: -1rem
```

### Logical Properties (v4)
```
inset-inline-0       → inset-inline: 0px (left+right in RTL-aware way)
inset-inline-start-0 → inset-inline-start: 0px
inset-inline-end-0   → inset-inline-end: 0px

start-0    → inset-inline-start: 0px (respects RTL)
start-4    → inset-inline-start: 1rem
end-0      → inset-inline-end: 0px
end-4      → inset-inline-end: 1rem
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

-z-10  → z-index: -10
-z-50  → z-index: -50
```

## Float

```
float-start → float: inline-start (v4 - RTL-aware)
float-end   → float: inline-end (v4 - RTL-aware)
float-right → float: right
float-left  → float: left
float-none  → float: none
```

## Clear

```
clear-start → clear: inline-start (v4)
clear-end   → clear: inline-end (v4)
clear-left  → clear: left
clear-right → clear: right
clear-both  → clear: both
clear-none  → clear: none
```

## Isolation

```
isolate       → isolation: isolate
isolation-auto → isolation: auto
```

## Object Position (for replaced elements)

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

## Object Fit

```
object-contain    → object-fit: contain
object-cover      → object-fit: cover
object-fill       → object-fit: fill
object-none       → object-fit: none
object-scale-down → object-fit: scale-down
```

## Arbitrary Values

```
top-[117px]      → top: 117px
left-[3rem]      → left: 3rem
inset-[10px]     → inset: 10px
z-[100]          → z-index: 100
z-[999]          → z-index: 999
```

## Common Patterns

### Full-Screen Overlay
```html
<div class="fixed inset-0 bg-black/50 z-50">
  <!-- Covers entire viewport -->
</div>
```

### Centered Modal
```html
<div class="fixed inset-0 flex items-center justify-center z-50">
  <div class="relative bg-white rounded-lg p-6">
    Modal content
  </div>
</div>
```

### Absolutely Centered Element
```html
<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
  <!-- Perfectly centered in parent -->
</div>
```

### Sticky Header
```html
<header class="sticky top-0 z-40 bg-white">
  <!-- Sticks to top on scroll -->
</header>
```

### Corner Badge
```html
<div class="relative">
  <img src="..." />
  <span class="absolute top-0 right-0 -translate-y-1/2 translate-x-1/2">
    Badge
  </span>
</div>
```

### Notification Dot
```html
<div class="relative">
  <button>Bell Icon</button>
  <span class="absolute top-0 right-0 h-2 w-2 rounded-full bg-red-500"></span>
</div>
```

### Full-Bleed Image
```html
<img class="absolute inset-0 h-full w-full object-cover" src="..." />
```

### Fixed Footer
```html
<footer class="fixed bottom-0 inset-x-0 z-30">
  <!-- Fixed to bottom -->
</footer>
```

### Dropdown Menu
```html
<div class="relative">
  <button>Menu</button>
  <div class="absolute top-full left-0 mt-2 z-50">
    <!-- Positioned below button -->
  </div>
</div>
```

### Layered Elements
```html
<div class="relative">
  <div class="absolute z-10">Layer 1</div>
  <div class="absolute z-20">Layer 2 (on top)</div>
  <div class="absolute z-30">Layer 3 (topmost)</div>
</div>
```
