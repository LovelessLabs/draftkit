# Border Utilities (Tailwind CSS v3)

## Border Width

### All Sides
```
border-0 → border-width: 0px
border   → border-width: 1px (default)
border-2 → border-width: 2px
border-4 → border-width: 4px
border-8 → border-width: 8px
```

### Individual Sides
```
border-t-0, border-t, border-t-2, border-t-4, border-t-8
border-r-0, border-r, border-r-2, border-r-4, border-r-8
border-b-0, border-b, border-b-2, border-b-4, border-b-8
border-l-0, border-l, border-l-2, border-l-4, border-l-8
```

### Horizontal & Vertical
```
border-x-0, border-x, border-x-2, border-x-4, border-x-8
border-y-0, border-y, border-y-2, border-y-4, border-y-8
```

### Logical Properties (RTL-aware)
```
border-s-0, border-s, border-s-2, border-s-4, border-s-8 (start)
border-e-0, border-e, border-e-2, border-e-4, border-e-8 (end)
```

## Border Color
```
border-inherit     → border-color: inherit
border-current     → border-color: currentColor
border-transparent → border-color: transparent
border-black       → border-color: #000
border-white       → border-color: #fff
border-{color}-{shade} → border-color: {value}
```

### Per-Side Colors
```
border-t-{color}
border-r-{color}
border-b-{color}
border-l-{color}
border-x-{color}
border-y-{color}
border-s-{color}
border-e-{color}
```

## Border Style
```
border-solid  → border-style: solid
border-dashed → border-style: dashed
border-dotted → border-style: dotted
border-double → border-style: double
border-hidden → border-style: hidden
border-none   → border-style: none
```

## Border Radius

### All Corners
```
rounded-none → border-radius: 0px
rounded-sm   → border-radius: 0.125rem (2px)
rounded      → border-radius: 0.25rem (4px)
rounded-md   → border-radius: 0.375rem (6px)
rounded-lg   → border-radius: 0.5rem (8px)
rounded-xl   → border-radius: 0.75rem (12px)
rounded-2xl  → border-radius: 1rem (16px)
rounded-3xl  → border-radius: 1.5rem (24px)
rounded-full → border-radius: 9999px
```

### Side-specific
```
rounded-t-{size}  → top-left + top-right
rounded-r-{size}  → top-right + bottom-right
rounded-b-{size}  → bottom-right + bottom-left
rounded-l-{size}  → top-left + bottom-left
```

### Corner-specific
```
rounded-tl-{size} → border-top-left-radius
rounded-tr-{size} → border-top-right-radius
rounded-br-{size} → border-bottom-right-radius
rounded-bl-{size} → border-bottom-left-radius
```

### Logical Properties (RTL-aware)
```
rounded-s-{size}  → start corners (start-start, end-start)
rounded-e-{size}  → end corners (start-end, end-end)
rounded-ss-{size} → border-start-start-radius
rounded-se-{size} → border-start-end-radius
rounded-ee-{size} → border-end-end-radius
rounded-es-{size} → border-end-start-radius
```

## Divide Width (Between Children)
```
divide-x-0 → border-left-width: 0px (children)
divide-x   → border-left-width: 1px (children)
divide-x-2 → border-left-width: 2px (children)
divide-x-4, divide-x-8

divide-y-0 → border-top-width: 0px (children)
divide-y   → border-top-width: 1px (children)
divide-y-2, divide-y-4, divide-y-8

divide-x-reverse → reverses divide direction
divide-y-reverse
```

## Divide Color
```
divide-{color}-{shade} → border-color on children
divide-inherit
divide-current
divide-transparent
divide-black
divide-white
```

## Divide Style
```
divide-solid  → border-style: solid (children)
divide-dashed → border-style: dashed
divide-dotted → border-style: dotted
divide-double → border-style: double
divide-none   → border-style: none
```

## Outline

### Outline Width
```
outline-0 → outline-width: 0px
outline-1 → outline-width: 1px
outline-2 → outline-width: 2px
outline-4 → outline-width: 4px
outline-8 → outline-width: 8px
```

### Outline Style
```
outline-none   → outline: 2px solid transparent; outline-offset: 2px
outline        → outline-style: solid
outline-dashed → outline-style: dashed
outline-dotted → outline-style: dotted
outline-double → outline-style: double
```

### Outline Color
```
outline-{color}-{shade} → outline-color: {value}
outline-inherit
outline-current
outline-transparent
```

### Outline Offset
```
outline-offset-0 → outline-offset: 0px
outline-offset-1 → outline-offset: 1px
outline-offset-2 → outline-offset: 2px
outline-offset-4 → outline-offset: 4px
outline-offset-8 → outline-offset: 8px
```

## Ring (Focus Ring)

### Ring Width
```
ring-0   → box-shadow: none (ring)
ring-1   → box-shadow: 0 0 0 1px (ring)
ring-2   → box-shadow: 0 0 0 2px
ring     → box-shadow: 0 0 0 3px (default)
ring-4   → box-shadow: 0 0 0 4px
ring-8   → box-shadow: 0 0 0 8px
ring-inset → inset box-shadow
```

### Ring Color
```
ring-{color}-{shade} → --tw-ring-color: {value}
ring-inherit
ring-current
ring-transparent
```

### Ring Offset
```
ring-offset-0 → --tw-ring-offset-width: 0px
ring-offset-1 → --tw-ring-offset-width: 1px
ring-offset-2 → --tw-ring-offset-width: 2px
ring-offset-4 → --tw-ring-offset-width: 4px
ring-offset-8 → --tw-ring-offset-width: 8px
```

### Ring Offset Color
```
ring-offset-{color}-{shade}
ring-offset-inherit
ring-offset-current
ring-offset-transparent
ring-offset-white
ring-offset-black
```

## Common Patterns

### Card Border
```html
<div class="border border-gray-200 rounded-lg">
  Card content
</div>
```

### Pill Shape
```html
<span class="border rounded-full px-3 py-1">
  Tag
</span>
```

### Focus Ring
```html
<button class="focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
  Focused button
</button>
```

### Divided List
```html
<ul class="divide-y divide-gray-200">
  <li class="py-4">Item 1</li>
  <li class="py-4">Item 2</li>
  <li class="py-4">Item 3</li>
</ul>
```

### Asymmetric Border
```html
<div class="border-l-4 border-l-blue-500 pl-4">
  Left accent border
</div>
```

### Bottom Border Only
```html
<div class="border-b border-gray-300 pb-4">
  Section with bottom border
</div>
```

### Gradient Border
```html
<div class="relative">
  <div class="absolute inset-0 bg-gradient-to-r from-pink-500 to-purple-500 rounded-lg"></div>
  <div class="relative bg-white m-0.5 rounded-lg p-4">
    Content
  </div>
</div>
```

### Input with Ring
```html
<input class="border border-gray-300 rounded-md px-3 py-2
              focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent">
```
