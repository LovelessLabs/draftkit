# Flexbox Utilities (Tailwind CSS v3)

## Display Flex
```
flex        → display: flex
inline-flex → display: inline-flex
```

## Flex Direction
```
flex-row         → flex-direction: row (default)
flex-row-reverse → flex-direction: row-reverse
flex-col         → flex-direction: column
flex-col-reverse → flex-direction: column-reverse
```

## Flex Wrap
```
flex-wrap         → flex-wrap: wrap
flex-wrap-reverse → flex-wrap: wrap-reverse
flex-nowrap       → flex-wrap: nowrap (default)
```

## Justify Content (Main Axis)
```
justify-normal  → justify-content: normal
justify-start   → justify-content: flex-start
justify-end     → justify-content: flex-end
justify-center  → justify-content: center
justify-between → justify-content: space-between
justify-around  → justify-content: space-around
justify-evenly  → justify-content: space-evenly
justify-stretch → justify-content: stretch
```

## Align Items (Cross Axis)
```
items-start    → align-items: flex-start
items-end      → align-items: flex-end
items-center   → align-items: center
items-baseline → align-items: baseline
items-stretch  → align-items: stretch (default)
```

## Align Content (Multi-line)
```
content-normal  → align-content: normal
content-center  → align-content: center
content-start   → align-content: flex-start
content-end     → align-content: flex-end
content-between → align-content: space-between
content-around  → align-content: space-around
content-evenly  → align-content: space-evenly
content-baseline → align-content: baseline
content-stretch → align-content: stretch
```

## Align Self
```
self-auto     → align-self: auto
self-start    → align-self: flex-start
self-end      → align-self: flex-end
self-center   → align-self: center
self-stretch  → align-self: stretch
self-baseline → align-self: baseline
```

## Flex Grow & Shrink
```
grow       → flex-grow: 1
grow-0     → flex-grow: 0
shrink     → flex-shrink: 1
shrink-0   → flex-shrink: 0

# Deprecated aliases (still work)
flex-grow    → flex-grow: 1
flex-grow-0  → flex-grow: 0
flex-shrink  → flex-shrink: 1
flex-shrink-0 → flex-shrink: 0
```

## Flex Basis
```
basis-0      → flex-basis: 0px
basis-1      → flex-basis: 0.25rem (4px)
basis-2      → flex-basis: 0.5rem (8px)
basis-auto   → flex-basis: auto
basis-px     → flex-basis: 1px
basis-full   → flex-basis: 100%
basis-1/2    → flex-basis: 50%
basis-1/3    → flex-basis: 33.333333%
basis-2/3    → flex-basis: 66.666667%
basis-1/4    → flex-basis: 25%
basis-3/4    → flex-basis: 75%
basis-1/5    → flex-basis: 20%
basis-2/5    → flex-basis: 40%
basis-3/5    → flex-basis: 60%
basis-4/5    → flex-basis: 80%
basis-1/6    → flex-basis: 16.666667%
basis-5/6    → flex-basis: 83.333333%
basis-1/12   → flex-basis: 8.333333%
basis-11/12  → flex-basis: 91.666667%
```

## Flex Shorthand
```
flex-1       → flex: 1 1 0%     (grow, shrink, ignore basis)
flex-auto    → flex: 1 1 auto   (grow, shrink, use auto basis)
flex-initial → flex: 0 1 auto   (don't grow, can shrink)
flex-none    → flex: none       (don't grow or shrink)
```

## Gap (Spacing Between Items)
```
gap-0    → gap: 0px
gap-px   → gap: 1px
gap-0.5  → gap: 0.125rem (2px)
gap-1    → gap: 0.25rem (4px)
gap-2    → gap: 0.5rem (8px)
gap-4    → gap: 1rem (16px)
gap-8    → gap: 2rem (32px)
gap-x-4  → column-gap: 1rem
gap-y-4  → row-gap: 1rem
```

## Order
```
order-1      → order: 1
order-2      → order: 2
...
order-12     → order: 12
order-first  → order: -9999
order-last   → order: 9999
order-none   → order: 0
```

## Common Patterns

### Centering Content
```html
<div class="flex items-center justify-center">
  <!-- Centered both horizontally and vertically -->
</div>
```

### Space Between Items
```html
<nav class="flex justify-between items-center">
  <div>Logo</div>
  <div>Menu</div>
</nav>
```

### Responsive Flex Direction
```html
<div class="flex flex-col md:flex-row gap-4">
  <!-- Column on mobile, row on desktop -->
</div>
```

### Equal Width Columns
```html
<div class="flex">
  <div class="flex-1">Column 1</div>
  <div class="flex-1">Column 2</div>
  <div class="flex-1">Column 3</div>
</div>
```

### Fixed + Flexible Sidebar
```html
<div class="flex">
  <aside class="w-64 shrink-0">Fixed width sidebar</aside>
  <main class="flex-1">Flexible content area</main>
</div>
```

### Wrapping Items
```html
<div class="flex flex-wrap gap-4">
  <div class="w-32">Item 1</div>
  <div class="w-32">Item 2</div>
  <div class="w-32">Item 3</div>
  <!-- Items wrap to next line when container is too narrow -->
</div>
```
