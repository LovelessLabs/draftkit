# Tailwind CSS v4 Changes

## What's New in v4

Tailwind CSS v4 is a ground-up rewrite with significant improvements. This guide covers what's different from v3.

## Configuration Changes

### No More tailwind.config.js

Configuration now lives in CSS:

**v3 (JavaScript):**
```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        brand: '#ff5500'
      }
    }
  }
}
```

**v4 (CSS):**
```css
@import "tailwindcss";

@theme {
  --color-brand: #ff5500;
}
```

### Content Detection is Automatic

No more `content` array - v4 automatically detects your template files.

## New CSS-First Features

### @theme Directive

Define design tokens in CSS:
```css
@import "tailwindcss";

@theme {
  /* Colors */
  --color-primary: oklch(0.6 0.2 250);
  --color-secondary: oklch(0.7 0.15 180);

  /* Spacing */
  --spacing-18: 4.5rem;

  /* Font sizes */
  --font-size-tiny: 0.625rem;

  /* Breakpoints */
  --breakpoint-3xl: 1920px;
}
```

### @variant Directive

Create custom variants:
```css
/* Dark mode with class strategy */
@variant dark (&:where(.dark, .dark *));

/* Custom state variant */
@variant hocus (&:hover, &:focus);

/* Custom media query variant */
@variant tall (@media (min-height: 800px));
```

### @utility Directive

Create custom utilities:
```css
@utility tab-4 {
  tab-size: 4;
}

/* With variants */
@utility content-auto {
  content-visibility: auto;
}
```

## New Utilities

### Container Queries

Style based on parent container size:
```html
<div class="@container">
  <div class="@md:flex @lg:grid">
    <!-- Responsive to container, not viewport -->
  </div>
</div>
```

Container breakpoints:
```
@sm  → 20rem  (320px)
@md  → 28rem  (448px)
@lg  → 32rem  (512px)
@xl  → 36rem  (576px)
@2xl → 42rem  (672px)
```

### Text Wrapping

New text wrap utilities:
```
text-wrap    → text-wrap: wrap
text-nowrap  → text-wrap: nowrap
text-balance → text-wrap: balance (balances line lengths)
text-pretty  → text-wrap: pretty (avoids orphans)
```

### 3D Transforms

Full 3D transform support:
```
rotate-x-45  → transform: rotateX(45deg)
rotate-y-90  → transform: rotateY(90deg)
translate-z-4 → transform: translateZ(1rem)
perspective-500 → perspective: 500px
transform-3d → transform-style: preserve-3d
backface-hidden → backface-visibility: hidden
```

### Logical Properties

RTL-aware utilities:
```
/* Padding */
ps-4 → padding-inline-start: 1rem
pe-4 → padding-inline-end: 1rem

/* Margin */
ms-4 → margin-inline-start: 1rem
me-4 → margin-inline-end: 1rem

/* Border */
border-s-4 → border-inline-start-width: 4px
border-e-4 → border-inline-end-width: 4px

/* Position */
start-0 → inset-inline-start: 0
end-0 → inset-inline-end: 0

/* Border radius */
rounded-s-lg → border-start-start-radius + border-start-end-radius
rounded-e-lg → border-end-start-radius + border-end-end-radius
```

### Max-Width Breakpoints

Target below breakpoints:
```
max-sm:hidden  → @media (max-width: 639px) { display: none }
max-md:flex    → @media (max-width: 767px) { display: flex }
```

### Range Breakpoints

Target between breakpoints:
```html
<div class="md:max-lg:text-center">
  <!-- Only centered between md and lg -->
</div>
```

### Has Selector

Style parent based on child:
```html
<div class="has-[:checked]:bg-blue-100">
  <input type="checkbox" />
  <!-- Parent styled when child is checked -->
</div>

<div class="has-[>img]:p-0">
  <!-- No padding when direct child is img -->
</div>
```

### Not Selector

Apply when condition is NOT met:
```html
<li class="not-[:last-child]:border-b">
  <!-- Border on all except last -->
</li>
```

### Subgrid

CSS subgrid support:
```
grid-cols-subgrid → grid-template-columns: subgrid
grid-rows-subgrid → grid-template-rows: subgrid
```

### Field Sizing

Auto-sizing for form fields:
```
field-sizing-content → field-sizing: content
field-sizing-fixed   → field-sizing: fixed
```

### Color Mix

Mix colors with CSS:
```html
<div class="bg-red-500/blue-500">
  <!-- Not direct utility, but v4 uses oklch internally -->
</div>
```

## Changed Utilities

### Color System

v4 uses oklch internally for better color mixing:
```css
/* v4 generates */
--color-blue-500: oklch(0.623 0.214 259.1);
```

### Opacity Modifiers

Work on all color utilities:
```
bg-black/50      → background-color: rgb(0 0 0 / 0.5)
border-white/10  → border-color: rgb(255 255 255 / 0.1)
ring-blue-500/75 → --tw-ring-color: rgb(59 130 246 / 0.75)
```

### Ring Defaults

Ring now defaults to 1px instead of 3px:
```
ring   → box-shadow: 0 0 0 1px (was 3px in v3)
ring-3 → box-shadow: 0 0 0 3px (for old default)
```

### Gradient Stop Positions

Specify gradient positions:
```
from-blue-500 from-10%
via-purple-500 via-50%
to-pink-500 to-90%
```

## Removed Features

### JIT Mode

Always on - no configuration needed.

### Purge/Content Arrays

Automatic content detection replaces manual configuration.

### Some Deprecated Classes

- `overflow-ellipsis` → `text-ellipsis`
- `decoration-slice` → `box-decoration-slice`
- `decoration-clone` → `box-decoration-clone`
- `flex-grow-0` → `grow-0`
- `flex-shrink-0` → `shrink-0`

## Performance Improvements

- **Faster builds**: 10x faster than v3
- **Smaller CSS**: Better tree-shaking
- **Incremental builds**: Only rebuilds what changed
- **No PostCSS required**: Built-in processing

## Migration Tips

1. **Move config to CSS**: Convert tailwind.config.js to @theme
2. **Update imports**: Use `@import "tailwindcss"`
3. **Check ring usage**: Ring default changed from 3px to 1px
4. **Update deprecated classes**: Run migration tool or search-replace
5. **Test container queries**: New feature, start using them
6. **Review breakpoints**: New max-* and range options available

## Installation

```bash
npm install tailwindcss@next
```

```css
/* styles.css */
@import "tailwindcss";
```

No PostCSS config needed for basic usage.
