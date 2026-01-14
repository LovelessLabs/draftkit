# Tailwind CSS v3 Utility Mappings

Extracted from `tailwindcss-3.4.19/src/corePlugins.js`.

## Spacing Utilities

### Padding

| Utility | CSS Properties |
|---------|----------------|
| `p-<n>` | `padding: <value>` |
| `px-<n>` | `padding-left: <value>; padding-right: <value>` |
| `py-<n>` | `padding-top: <value>; padding-bottom: <value>` |
| `ps-<n>` | `padding-inline-start: <value>` |
| `pe-<n>` | `padding-inline-end: <value>` |
| `pt-<n>` | `padding-top: <value>` |
| `pr-<n>` | `padding-right: <value>` |
| `pb-<n>` | `padding-bottom: <value>` |
| `pl-<n>` | `padding-left: <value>` |

**Key Point**: `px` and `py` use TWO physical property declarations.

Source evidence (corePlugins.js:2033-2043):

```javascript
padding: createUtilityPlugin('padding', [
  ['p', ['padding']],
  [
    ['px', ['padding-left', 'padding-right']],
    ['py', ['padding-top', 'padding-bottom']],
  ],
  [
    ['ps', ['padding-inline-start']],
    ['pe', ['padding-inline-end']],
    // ...
  ],
])
```

### Margin

| Utility | CSS Properties |
|---------|----------------|
| `m-<n>` | `margin: <value>` |
| `mx-<n>` | `margin-left: <value>; margin-right: <value>` |
| `my-<n>` | `margin-top: <value>; margin-bottom: <value>` |
| `ms-<n>` | `margin-inline-start: <value>` |
| `me-<n>` | `margin-inline-end: <value>` |
| `mt-<n>` | `margin-top: <value>` |
| `mr-<n>` | `margin-right: <value>` |
| `mb-<n>` | `margin-bottom: <value>` |
| `ml-<n>` | `margin-left: <value>` |
| `m-auto` | `margin: auto` |
| `-m-<n>` | `margin: -<value>` |

**Key Point**: `mx` and `my` use TWO physical property declarations.

### Gap

| Utility | CSS Property |
|---------|--------------|
| `gap-<n>` | `gap: <value>` |
| `gap-x-<n>` | `column-gap: <value>` |
| `gap-y-<n>` | `row-gap: <value>` |

## Transform Utilities

### The Transform Chain

All transform utilities in v3 share a single `transform` property with CSS variables:

```javascript
let cssTransformValue = [
  'translate(var(--tw-translate-x), var(--tw-translate-y))',
  'rotate(var(--tw-rotate))',
  'skewX(var(--tw-skew-x))',
  'skewY(var(--tw-skew-y))',
  'scaleX(var(--tw-scale-x))',
  'scaleY(var(--tw-scale-y))',
].join(' ')
```

### Scale

| Utility | CSS Output |
|---------|------------|
| `scale-50` | `--tw-scale-x: .5; --tw-scale-y: .5; transform: translate(...) rotate(...) skewX(...) skewY(...) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-50` | `--tw-scale-x: .5; transform: ...` |
| `scale-y-50` | `--tw-scale-y: .5; transform: ...` |
| `-scale-50` | `--tw-scale-x: -.5; --tw-scale-y: -.5; transform: ...` |

### Rotate

| Utility | CSS Output |
|---------|------------|
| `rotate-45` | `--tw-rotate: 45deg; transform: translate(...) rotate(var(--tw-rotate)) ...` |
| `-rotate-45` | `--tw-rotate: -45deg; transform: ...` |

### Translate

| Utility | CSS Output |
|---------|------------|
| `translate-x-4` | `--tw-translate-x: 1rem; transform: translate(var(--tw-translate-x), var(--tw-translate-y)) ...` |
| `translate-y-4` | `--tw-translate-y: 1rem; transform: ...` |
| `-translate-x-4` | `--tw-translate-x: -1rem; transform: ...` |

