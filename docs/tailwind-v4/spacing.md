# Spacing Utilities (Tailwind CSS v4)

## Architecture Note

Tailwind CSS v4 uses **CSS custom properties** for spacing calculations:

```css
--spacing: 0.25rem;  /* Base spacing unit */
```

All numeric spacing values resolve through: `calc(var(--spacing) * n)`

For example, `p-4` generates `padding: calc(var(--spacing) * 4)` which computes to `1rem`.

## Spacing Scale

| Key | CSS Value | Computed |
|-----|-----------|----------|
| `0` | `0px` | 0px |
| `px` | `1px` | 1px |
| `0.5` | `calc(var(--spacing) * 0.5)` | 0.125rem |
| `1` | `calc(var(--spacing) * 1)` | 0.25rem |
| `1.5` | `calc(var(--spacing) * 1.5)` | 0.375rem |
| `2` | `calc(var(--spacing) * 2)` | 0.5rem |
| `2.5` | `calc(var(--spacing) * 2.5)` | 0.625rem |
| `3` | `calc(var(--spacing) * 3)` | 0.75rem |
| `3.5` | `calc(var(--spacing) * 3.5)` | 0.875rem |
| `4` | `calc(var(--spacing) * 4)` | 1rem |
| `5` | `calc(var(--spacing) * 5)` | 1.25rem |
| `6` | `calc(var(--spacing) * 6)` | 1.5rem |
| `7` | `calc(var(--spacing) * 7)` | 1.75rem |
| `8` | `calc(var(--spacing) * 8)` | 2rem |
| `9` | `calc(var(--spacing) * 9)` | 2.25rem |
| `10` | `calc(var(--spacing) * 10)` | 2.5rem |
| `11` | `calc(var(--spacing) * 11)` | 2.75rem |
| `12` | `calc(var(--spacing) * 12)` | 3rem |
| `14` | `calc(var(--spacing) * 14)` | 3.5rem |
| `16` | `calc(var(--spacing) * 16)` | 4rem |
| `20` | `calc(var(--spacing) * 20)` | 5rem |
| `24` | `calc(var(--spacing) * 24)` | 6rem |
| `28` | `calc(var(--spacing) * 28)` | 7rem |
| `32` | `calc(var(--spacing) * 32)` | 8rem |
| `36` | `calc(var(--spacing) * 36)` | 9rem |
| `40` | `calc(var(--spacing) * 40)` | 10rem |
| `44` | `calc(var(--spacing) * 44)` | 11rem |
| `48` | `calc(var(--spacing) * 48)` | 12rem |
| `52` | `calc(var(--spacing) * 52)` | 13rem |
| `56` | `calc(var(--spacing) * 56)` | 14rem |
| `60` | `calc(var(--spacing) * 60)` | 15rem |
| `64` | `calc(var(--spacing) * 64)` | 16rem |
| `72` | `calc(var(--spacing) * 72)` | 18rem |
| `80` | `calc(var(--spacing) * 80)` | 20rem |
| `96` | `calc(var(--spacing) * 96)` | 24rem |

## Padding

### All Sides

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `p-0` | `padding` | `0px` |
| `p-px` | `padding` | `1px` |
| `p-1` | `padding` | `calc(var(--spacing) * 1)` |
| `p-2` | `padding` | `calc(var(--spacing) * 2)` |
| `p-4` | `padding` | `calc(var(--spacing) * 4)` |
| `p-8` | `padding` | `calc(var(--spacing) * 8)` |

### Horizontal & Vertical (Logical Properties)

v4 uses **CSS Logical Properties** for direction-agnostic layouts:

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `px-4` | `padding-inline` | `calc(var(--spacing) * 4)` |
| `py-4` | `padding-block` | `calc(var(--spacing) * 4)` |

**Key difference from v3**: These use single logical properties, not two physical properties.

### Individual Sides (Physical)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `pt-4` | `padding-top` | `calc(var(--spacing) * 4)` |
| `pr-4` | `padding-right` | `calc(var(--spacing) * 4)` |
| `pb-4` | `padding-bottom` | `calc(var(--spacing) * 4)` |
| `pl-4` | `padding-left` | `calc(var(--spacing) * 4)` |

### Logical Start/End

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `ps-4` | `padding-inline-start` | `calc(var(--spacing) * 4)` |
| `pe-4` | `padding-inline-end` | `calc(var(--spacing) * 4)` |

