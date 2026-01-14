# Tailwind CSS v4 Utility Mappings

Extracted from `tailwindcss-4.1.18/packages/tailwindcss/src/utilities.ts`.

## Spacing Utilities

### Padding

| Utility | CSS Property | CSS Value Pattern |
|---------|--------------|-------------------|
| `p-<n>` | `padding` | `calc(var(--spacing) * <n>)` |
| `px-<n>` | `padding-inline` | `calc(var(--spacing) * <n>)` |
| `py-<n>` | `padding-block` | `calc(var(--spacing) * <n>)` |
| `ps-<n>` | `padding-inline-start` | `calc(var(--spacing) * <n>)` |
| `pe-<n>` | `padding-inline-end` | `calc(var(--spacing) * <n>)` |
| `pt-<n>` | `padding-top` | `calc(var(--spacing) * <n>)` |
| `pr-<n>` | `padding-right` | `calc(var(--spacing) * <n>)` |
| `pb-<n>` | `padding-bottom` | `calc(var(--spacing) * <n>)` |
| `pl-<n>` | `padding-left` | `calc(var(--spacing) * <n>)` |
| `p-px` | `padding` | `1px` |

### Margin

| Utility | CSS Property | CSS Value Pattern |
|---------|--------------|-------------------|
| `m-<n>` | `margin` | `calc(var(--spacing) * <n>)` |
| `mx-<n>` | `margin-inline` | `calc(var(--spacing) * <n>)` |
| `my-<n>` | `margin-block` | `calc(var(--spacing) * <n>)` |
| `ms-<n>` | `margin-inline-start` | `calc(var(--spacing) * <n>)` |
| `me-<n>` | `margin-inline-end` | `calc(var(--spacing) * <n>)` |
| `mt-<n>` | `margin-top` | `calc(var(--spacing) * <n>)` |
| `mr-<n>` | `margin-right` | `calc(var(--spacing) * <n>)` |
| `mb-<n>` | `margin-bottom` | `calc(var(--spacing) * <n>)` |
| `ml-<n>` | `margin-left` | `calc(var(--spacing) * <n>)` |
| `m-auto` | `margin` | `auto` |
| `-m-<n>` | `margin` | `calc(var(--spacing) * -<n>)` |

### Gap

| Utility | CSS Property | CSS Value Pattern |
|---------|--------------|-------------------|
| `gap-<n>` | `gap` | `calc(var(--spacing) * <n>)` |
| `gap-x-<n>` | `column-gap` | `calc(var(--spacing) * <n>)` |
| `gap-y-<n>` | `row-gap` | `calc(var(--spacing) * <n>)` |

## Transform Utilities

### Scale (Native CSS Property)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `scale-<n>` | `scale` | `<n>%` |
| `scale-x-<n>` | `scale` | `<n>% var(--tw-scale-y)` |
| `scale-y-<n>` | `scale` | `var(--tw-scale-x) <n>%` |
| `scale-none` | `scale` | `none` |
| `scale-3d` | `scale` | `var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z)` |
| `-scale-<n>` | `scale` | `calc(<n>% * -1)` |

### Rotate (Native CSS Property)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `rotate-<n>` | `rotate` | `<n>deg` |
| `-rotate-<n>` | `rotate` | `calc(<n>deg * -1)` |
| `rotate-none` | `rotate` | `none` |

### Translate (Native CSS Property)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `translate-<n>` | `translate` | `var(--tw-translate-x) var(--tw-translate-y)` |
| `translate-x-<n>` | `translate` | `<value> var(--tw-translate-y)` |
| `translate-y-<n>` | `translate` | `var(--tw-translate-x) <value>` |
| `translate-z-<n>` | `translate` | `... var(--tw-translate-z)` |
| `translate-none` | `translate` | `none` |
| `translate-full` | `translate` | `100% 100%` |
| `-translate-full` | `translate` | `-100% -100%` |
| `translate-1/2` | `translate` | `50% 50%` |
| `translate-3d` | `translate` | `var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z)` |

## Filter Utilities

### Blur

| Utility | CSS Output |
|---------|------------|
| `blur-xs` | `filter: blur(var(--blur-xs))` → `blur(4px)` |
| `blur-sm` | `filter: blur(var(--blur-sm))` → `blur(8px)` |
| `blur-md` | `filter: blur(var(--blur-md))` → `blur(12px)` |
| `blur-lg` | `filter: blur(var(--blur-lg))` → `blur(16px)` |
| `blur-xl` | `filter: blur(var(--blur-xl))` → `blur(24px)` |
| `blur-2xl` | `filter: blur(var(--blur-2xl))` → `blur(40px)` |
| `blur-3xl` | `filter: blur(var(--blur-3xl))` → `blur(64px)` |
| `blur-none` | `filter: blur(0)` |

