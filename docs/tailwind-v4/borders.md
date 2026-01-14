# Border Utilities (Tailwind CSS v4)

## Border Width

### All Sides
```
border-0 → border-width: 0px
border   → border-width: 1px
border-2 → border-width: 2px
border-4 → border-width: 4px
border-8 → border-width: 8px
```

### Individual Sides
```
border-t-0 → border-top-width: 0px
border-t   → border-top-width: 1px
border-t-2 → border-top-width: 2px

border-r-0 → border-right-width: 0px
border-r   → border-right-width: 1px
border-r-2 → border-right-width: 2px

border-b-0 → border-bottom-width: 0px
border-b   → border-bottom-width: 1px
border-b-2 → border-bottom-width: 2px

border-l-0 → border-left-width: 0px
border-l   → border-left-width: 1px
border-l-2 → border-left-width: 2px
```

### Horizontal & Vertical
```
border-x-0 → border-left/right-width: 0px
border-x   → border-left/right-width: 1px
border-x-2 → border-left/right-width: 2px

border-y-0 → border-top/bottom-width: 0px
border-y   → border-top/bottom-width: 1px
border-y-2 → border-top/bottom-width: 2px
```

### Logical Properties (v4)
```
border-s-0 → border-inline-start-width: 0px (respects RTL)
border-s   → border-inline-start-width: 1px
border-e-0 → border-inline-end-width: 0px
border-e   → border-inline-end-width: 1px
```

## Border Color

See colors.md for full palette.
```
border-inherit     → border-color: inherit
border-current     → border-color: currentColor
border-transparent → border-color: transparent
border-black       → border-color: #000
border-white       → border-color: #fff
border-gray-200    → border-color: rgb(229 231 235)
border-red-500     → border-color: rgb(239 68 68)
```

### Side-Specific Colors
```
border-t-red-500 → border-top-color: rgb(239 68 68)
border-r-blue-500 → border-right-color: rgb(59 130 246)
border-b-green-500 → border-bottom-color: rgb(34 197 94)
border-l-yellow-500 → border-left-color: rgb(234 179 8)
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
rounded-full → border-radius: 9999px (pill/circle)
```

### Individual Corners
```
rounded-t-lg  → border-top-left/right-radius: 0.5rem
rounded-r-lg  → border-top/bottom-right-radius: 0.5rem
rounded-b-lg  → border-bottom-left/right-radius: 0.5rem
rounded-l-lg  → border-top/bottom-left-radius: 0.5rem

rounded-tl-lg → border-top-left-radius: 0.5rem
rounded-tr-lg → border-top-right-radius: 0.5rem
rounded-br-lg → border-bottom-right-radius: 0.5rem
rounded-bl-lg → border-bottom-left-radius: 0.5rem
```

### Logical Properties (v4)
```
rounded-s-lg  → border-start-start/end-radius: 0.5rem (respects RTL)
rounded-e-lg  → border-end-start/end-radius: 0.5rem
rounded-ss-lg → border-start-start-radius: 0.5rem
rounded-se-lg → border-start-end-radius: 0.5rem
rounded-es-lg → border-end-start-radius: 0.5rem
rounded-ee-lg → border-end-end-radius: 0.5rem
```

## Divide (Between Children)

Adds borders between flex/grid children:
```
divide-x-0 → border-left-width: 0px (on children)
divide-x   → border-left-width: 1px
divide-x-2 → border-left-width: 2px

divide-y-0 → border-top-width: 0px
divide-y   → border-top-width: 1px
divide-y-2 → border-top-width: 2px
```

### Divide Color
```
divide-gray-200   → border-color: rgb(229 231 235)
divide-transparent → border-color: transparent
```

### Divide Style
```
divide-solid  → border-style: solid
divide-dashed → border-style: dashed
divide-dotted → border-style: dotted
divide-none   → border-style: none
```

### Reverse Order
```
divide-x-reverse → reverses border placement (for flex-row-reverse)
divide-y-reverse → reverses border placement (for flex-col-reverse)
```

## Outline

### Width
```
outline-0 → outline-width: 0px
outline-1 → outline-width: 1px
outline-2 → outline-width: 2px
outline-4 → outline-width: 4px
outline-8 → outline-width: 8px
```

### Style
```
outline-none   → outline: 2px solid transparent; outline-offset: 2px
outline        → outline-style: solid
outline-dashed → outline-style: dashed
outline-dotted → outline-style: dotted
outline-double → outline-style: double
```

### Color
```
outline-inherit     → outline-color: inherit
outline-current     → outline-color: currentColor
outline-transparent → outline-color: transparent
outline-black       → outline-color: #000
outline-white       → outline-color: #fff
outline-blue-500    → outline-color: rgb(59 130 246)
```

### Offset
```
outline-offset-0 → outline-offset: 0px
outline-offset-1 → outline-offset: 1px
outline-offset-2 → outline-offset: 2px
outline-offset-4 → outline-offset: 4px
outline-offset-8 → outline-offset: 8px
```

## Ring (Focus Rings)

### Width
```
ring-0 → box-shadow: none (ring)
ring-1 → box-shadow: 0 0 0 1px (ring)
ring-2 → box-shadow: 0 0 0 2px (ring)
ring   → box-shadow: 0 0 0 3px (ring)
ring-4 → box-shadow: 0 0 0 4px (ring)
ring-8 → box-shadow: 0 0 0 8px (ring)
ring-inset → box-shadow: inset 0 0 0 (ring)
```

### Color
```
ring-inherit     → --tw-ring-color: inherit
ring-current     → --tw-ring-color: currentColor
ring-transparent → --tw-ring-color: transparent
ring-black       → --tw-ring-color: #000
ring-white       → --tw-ring-color: #fff
ring-blue-500    → --tw-ring-color: rgb(59 130 246)
```

### Offset
```
ring-offset-0 → --tw-ring-offset-width: 0px
ring-offset-1 → --tw-ring-offset-width: 1px
ring-offset-2 → --tw-ring-offset-width: 2px
ring-offset-4 → --tw-ring-offset-width: 4px
ring-offset-8 → --tw-ring-offset-width: 8px
```

### Offset Color
```
ring-offset-white   → --tw-ring-offset-color: #fff
ring-offset-gray-50 → --tw-ring-offset-color: rgb(249 250 251)
```

## Common Patterns

### Basic Card
```html
<div class="rounded-lg border border-gray-200 p-4">
  Card content
</div>
```

### Input Field
```html
<input class="rounded-md border border-gray-300 focus:border-blue-500 focus:ring-2 focus:ring-blue-500" />
```

### Button with Ring Focus
```html
<button class="rounded-md bg-blue-500 px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
  Click me
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

### Avatar with Border
```html
<img class="size-12 rounded-full border-2 border-white ring-2 ring-gray-200" src="..." />
```

### Tab with Bottom Border
```html
<button class="border-b-2 border-blue-500 px-4 py-2">
  Active Tab
</button>
```
