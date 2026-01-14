# Color Utilities (Tailwind CSS v3)

## Color Palette

Tailwind includes a comprehensive color palette with shades from 50 (lightest) to 950 (darkest):

### Gray Scales
```
slate   → blue-tinted gray
gray    → neutral gray
zinc    → cool gray
neutral → true neutral
stone   → warm gray
```

### Colors
```
red, orange, amber, yellow, lime, green, emerald,
teal, cyan, sky, blue, indigo, violet, purple, fuchsia, pink, rose
```

### Special Colors
```
inherit     → inherits from parent
current     → currentColor
transparent → transparent
black       → #000000
white       → #ffffff
```

## Color Shade Scale
```
{color}-50   → lightest
{color}-100
{color}-200
{color}-300
{color}-400
{color}-500  → base/middle
{color}-600
{color}-700
{color}-800
{color}-900
{color}-950  → darkest
```

## Text Color
```
text-black       → color: #000
text-white       → color: #fff
text-gray-500    → color: #6b7280
text-red-500     → color: #ef4444
text-blue-500    → color: #3b82f6
text-green-500   → color: #22c55e
text-inherit     → color: inherit
text-current     → color: currentColor
text-transparent → color: transparent
```

## Background Color
```
bg-black       → background-color: #000
bg-white       → background-color: #fff
bg-gray-100    → background-color: #f3f4f6
bg-gray-500    → background-color: #6b7280
bg-blue-500    → background-color: #3b82f6
bg-inherit     → background-color: inherit
bg-current     → background-color: currentColor
bg-transparent → background-color: transparent
```

## Border Color
```
border-black     → border-color: #000
border-white     → border-color: #fff
border-gray-300  → border-color: #d1d5db
border-red-500   → border-color: #ef4444
border-inherit   → border-color: inherit
border-current   → border-color: currentColor
border-transparent → border-color: transparent
```

## Ring Color
```
ring-black    → --tw-ring-color: #000
ring-white    → --tw-ring-color: #fff
ring-blue-500 → --tw-ring-color: #3b82f6
ring-inherit  → --tw-ring-color: inherit
ring-current  → --tw-ring-color: currentColor
```

## Divide Color
```
divide-gray-200 → border-color: #e5e7eb (on child elements)
divide-black    → border-color: #000
```

## Placeholder Color
```
placeholder-gray-400 → ::placeholder { color: #9ca3af }
placeholder-gray-500 → ::placeholder { color: #6b7280 }
```

## Caret Color (Cursor)
```
caret-blue-500 → caret-color: #3b82f6
caret-black    → caret-color: #000
```

## Accent Color (Form Controls)
```
accent-blue-500 → accent-color: #3b82f6
accent-auto     → accent-color: auto
```

## Opacity Modifiers

Use `/` to add opacity to any color:
```
text-black/50   → color: rgb(0 0 0 / 0.5)
text-black/75   → color: rgb(0 0 0 / 0.75)
bg-white/80     → background-color: rgb(255 255 255 / 0.8)
bg-blue-500/25  → background-color: rgb(59 130 246 / 0.25)
border-black/10 → border-color: rgb(0 0 0 / 0.1)
```

### Available Opacity Values
```
/0   → 0%
/5   → 5%
/10  → 10%
/20  → 20%
/25  → 25%
/30  → 30%
/40  → 40%
/50  → 50%
/60  → 60%
/70  → 70%
/75  → 75%
/80  → 80%
/90  → 90%
/95  → 95%
/100 → 100%
```

## Legacy Opacity Utilities
```
text-opacity-50   → --tw-text-opacity: 0.5
bg-opacity-75     → --tw-bg-opacity: 0.75
border-opacity-50 → --tw-border-opacity: 0.5
```

## Gradient Colors

### Gradient Direction
```
bg-gradient-to-t  → linear-gradient(to top, ...)
bg-gradient-to-tr → linear-gradient(to top right, ...)
bg-gradient-to-r  → linear-gradient(to right, ...)
bg-gradient-to-br → linear-gradient(to bottom right, ...)
bg-gradient-to-b  → linear-gradient(to bottom, ...)
bg-gradient-to-bl → linear-gradient(to bottom left, ...)
bg-gradient-to-l  → linear-gradient(to left, ...)
bg-gradient-to-tl → linear-gradient(to top left, ...)
```

### Gradient Stops
```
from-{color} → starting color
via-{color}  → middle color
to-{color}   → ending color
```

### Gradient Stop Positions (v3.4+)
```
from-10%  → gradient starts at 10%
via-50%   → middle color at 50%
to-90%    → gradient ends at 90%
```

## Common Patterns

### Text with Subtle Background
```html
<span class="bg-blue-100 text-blue-800 px-2 py-1 rounded">
  Badge
</span>
```

### Button Hover States
```html
<button class="bg-blue-500 hover:bg-blue-600 text-white">
  Click me
</button>
```

### Gradient Background
```html
<div class="bg-gradient-to-r from-purple-500 to-pink-500">
  Gradient
</div>
```

### Transparent Overlay
```html
<div class="bg-black/50">
  Semi-transparent overlay
</div>
```

### Border with Low Opacity
```html
<div class="border border-gray-200/50">
  Subtle border
</div>
```

### Focus Ring
```html
<button class="focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
  Focusable
</button>
```

### Dark Mode Colors
```html
<div class="bg-white dark:bg-gray-900 text-gray-900 dark:text-white">
  Adapts to dark mode
</div>
```

## Color Reference (Selected)

### Gray-500 Variants
```
slate-500   → #64748b
gray-500    → #6b7280
zinc-500    → #71717a
neutral-500 → #737373
stone-500   → #78716c
```

### Primary Colors at 500
```
red-500     → #ef4444
orange-500  → #f97316
amber-500   → #f59e0b
yellow-500  → #eab308
lime-500    → #84cc16
green-500   → #22c55e
emerald-500 → #10b981
teal-500    → #14b8a6
cyan-500    → #06b6d4
sky-500     → #0ea5e9
blue-500    → #3b82f6
indigo-500  → #6366f1
violet-500  → #8b5cf6
purple-500  → #a855f7
fuchsia-500 → #d946ef
pink-500    → #ec4899
rose-500    → #f43f5e
```