## Filter Utilities

### Filter Chain

All filter utilities share a single `filter` property:

```javascript
let cssFilterValue = [
  'var(--tw-blur)',
  'var(--tw-brightness)',
  'var(--tw-contrast)',
  'var(--tw-grayscale)',
  'var(--tw-hue-rotate)',
  'var(--tw-invert)',
  'var(--tw-saturate)',
  'var(--tw-sepia)',
  'var(--tw-drop-shadow)',
].join(' ')
```

### Blur

| Utility | CSS Output |
|---------|------------|
| `blur-sm` | `--tw-blur: blur(4px); filter: var(--tw-blur) var(--tw-brightness) ...` |
| `blur` | `--tw-blur: blur(8px); filter: ...` |
| `blur-md` | `--tw-blur: blur(12px); filter: ...` |
| `blur-lg` | `--tw-blur: blur(16px); filter: ...` |
| `blur-none` | `--tw-blur: ; filter: ...` |

### Drop Shadow

| Utility | CSS Output |
|---------|------------|
| `drop-shadow-sm` | `--tw-drop-shadow: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05)); filter: ...` |
| `drop-shadow` | `--tw-drop-shadow: drop-shadow(...); filter: ...` |
| `drop-shadow-none` | `--tw-drop-shadow: drop-shadow(0 0 #0000); filter: ...` |

## Typography Utilities

### Font Size

In v3, font-size utilities set both `font-size` and `line-height`:

| Utility | CSS Output |
|---------|------------|
| `text-xs` | `font-size: 0.75rem; line-height: 1rem` |
| `text-sm` | `font-size: 0.875rem; line-height: 1.25rem` |
| `text-base` | `font-size: 1rem; line-height: 1.5rem` |
| `text-lg` | `font-size: 1.125rem; line-height: 1.75rem` |
| `text-xl` | `font-size: 1.25rem; line-height: 1.75rem` |
| `text-2xl` | `font-size: 1.5rem; line-height: 2rem` |

### Letter Spacing

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `tracking-tighter` | `letter-spacing` | -0.05em |
| `tracking-tight` | `letter-spacing` | -0.025em |
| `tracking-normal` | `letter-spacing` | 0em |
| `tracking-wide` | `letter-spacing` | 0.025em |
| `tracking-wider` | `letter-spacing` | 0.05em |
| `tracking-widest` | `letter-spacing` | 0.1em |

## Key Patterns

### Static Values

All values in v3 are static - no CSS variable calculations:

```javascript
// p-4 generates:
{ padding: '1rem' }

// NOT:
{ padding: 'calc(var(--spacing) * 4)' }
```

### Physical Properties for px/py/mx/my

v3 generates two property declarations:

```javascript
// px-4 generates:
{
  'padding-left': '1rem',
  'padding-right': '1rem'
}

// NOT:
{ 'padding-inline': '1rem' }
```

### Transform Chain

All transform utilities output the full chain:

```css
.scale-50 {
  --tw-scale-x: .5;
  --tw-scale-y: .5;
  transform: translate(var(--tw-translate-x), var(--tw-translate-y))
             rotate(var(--tw-rotate))
             skewX(var(--tw-skew-x))
             skewY(var(--tw-skew-y))
             scaleX(var(--tw-scale-x))
             scaleY(var(--tw-scale-y));
}
```

This is fundamentally different from v4's native `scale: 50%` approach.

## Differences from v4

| Aspect | v3 | v4 |
|--------|----|----|
| `px-4` | `padding-left: 1rem; padding-right: 1rem` | `padding-inline: calc(var(--spacing) * 4)` |
| `scale-50` | `transform: ... scaleX(.5) scaleY(.5)` | `scale: 50%` |
| `rotate-45` | `transform: ... rotate(45deg) ...` | `rotate: 45deg` |
| `blur-sm` value | 4px | 8px |
| Value format | Static | CSS variable calculation |