These respect text direction (LTR/RTL).

## Margin

### All Sides

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `m-0` | `margin` | `0px` |
| `m-px` | `margin` | `1px` |
| `m-1` | `margin` | `calc(var(--spacing) * 1)` |
| `m-4` | `margin` | `calc(var(--spacing) * 4)` |
| `m-auto` | `margin` | `auto` |

### Horizontal & Vertical (Logical Properties)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `mx-4` | `margin-inline` | `calc(var(--spacing) * 4)` |
| `my-4` | `margin-block` | `calc(var(--spacing) * 4)` |
| `mx-auto` | `margin-inline` | `auto` |

### Individual Sides (Physical)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `mt-4` | `margin-top` | `calc(var(--spacing) * 4)` |
| `mr-4` | `margin-right` | `calc(var(--spacing) * 4)` |
| `mb-4` | `margin-bottom` | `calc(var(--spacing) * 4)` |
| `ml-4` | `margin-left` | `calc(var(--spacing) * 4)` |

### Logical Start/End

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `ms-4` | `margin-inline-start` | `calc(var(--spacing) * 4)` |
| `me-4` | `margin-inline-end` | `calc(var(--spacing) * 4)` |

### Negative Margins

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `-m-4` | `margin` | `calc(var(--spacing) * -4)` |
| `-mt-4` | `margin-top` | `calc(var(--spacing) * -4)` |
| `-mx-4` | `margin-inline` | `calc(var(--spacing) * -4)` |
| `-my-4` | `margin-block` | `calc(var(--spacing) * -4)` |

## Gap (Flexbox & Grid)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `gap-4` | `gap` | `calc(var(--spacing) * 4)` |
| `gap-x-4` | `column-gap` | `calc(var(--spacing) * 4)` |
| `gap-y-4` | `row-gap` | `calc(var(--spacing) * 4)` |

## Space Between (Legacy)

Adds margin between direct children (excludes first child):

| Utility | Effect |
|---------|--------|
| `space-x-4` | `margin-left` on children except first |
| `space-y-4` | `margin-top` on children except first |
| `space-x-reverse` | Reverses space direction |
| `space-y-reverse` | Reverses space direction |

**Note**: Prefer `gap` utilities for flexbox/grid layouts.

## Arbitrary Values

Use brackets for custom values:

| Utility | CSS Output |
|---------|------------|
| `p-[20px]` | `padding: 20px` |
| `m-[2.5rem]` | `margin: 2.5rem` |
| `mt-[10vh]` | `margin-top: 10vh` |
| `px-[5%]` | `padding-inline: 5%` |
| `gap-[clamp(1rem,2vw,2rem)]` | `gap: clamp(1rem, 2vw, 2rem)` |

## Common Patterns

### Centered Container
```html
<div class="mx-auto max-w-4xl px-4">
  <!-- Centered with horizontal padding -->
</div>
```

### Card with Padding
```html
<div class="p-6 md:p-8">
  <!-- More padding on larger screens -->
</div>
```

### Stacked Elements with Gap
```html
<div class="flex flex-col gap-4">
  <div>Item 1</div>
  <div>Item 2</div>
  <div>Item 3</div>
</div>
```

### Section Spacing
```html
<section class="py-12 md:py-24">
  <!-- Responsive vertical padding -->
</section>
```

### Inline Elements with Gap
```html
<div class="flex gap-2">
  <button>Button 1</button>
  <button>Button 2</button>
</div>
```

### Negative Margin for Overlap
```html
<div class="-mt-8 relative z-10">
  <!-- Overlaps previous element by 2rem -->
</div>
```

## v3 to v4 Migration Notes

| v3 Output | v4 Output |
|-----------|-----------|
| `px-4` → `padding-left: 1rem; padding-right: 1rem` | `px-4` → `padding-inline: calc(var(--spacing) * 4)` |
| `py-4` → `padding-top: 1rem; padding-bottom: 1rem` | `py-4` → `padding-block: calc(var(--spacing) * 4)` |
| `mx-4` → `margin-left: 1rem; margin-right: 1rem` | `mx-4` → `margin-inline: calc(var(--spacing) * 4)` |
| `my-4` → `margin-top: 1rem; margin-bottom: 1rem` | `my-4` → `margin-block: calc(var(--spacing) * 4)` |