### Drop Shadow

| Utility | CSS Output |
|---------|------------|
| `drop-shadow-xs` | `filter: drop-shadow(var(--drop-shadow-xs))` |
| `drop-shadow-sm` | `filter: drop-shadow(var(--drop-shadow-sm))` |
| `drop-shadow-md` | `filter: drop-shadow(var(--drop-shadow-md))` |
| `drop-shadow-lg` | `filter: drop-shadow(var(--drop-shadow-lg))` |
| `drop-shadow-xl` | `filter: drop-shadow(var(--drop-shadow-xl))` |
| `drop-shadow-2xl` | `filter: drop-shadow(var(--drop-shadow-2xl))` |
| `drop-shadow-none` | `filter: none` |

## Typography Utilities

### Font Size

| Utility | CSS Output |
|---------|------------|
| `text-xs` | `font-size: var(--text-xs); line-height: var(--text-xs--line-height)` |
| `text-sm` | `font-size: var(--text-sm); line-height: var(--text-sm--line-height)` |
| `text-base` | `font-size: var(--text-base); line-height: var(--text-base--line-height)` |
| `text-lg` | `font-size: var(--text-lg); line-height: var(--text-lg--line-height)` |
| ... | ... |

### Font Size with Line Height Modifier

`text-sm/6` → `font-size: var(--text-sm); line-height: calc(var(--spacing) * 6)`

### Letter Spacing

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `tracking-tighter` | `letter-spacing` | `var(--tracking-tighter)` |
| `tracking-tight` | `letter-spacing` | `var(--tracking-tight)` |
| `tracking-normal` | `letter-spacing` | `var(--tracking-normal)` |
| `tracking-wide` | `letter-spacing` | `var(--tracking-wide)` |
| `tracking-wider` | `letter-spacing` | `var(--tracking-wider)` |
| `tracking-widest` | `letter-spacing` | `var(--tracking-widest)` |

## Flexbox Utilities

### Justify Content

| Utility | CSS Value |
|---------|-----------|
| `justify-start` | `justify-content: flex-start` |
| `justify-end` | `justify-content: flex-end` |
| `justify-end-safe` | `justify-content: safe flex-end` |
| `justify-center` | `justify-content: center` |
| `justify-center-safe` | `justify-content: safe center` |
| `justify-between` | `justify-content: space-between` |
| `justify-around` | `justify-content: space-around` |
| `justify-evenly` | `justify-content: space-evenly` |
| `justify-stretch` | `justify-content: stretch` |
| `justify-baseline` | `justify-content: baseline` |
| `justify-normal` | `justify-content: normal` |

### Align Items

| Utility | CSS Value |
|---------|-----------|
| `items-start` | `align-items: flex-start` |
| `items-end` | `align-items: flex-end` |
| `items-end-safe` | `align-items: safe flex-end` |
| `items-center` | `align-items: center` |
| `items-center-safe` | `align-items: safe center` |
| `items-baseline` | `align-items: baseline` |
| `items-baseline-last` | `align-items: last baseline` |
| `items-stretch` | `align-items: stretch` |

## Key Patterns

### Spacing Calculation

All spacing-based utilities resolve through `spacingUtility()` which:

1. Gets `--spacing` value from theme (default: `0.25rem`)
2. Multiplies by the numeric value: `calc(0.25rem * <n>)`

### Theme Key Resolution

Utilities specify `themeKeys` array for value resolution priority:

```typescript
spacingUtility('p', ['--padding', '--spacing'], ...)
// Tries --padding-* first, falls back to --spacing
```

### Native CSS Properties

v4 uses native CSS transform properties (not `transform: ...`):

- `scale` property instead of `transform: scale()`
- `rotate` property instead of `transform: rotate()`
- `translate` property instead of `transform: translate()`

### Logical Properties

v4 uses CSS Logical Properties for direction-agnostic utilities:

- `padding-inline` instead of `padding-left` + `padding-right`
- `padding-block` instead of `padding-top` + `padding-bottom`
- `margin-inline` instead of `margin-left` + `margin-right`
- `margin-block` instead of `margin-top` + `margin-bottom`
