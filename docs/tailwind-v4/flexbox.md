# Flexbox Utilities (Tailwind CSS v4)

## Display Flex

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `flex` | `display` | `flex` |
| `inline-flex` | `display` | `inline-flex` |

## Flex Direction

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `flex-row` | `flex-direction` | `row` (default) |
| `flex-row-reverse` | `flex-direction` | `row-reverse` |
| `flex-col` | `flex-direction` | `column` |
| `flex-col-reverse` | `flex-direction` | `column-reverse` |

## Flex Wrap

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `flex-wrap` | `flex-wrap` | `wrap` |
| `flex-wrap-reverse` | `flex-wrap` | `wrap-reverse` |
| `flex-nowrap` | `flex-wrap` | `nowrap` (default) |

## Justify Content (Main Axis)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `justify-normal` | `justify-content` | `normal` |
| `justify-start` | `justify-content` | `flex-start` |
| `justify-end` | `justify-content` | `flex-end` |
| `justify-end-safe` | `justify-content` | `safe flex-end` |
| `justify-center` | `justify-content` | `center` |
| `justify-center-safe` | `justify-content` | `safe center` |
| `justify-between` | `justify-content` | `space-between` |
| `justify-around` | `justify-content` | `space-around` |
| `justify-evenly` | `justify-content` | `space-evenly` |
| `justify-stretch` | `justify-content` | `stretch` |
| `justify-baseline` | `justify-content` | `baseline` |

### Safe Alignment (v4)

The `safe` keyword prevents content overflow when items don't fit:

```css
/* With safe: content stays visible even when overflowing */
justify-content: safe center;

/* Without safe: content may overflow and become inaccessible */
justify-content: center;
```

## Align Items (Cross Axis)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `items-start` | `align-items` | `flex-start` |
| `items-end` | `align-items` | `flex-end` |
| `items-end-safe` | `align-items` | `safe flex-end` |
| `items-center` | `align-items` | `center` |
| `items-center-safe` | `align-items` | `safe center` |
| `items-baseline` | `align-items` | `baseline` |
| `items-baseline-last` | `align-items` | `last baseline` |
| `items-stretch` | `align-items` | `stretch` (default) |

### Last Baseline (v4)

`items-baseline-last` aligns items to the last baseline instead of the first. Useful for aligning form inputs with their labels.

## Align Content (Multiple Rows)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `content-normal` | `align-content` | `normal` |
| `content-start` | `align-content` | `flex-start` |
| `content-end` | `align-content` | `flex-end` |
| `content-center` | `align-content` | `center` |
| `content-between` | `align-content` | `space-between` |
| `content-around` | `align-content` | `space-around` |
| `content-evenly` | `align-content` | `space-evenly` |
| `content-baseline` | `align-content` | `baseline` |
| `content-stretch` | `align-content` | `stretch` |

## Align Self

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `self-auto` | `align-self` | `auto` |
| `self-start` | `align-self` | `flex-start` |
| `self-end` | `align-self` | `flex-end` |
| `self-center` | `align-self` | `center` |
| `self-stretch` | `align-self` | `stretch` |
| `self-baseline` | `align-self` | `baseline` |

## Flex Grow & Shrink

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `grow` | `flex-grow` | `1` |
| `grow-0` | `flex-grow` | `0` |
| `shrink` | `flex-shrink` | `1` |
| `shrink-0` | `flex-shrink` | `0` |

## Flex Basis

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `basis-0` | `flex-basis` | `0px` |
| `basis-1` | `flex-basis` | `calc(var(--spacing) * 1)` (0.25rem) |
| `basis-2` | `flex-basis` | `calc(var(--spacing) * 2)` (0.5rem) |
| `basis-4` | `flex-basis` | `calc(var(--spacing) * 4)` (1rem) |
| `basis-auto` | `flex-basis` | `auto` |
| `basis-full` | `flex-basis` | `100%` |
| `basis-1/2` | `flex-basis` | `50%` |
| `basis-1/3` | `flex-basis` | `33.333333%` |
| `basis-2/3` | `flex-basis` | `66.666667%` |
| `basis-1/4` | `flex-basis` | `25%` |
| `basis-3/4` | `flex-basis` | `75%` |
| `basis-1/5` | `flex-basis` | `20%` |
| `basis-2/5` | `flex-basis` | `40%` |
| `basis-3/5` | `flex-basis` | `60%` |
| `basis-4/5` | `flex-basis` | `80%` |
| `basis-1/6` | `flex-basis` | `16.666667%` |
| `basis-5/6` | `flex-basis` | `83.333333%` |
| `basis-1/12` | `flex-basis` | `8.333333%` |

## Flex Shorthand

| Utility | CSS Property | CSS Value | Description |
|---------|--------------|-----------|-------------|
| `flex-1` | `flex` | `1 1 0%` | Grow, shrink, ignore basis |
| `flex-auto` | `flex` | `1 1 auto` | Grow, shrink, use auto basis |
| `flex-initial` | `flex` | `0 1 auto` | Don't grow, can shrink |
| `flex-none` | `flex` | `none` | Don't grow or shrink |

## Gap (Spacing Between Items)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `gap-0` | `gap` | `0px` |
| `gap-px` | `gap` | `1px` |
| `gap-1` | `gap` | `calc(var(--spacing) * 1)` (0.25rem) |
| `gap-2` | `gap` | `calc(var(--spacing) * 2)` (0.5rem) |
| `gap-4` | `gap` | `calc(var(--spacing) * 4)` (1rem) |
| `gap-8` | `gap` | `calc(var(--spacing) * 8)` (2rem) |
| `gap-x-4` | `column-gap` | `calc(var(--spacing) * 4)` |
| `gap-y-4` | `row-gap` | `calc(var(--spacing) * 4)` |

## Order

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `order-first` | `order` | `-9999` |
| `order-last` | `order` | `9999` |
| `order-none` | `order` | `0` |
| `order-1` | `order` | `1` |
| `order-2` | `order` | `2` |
| ... | ... | ... |
| `order-12` | `order` | `12` |
| `-order-1` | `order` | `-1` |

## Common Patterns

### Centering Content
```html
<div class="flex items-center justify-center">
  <!-- Centered both horizontally and vertically -->
</div>
```

### Safe Centering (v4)
```html
<div class="flex items-center-safe justify-center-safe">
  <!-- Centered but scrollable if content overflows -->
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

### Wrap with Gap
```html
<div class="flex flex-wrap gap-4">
  <div class="basis-full md:basis-1/2 lg:basis-1/3">Item</div>
  <!-- Items wrap with consistent spacing -->
</div>
```

### Baseline Alignment for Forms
```html
<div class="flex items-baseline-last gap-4">
  <label>Name</label>
  <input type="text" class="text-lg" />
  <!-- Label aligns to input's text baseline -->
</div>
```

## v3 to v4 Migration Notes

| Change | v3 | v4 |
|--------|----|----|
| Gap values | Static (`gap: 1rem`) | CSS variables (`gap: calc(var(--spacing) * 4)`) |
| `justify-center-safe` | N/A | New (safe overflow) |
| `justify-end-safe` | N/A | New (safe overflow) |
| `items-center-safe` | N/A | New (safe overflow) |
| `items-end-safe` | N/A | New (safe overflow) |
| `items-baseline-last` | N/A | New (last baseline) |
| `justify-normal` | N/A | New |
| `justify-baseline` | N/A | New |
