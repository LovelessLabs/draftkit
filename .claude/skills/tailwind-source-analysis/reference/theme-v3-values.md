# Tailwind CSS v3 Theme Values

Extracted from `tailwindcss-3.4.19/stubs/config.full.js`.

## Spacing

Static rem values (no CSS variables):

| Key | Value |
|-----|-------|
| `px` | 1px |
| `0` | 0px |
| `0.5` | 0.125rem |
| `1` | 0.25rem |
| `1.5` | 0.375rem |
| `2` | 0.5rem |
| `2.5` | 0.625rem |
| `3` | 0.75rem |
| `3.5` | 0.875rem |
| `4` | 1rem |
| `5` | 1.25rem |
| `6` | 1.5rem |
| `7` | 1.75rem |
| `8` | 2rem |
| `9` | 2.25rem |
| `10` | 2.5rem |
| `11` | 2.75rem |
| `12` | 3rem |
| `14` | 3.5rem |
| `16` | 4rem |
| `20` | 5rem |
| `24` | 6rem |
| `28` | 7rem |
| `32` | 8rem |
| `36` | 9rem |
| `40` | 10rem |
| `44` | 11rem |
| `48` | 12rem |
| `52` | 13rem |
| `56` | 14rem |
| `60` | 15rem |
| `64` | 16rem |
| `72` | 18rem |
| `80` | 20rem |
| `96` | 24rem |

## Typography

### Font Sizes (with line-height)

| Key | Font Size | Line Height |
|-----|-----------|-------------|
| `xs` | 0.75rem | 1rem |
| `sm` | 0.875rem | 1.25rem |
| `base` | 1rem | 1.5rem |
| `lg` | 1.125rem | 1.75rem |
| `xl` | 1.25rem | 1.75rem |
| `2xl` | 1.5rem | 2rem |
| `3xl` | 1.875rem | 2.25rem |
| `4xl` | 2.25rem | 2.5rem |
| `5xl` | 3rem | 1 |
| `6xl` | 3.75rem | 1 |
| `7xl` | 4.5rem | 1 |
| `8xl` | 6rem | 1 |
| `9xl` | 8rem | 1 |

### Letter Spacing

| Key | Value |
|-----|-------|
| `tighter` | -0.05em |
| `tight` | -0.025em |
| `normal` | 0em |
| `wide` | 0.025em |
| `wider` | 0.05em |
| `widest` | 0.1em |

### Line Height

| Key | Value |
|-----|-------|
| `none` | 1 |
| `tight` | 1.25 |
| `snug` | 1.375 |
| `normal` | 1.5 |
| `relaxed` | 1.625 |
| `loose` | 2 |
| `3` | 0.75rem |
| `4` | 1rem |
| `5` | 1.25rem |
| `6` | 1.5rem |
| `7` | 1.75rem |
| `8` | 2rem |
| `9` | 2.25rem |
| `10` | 2.5rem |

### Font Weights

| Key | Value |
|-----|-------|
| `thin` | 100 |
| `extralight` | 200 |
| `light` | 300 |
| `normal` | 400 |
| `medium` | 500 |
| `semibold` | 600 |
| `bold` | 700 |
| `extrabold` | 800 |
| `black` | 900 |

## Border Radius

| Key | Value |
|-----|-------|
| `none` | 0px |
| `sm` | 0.125rem |
| `DEFAULT` | 0.25rem |
| `md` | 0.375rem |
| `lg` | 0.5rem |
| `xl` | 0.75rem |
| `2xl` | 1rem |
| `3xl` | 1.5rem |
| `full` | 9999px |

## Blur Values

| Key | Value |
|-----|-------|
| `0` | 0 |
| `none` | (empty) |
| `sm` | **4px** |
| `DEFAULT` | 8px |
| `md` | 12px |
| `lg` | 16px |
| `xl` | 24px |
| `2xl` | 40px |
| `3xl` | 64px |

**Important**: `blur-sm` is 4px in v3, but 8px in v4.
The bare `blur` utility uses `DEFAULT` (8px).

## Shadows

### Box Shadows

| Key | Value |
|-----|-------|
| `sm` | `0 1px 2px 0 rgb(0 0 0 / 0.05)` |
| `DEFAULT` | `0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)` |
| `md` | `0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)` |
| `lg` | `0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)` |
| `xl` | `0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)` |
| `2xl` | `0 25px 50px -12px rgb(0 0 0 / 0.25)` |
| `inner` | `inset 0 2px 4px 0 rgb(0 0 0 / 0.05)` |
| `none` | none |

### Drop Shadows

| Key | Value |
|-----|-------|
| `sm` | `0 1px 1px rgb(0 0 0 / 0.05)` |
| `DEFAULT` | `0 1px 2px rgb(0 0 0 / 0.1), 0 1px 1px rgb(0 0 0 / 0.06)` |
| `md` | `0 4px 3px rgb(0 0 0 / 0.07), 0 2px 2px rgb(0 0 0 / 0.06)` |
| `lg` | `0 10px 8px rgb(0 0 0 / 0.04), 0 4px 3px rgb(0 0 0 / 0.1)` |
| `xl` | `0 20px 13px rgb(0 0 0 / 0.03), 0 8px 5px rgb(0 0 0 / 0.08)` |
| `2xl` | `0 25px 25px rgb(0 0 0 / 0.15)` |
| `none` | `0 0 #0000` |

## Scale

| Key | Value |
|-----|-------|
| `0` | 0 |
| `50` | .5 |
| `75` | .75 |
| `90` | .9 |
| `95` | .95 |
| `100` | 1 |
| `105` | 1.05 |
| `110` | 1.1 |
| `125` | 1.25 |
| `150` | 1.5 |

## Rotate

| Key | Value |
|-----|-------|
| `0` | 0deg |
| `1` | 1deg |
| `2` | 2deg |
| `3` | 3deg |
| `6` | 6deg |
| `12` | 12deg |
| `45` | 45deg |
| `90` | 90deg |
| `180` | 180deg |

## Breakpoints (screens)

| Key | Value |
|-----|-------|
| `sm` | 640px |
| `md` | 768px |
| `lg` | 1024px |
| `xl` | 1280px |
| `2xl` | 1536px |

## Key Differences from v4

1. **No CSS variables** - All values are static
2. **`blur-sm` is 4px** (v4 is 8px)
3. **`DEFAULT` key** for bare utilities (v4 uses named sizes only)
4. **Font sizes use fixed line-heights** (v4 uses ratio calculations)
