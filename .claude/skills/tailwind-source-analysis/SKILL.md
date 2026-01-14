---
name: tailwind-source-analysis
description: Extracting accurate Tailwind CSS documentation from source code, not from proprietary MDX docs.
---

# Tailwind CSS Source Code Analysis

When generating or validating Tailwind CSS documentation,
use the source code as the single source of truth.
Do NOT rely on the tailwindcss.com MDX files, which are proprietary.

## Version-Specific Source Locations

### Tailwind CSS v4

```
packages/tailwindcss/
├── theme.css              # DEFAULT VALUES (CSS variables in @theme default)
├── src/
│   ├── utilities.ts       # UTILITY MAPPINGS (staticUtility, functionalUtility)
│   └── utilities.test.ts  # Test cases showing expected outputs
```

### Tailwind CSS v3

```
├── stubs/config.full.js   # DEFAULT VALUES (JavaScript config object)
├── src/
│   └── corePlugins.js     # UTILITY MAPPINGS (createUtilityPlugin)
```

## Critical v3 vs v4 Differences

| Aspect | v3 | v4 |
|--------|----|----|
| Theme format | JS config object | CSS `@theme default` block |
| Value syntax | Static (`'1rem'`) | CSS variables (`var(--spacing)`) |
| `px-*` maps to | `padding-left` + `padding-right` | `padding-inline` |
| `py-*` maps to | `padding-top` + `padding-bottom` | `padding-block` |
| `mx-*` maps to | `margin-left` + `margin-right` | `margin-inline` |
| `my-*` maps to | `margin-top` + `margin-bottom` | `margin-block` |
| `scale-*` | `transform: ... scaleX() scaleY() ...` | `scale: <value>` |
| `rotate-*` | `transform: ... rotate() ...` | `rotate: <value>` |
| `translate-*` | `transform: translate() ...` | `translate: <value>` |
| `blur-sm` | 4px | 8px (renamed from DEFAULT) |

## v3 Source Analysis

### Theme Values (stubs/config.full.js)

Values are static JavaScript objects:

```javascript
spacing: {
  px: '1px',
  0: '0px',
  1: '0.25rem',
  2: '0.5rem',
  4: '1rem',
  // ...
}
```

So `p-4` → `padding: 1rem` (static value).

### Utility Mappings (src/corePlugins.js)

Uses `createUtilityPlugin` function:

```javascript
padding: createUtilityPlugin('padding', [
  ['p', ['padding']],
  [
    ['px', ['padding-left', 'padding-right']],  // Two physical properties!
    ['py', ['padding-top', 'padding-bottom']],
  ],
  // ...
])
```

### Transform Chain (v3)

All transform utilities share one `transform` property:

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

So `rotate-45` → `--tw-rotate: 45deg; transform: translate(...) rotate(var(--tw-rotate)) ...;`

### Blur Values (v3)

```javascript
blur: {
  0: '0',
  none: '',
  sm: '4px',      // Note: 4px in v3
  DEFAULT: '8px', // Bare "blur" utility
  md: '12px',
  lg: '16px',
  // ...
}
```

## v4 Source Analysis

### Theme Values (theme.css)

Values use CSS custom properties:

```css
@theme default {
  --spacing: 0.25rem;
  --blur-xs: 4px;
  --blur-sm: 8px;  /* Note: 8px in v4, was DEFAULT in v3 */
  /* ... */
}
```

### Utility Mappings (src/utilities.ts)

Uses `spacingUtility`, `functionalUtility`, `staticUtility`:

```typescript
for (let [name, property] of [
  ['p', 'padding'],
  ['px', 'padding-inline'],   // Single logical property!
  ['py', 'padding-block'],    // Single logical property!
  // ...
])
```

### Native CSS Transform Properties (v4)

v4 uses native CSS properties, NOT `transform:`:

```typescript
// scale generates:
return [decl('scale', value)]  // scale: 50%

// rotate generates:
return [decl('rotate', value)]  // rotate: 45deg

// translate generates:
decl('translate', '...')  // translate: x y
```

### Spacing Calculation (v4)

```typescript
handleBareValue: ({ value }) => {
  let multiplier = theme.resolve(null, ['--spacing'])
  return `calc(${multiplier} * ${value})`
}
```

So `p-4` → `padding: calc(var(--spacing) * 4)` → `padding: calc(0.25rem * 4)`.

## Validation Workflow

### For v3 Documentation

1. Check `stubs/config.full.js` for theme values
2. Check `src/corePlugins.js` for utility-to-property mappings
3. Note: `px/py/mx/my` use physical properties (two declarations)
4. Note: Transforms use `transform:` property

### For v4 Documentation

1. Check `packages/tailwindcss/theme.css` for CSS variable values
2. Check `packages/tailwindcss/src/utilities.ts` for utility mappings
3. Note: `px/py/mx/my` use logical properties (single declaration)
4. Note: Transforms use native `scale/rotate/translate` properties

## Common Inaccuracies by Version

### v3 Errors

| Error | Wrong | Correct |
|-------|-------|---------|
| Using logical properties | `padding-inline` | `padding-left` + `padding-right` |
| Using native transforms | `scale: 50%` | `transform: ... scaleX(.5) scaleY(.5)` |
| Wrong blur-sm value | 8px | 4px |

### v4 Errors

| Error | Wrong | Correct |
|-------|-------|---------|
| Using physical properties | `padding-left/right` | `padding-inline` |
| Using transform function | `transform: scale(.5)` | `scale: 50%` |
| Wrong blur-sm value | 4px | 8px |
| Static values | `padding: 1rem` | `padding: calc(var(--spacing) * 4)` |

## Reference Documents

See `reference/` directory for extracted values:
- `theme-v3-values.md` — v3 default theme values
- `theme-v4-values.md` — v4 CSS variable values
- `utility-mappings-v3.md` — v3 utility-to-CSS mappings
- `utility-mappings-v4.md` — v4 utility-to-CSS mappings
