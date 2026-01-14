# Colors (Tailwind CSS v4)

## Color Palette

Tailwind v4 includes a comprehensive default color palette:

### Gray Scales
```
slate    → Blue-gray (cool)
gray     → Pure gray (neutral)
zinc     → Warm gray
neutral  → True neutral
stone    → Warm brown-gray
```

### Colors
```
red      → Error states, alerts
orange   → Warning states
amber    → Caution, highlights
yellow   → Attention, warnings
lime     → Fresh, positive
green    → Success states
emerald  → Growth, money
teal     → Professional, calm
cyan     → Information
sky      → Light, airy
blue     → Primary actions, links
indigo   → Creative, branded
violet   → Premium, creative
purple   → Luxury, creative
fuchsia  → Vibrant, modern
pink     → Playful, feminine
rose     → Soft, romantic
```

## Shade Scale

Each color has shades from 50 (lightest) to 950 (darkest):
```
{color}-50   → Lightest (backgrounds)
{color}-100  → Very light
{color}-200  → Light
{color}-300  → Light accent
{color}-400  → Medium light
{color}-500  → Base color
{color}-600  → Medium dark
{color}-700  → Dark
{color}-800  → Very dark
{color}-900  → Darkest
{color}-950  → Near black (v4)
```

## Text Color
```
text-inherit     → color: inherit
text-current     → color: currentColor
text-transparent → color: transparent
text-black       → color: #000
text-white       → color: #fff
text-slate-500   → color: rgb(100 116 139)
text-gray-500    → color: rgb(107 114 128)
text-red-500     → color: rgb(239 68 68)
text-blue-500    → color: rgb(59 130 246)
text-green-500   → color: rgb(34 197 94)
```

## Background Color
```
bg-inherit     → background-color: inherit
bg-current     → background-color: currentColor
bg-transparent → background-color: transparent
bg-black       → background-color: #000
bg-white       → background-color: #fff
bg-slate-100   → background-color: rgb(241 245 249)
bg-blue-500    → background-color: rgb(59 130 246)
```

## Border Color
```
border-inherit     → border-color: inherit
border-current     → border-color: currentColor
border-transparent → border-color: transparent
border-black       → border-color: #000
border-white       → border-color: #fff
border-gray-200    → border-color: rgb(229 231 235)
border-red-500     → border-color: rgb(239 68 68)
```

## Ring Color (Focus Rings)
```
ring-inherit     → --tw-ring-color: inherit
ring-current     → --tw-ring-color: currentColor
ring-transparent → --tw-ring-color: transparent
ring-blue-500    → --tw-ring-color: rgb(59 130 246)
```

## Divide Color (Between Children)
```
divide-inherit     → border-color: inherit (on children)
divide-gray-200    → border-color: rgb(229 231 235)
```

## Outline Color
```
outline-inherit     → outline-color: inherit
outline-current     → outline-color: currentColor
outline-transparent → outline-color: transparent
outline-blue-500    → outline-color: rgb(59 130 246)
```

## Accent Color (Form Elements)
```
accent-inherit   → accent-color: inherit
accent-current   → accent-color: currentColor
accent-auto      → accent-color: auto
accent-blue-500  → accent-color: rgb(59 130 246)
```

## Caret Color (Text Cursor)
```
caret-inherit     → caret-color: inherit
caret-current     → caret-color: currentColor
caret-transparent → caret-color: transparent
caret-blue-500    → caret-color: rgb(59 130 246)
```

## Opacity Modifiers

Apply opacity to any color using `/`:
```
text-black/50    → color: rgb(0 0 0 / 0.5)
bg-blue-500/75   → background-color: rgb(59 130 246 / 0.75)
border-white/10  → border-color: rgb(255 255 255 / 0.1)
```

Common opacity values:
```
/0   → 0% (transparent)
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
/100 → 100% (opaque)
```

## Arbitrary Colors

Use any color value with brackets:
```
text-[#1da1f2]       → color: #1da1f2
bg-[rgb(255,0,0)]    → background-color: rgb(255,0,0)
border-[hsl(0,100%,50%)] → border-color: hsl(0,100%,50%)
```

## CSS Variables in v4

Tailwind v4 uses CSS custom properties for colors:
```css
/* v4 generates */
--color-blue-500: oklch(0.623 0.214 259.1);

/* Usage */
.text-blue-500 {
  color: var(--color-blue-500);
}
```

## Common Patterns

### Primary Button
```html
<button class="bg-blue-500 text-white hover:bg-blue-600">
  Click me
</button>
```

### Subtle Background
```html
<div class="bg-gray-50 dark:bg-gray-900">
  <!-- Light gray in light mode, dark in dark mode -->
</div>
```

### Error State
```html
<input class="border-red-500 text-red-600 placeholder-red-300" />
<p class="text-red-500 text-sm">Error message</p>
```

### Semi-transparent Overlay
```html
<div class="fixed inset-0 bg-black/50">
  <!-- 50% opacity black overlay -->
</div>
```

### Gradient Text
```html
<h1 class="bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent">
  Gradient Heading
</h1>
```

### Bordered Card
```html
<div class="border border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
  <!-- Adaptive border and background -->
</div>
```
