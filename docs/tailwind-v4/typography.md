# Typography Utilities (Tailwind CSS v4)

## Architecture Note

Tailwind CSS v4 uses **CSS custom properties** for typography values:

```css
--text-xs: 0.75rem;
--text-xs--line-height: calc(1 / 0.75);
--text-sm: 0.875rem;
--text-sm--line-height: calc(1.25 / 0.875);
/* ... */
```

Each font size has an associated default line-height variable.

## Font Family

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `font-sans` | `font-family` | `var(--font-sans)` (ui-sans-serif, system-ui, sans-serif, ...) |
| `font-serif` | `font-family` | `var(--font-serif)` (ui-serif, Georgia, Cambria, ...) |
| `font-mono` | `font-family` | `var(--font-mono)` (ui-monospace, SFMono-Regular, Menlo, ...) |

## Font Size

Font sizes set both `font-size` and `line-height` via CSS variables:

| Utility | CSS Output | Computed Values |
|---------|------------|-----------------|
| `text-xs` | `font-size: var(--text-xs); line-height: var(--text-xs--line-height)` | 0.75rem / 1.33 |
| `text-sm` | `font-size: var(--text-sm); line-height: var(--text-sm--line-height)` | 0.875rem / 1.43 |
| `text-base` | `font-size: var(--text-base); line-height: var(--text-base--line-height)` | 1rem / 1.5 |
| `text-lg` | `font-size: var(--text-lg); line-height: var(--text-lg--line-height)` | 1.125rem / 1.56 |
| `text-xl` | `font-size: var(--text-xl); line-height: var(--text-xl--line-height)` | 1.25rem / 1.4 |
| `text-2xl` | `font-size: var(--text-2xl); line-height: var(--text-2xl--line-height)` | 1.5rem / 1.33 |
| `text-3xl` | `font-size: var(--text-3xl); line-height: var(--text-3xl--line-height)` | 1.875rem / 1.2 |
| `text-4xl` | `font-size: var(--text-4xl); line-height: var(--text-4xl--line-height)` | 2.25rem / 1.11 |
| `text-5xl` | `font-size: var(--text-5xl); line-height: var(--text-5xl--line-height)` | 3rem / 1 |
| `text-6xl` | `font-size: var(--text-6xl); line-height: var(--text-6xl--line-height)` | 3.75rem / 1 |
| `text-7xl` | `font-size: var(--text-7xl); line-height: var(--text-7xl--line-height)` | 4.5rem / 1 |
| `text-8xl` | `font-size: var(--text-8xl); line-height: var(--text-8xl--line-height)` | 6rem / 1 |
| `text-9xl` | `font-size: var(--text-9xl); line-height: var(--text-9xl--line-height)` | 8rem / 1 |

### Font Size with Line Height Modifier (v4)

Override the default line-height with the `/` modifier:

| Utility | CSS Output |
|---------|------------|
| `text-sm/6` | `font-size: var(--text-sm); line-height: calc(var(--spacing) * 6)` |
| `text-lg/8` | `font-size: var(--text-lg); line-height: calc(var(--spacing) * 8)` |
| `text-xl/tight` | `font-size: var(--text-xl); line-height: var(--leading-tight)` |
| `text-2xl/none` | `font-size: var(--text-2xl); line-height: 1` |

## Font Weight

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `font-thin` | `font-weight` | `100` |
| `font-extralight` | `font-weight` | `200` |
| `font-light` | `font-weight` | `300` |
| `font-normal` | `font-weight` | `400` |
| `font-medium` | `font-weight` | `500` |
| `font-semibold` | `font-weight` | `600` |
| `font-bold` | `font-weight` | `700` |
| `font-extrabold` | `font-weight` | `800` |
| `font-black` | `font-weight` | `900` |

## Font Style

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `italic` | `font-style` | `italic` |
| `not-italic` | `font-style` | `normal` |

## Line Height (Leading)

### Named Values

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `leading-none` | `line-height` | `1` |
| `leading-tight` | `line-height` | `var(--leading-tight)` (1.25) |
| `leading-snug` | `line-height` | `var(--leading-snug)` (1.375) |
| `leading-normal` | `line-height` | `var(--leading-normal)` (1.5) |
| `leading-relaxed` | `line-height` | `var(--leading-relaxed)` (1.625) |
| `leading-loose` | `line-height` | `var(--leading-loose)` (2) |

### Numeric Values (Spacing-Based)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `leading-3` | `line-height` | `calc(var(--spacing) * 3)` (0.75rem) |
| `leading-4` | `line-height` | `calc(var(--spacing) * 4)` (1rem) |
| `leading-5` | `line-height` | `calc(var(--spacing) * 5)` (1.25rem) |
| `leading-6` | `line-height` | `calc(var(--spacing) * 6)` (1.5rem) |
| `leading-7` | `line-height` | `calc(var(--spacing) * 7)` (1.75rem) |
| `leading-8` | `line-height` | `calc(var(--spacing) * 8)` (2rem) |
| `leading-9` | `line-height` | `calc(var(--spacing) * 9)` (2.25rem) |
| `leading-10` | `line-height` | `calc(var(--spacing) * 10)` (2.5rem) |

## Letter Spacing (Tracking)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `tracking-tighter` | `letter-spacing` | `var(--tracking-tighter)` (-0.05em) |
| `tracking-tight` | `letter-spacing` | `var(--tracking-tight)` (-0.025em) |
| `tracking-normal` | `letter-spacing` | `var(--tracking-normal)` (0em) |
| `tracking-wide` | `letter-spacing` | `var(--tracking-wide)` (0.025em) |
| `tracking-wider` | `letter-spacing` | `var(--tracking-wider)` (0.05em) |
| `tracking-widest` | `letter-spacing` | `var(--tracking-widest)` (0.1em) |

## Text Alignment

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `text-left` | `text-align` | `left` |
| `text-center` | `text-align` | `center` |
| `text-right` | `text-align` | `right` |
| `text-justify` | `text-align` | `justify` |
| `text-start` | `text-align` | `start` (logical) |
| `text-end` | `text-align` | `end` (logical) |

## Text Color

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `text-inherit` | `color` | `inherit` |
| `text-current` | `color` | `currentColor` |
| `text-transparent` | `color` | `transparent` |
| `text-black` | `color` | `var(--color-black)` |
| `text-white` | `color` | `var(--color-white)` |
| `text-gray-500` | `color` | `var(--color-gray-500)` |
| `text-red-500` | `color` | `var(--color-red-500)` |
| `text-blue-500` | `color` | `var(--color-blue-500)` |

## Text Decoration

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `underline` | `text-decoration-line` | `underline` |
| `overline` | `text-decoration-line` | `overline` |
| `line-through` | `text-decoration-line` | `line-through` |
| `no-underline` | `text-decoration-line` | `none` |

## Text Decoration Style

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `decoration-solid` | `text-decoration-style` | `solid` |
| `decoration-double` | `text-decoration-style` | `double` |
| `decoration-dotted` | `text-decoration-style` | `dotted` |
| `decoration-dashed` | `text-decoration-style` | `dashed` |
| `decoration-wavy` | `text-decoration-style` | `wavy` |

## Text Decoration Color

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `decoration-inherit` | `text-decoration-color` | `inherit` |
| `decoration-current` | `text-decoration-color` | `currentColor` |
| `decoration-transparent` | `text-decoration-color` | `transparent` |
| `decoration-red-500` | `text-decoration-color` | `var(--color-red-500)` |

## Text Transform

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `uppercase` | `text-transform` | `uppercase` |
| `lowercase` | `text-transform` | `lowercase` |
| `capitalize` | `text-transform` | `capitalize` |
| `normal-case` | `text-transform` | `none` |

## Text Overflow

| Utility | CSS Output |
|---------|------------|
| `truncate` | `overflow: hidden; text-overflow: ellipsis; white-space: nowrap` |
| `text-ellipsis` | `text-overflow: ellipsis` |
| `text-clip` | `text-overflow: clip` |

## Text Wrap (v4)

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `text-wrap` | `text-wrap` | `wrap` |
| `text-nowrap` | `text-wrap` | `nowrap` |
| `text-balance` | `text-wrap` | `balance` (balances line lengths) |
| `text-pretty` | `text-wrap` | `pretty` (avoids orphans) |

## Whitespace

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `whitespace-normal` | `white-space` | `normal` |
| `whitespace-nowrap` | `white-space` | `nowrap` |
| `whitespace-pre` | `white-space` | `pre` |
| `whitespace-pre-line` | `white-space` | `pre-line` |
| `whitespace-pre-wrap` | `white-space` | `pre-wrap` |
| `whitespace-break-spaces` | `white-space` | `break-spaces` |

## Word Break

| Utility | CSS Output |
|---------|------------|
| `break-normal` | `overflow-wrap: normal; word-break: normal` |
| `break-words` | `overflow-wrap: break-word` |
| `break-all` | `word-break: break-all` |
| `break-keep` | `word-break: keep-all` |

## Hyphens

| Utility | CSS Property | CSS Value |
|---------|--------------|-----------|
| `hyphens-none` | `hyphens` | `none` |
| `hyphens-manual` | `hyphens` | `manual` |
| `hyphens-auto` | `hyphens` | `auto` |

## Common Patterns

### Heading with Tight Tracking
```html
<h1 class="text-4xl font-bold tracking-tight text-gray-900">
  Page Title
</h1>
```

### Body Text
```html
<p class="text-base leading-relaxed text-gray-600">
  Paragraph content with comfortable reading line height.
</p>
```

### Custom Line Height Override
```html
<p class="text-lg/8">
  Text with custom line-height (2rem)
</p>
```

### Small Caps Label
```html
<span class="text-xs font-semibold uppercase tracking-wider text-gray-500">
  Category Label
</span>
```

### Truncated Text
```html
<p class="truncate">
  This very long text will be truncated with an ellipsis...
</p>
```

### Multi-line Truncation (with line-clamp)
```html
<p class="line-clamp-3">
  This text will be limited to 3 lines with ellipsis...
</p>
```

### Balanced Headline (v4)
```html
<h2 class="text-3xl font-bold text-balance">
  A headline that wraps evenly across lines
</h2>
```

### Pretty Paragraphs (v4)
```html
<p class="text-pretty">
  Avoids single-word orphans at the end of paragraphs.
</p>
```

## v3 to v4 Migration Notes

| Change | v3 | v4 |
|--------|----|----|
| Font sizes | Static values (`font-size: 0.875rem`) | CSS variables (`font-size: var(--text-sm)`) |
| Line heights | Fixed with size | CSS variable pair (`--text-sm--line-height`) |
| Line height override | Separate `leading-*` utility | Modifier syntax (`text-sm/6`) |
| `text-balance` | N/A | New (CSS `text-wrap: balance`) |
| `text-pretty` | N/A | New (CSS `text-wrap: pretty`) |
