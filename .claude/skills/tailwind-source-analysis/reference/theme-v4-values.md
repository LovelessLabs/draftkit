# Tailwind CSS v4 Theme Values

Extracted from `tailwindcss-4.1.18/packages/tailwindcss/theme.css`.

## Spacing

```css
--spacing: 0.25rem;
```

All spacing utilities use `calc(var(--spacing) * n)`:
- `p-4` → `padding: calc(0.25rem * 4)` = `1rem`
- `m-8` → `margin: calc(0.25rem * 8)` = `2rem`

## Typography

### Font Sizes

| Variable | Size | Line Height |
|----------|------|-------------|
| `--text-xs` | 0.75rem | `calc(1 / 0.75)` ≈ 1.333 |
| `--text-sm` | 0.875rem | `calc(1.25 / 0.875)` ≈ 1.429 |
| `--text-base` | 1rem | `calc(1.5 / 1)` = 1.5 |
| `--text-lg` | 1.125rem | `calc(1.75 / 1.125)` ≈ 1.556 |
| `--text-xl` | 1.25rem | `calc(1.75 / 1.25)` = 1.4 |
| `--text-2xl` | 1.5rem | `calc(2 / 1.5)` ≈ 1.333 |
| `--text-3xl` | 1.875rem | `calc(2.25 / 1.875)` = 1.2 |
| `--text-4xl` | 2.25rem | `calc(2.5 / 2.25)` ≈ 1.111 |
| `--text-5xl` | 3rem | 1 |
| `--text-6xl` | 3.75rem | 1 |
| `--text-7xl` | 4.5rem | 1 |
| `--text-8xl` | 6rem | 1 |
| `--text-9xl` | 8rem | 1 |

### Letter Spacing (Tracking)

| Variable | Value |
|----------|-------|
| `--tracking-tighter` | -0.05em |
| `--tracking-tight` | -0.025em |
| `--tracking-normal` | 0em |
| `--tracking-wide` | 0.025em |
| `--tracking-wider` | 0.05em |
| `--tracking-widest` | 0.1em |

### Line Height (Leading)

| Variable | Value |
|----------|-------|
| `--leading-tight` | 1.25 |
| `--leading-snug` | 1.375 |
| `--leading-normal` | 1.5 |
| `--leading-relaxed` | 1.625 |
| `--leading-loose` | 2 |

### Font Weights

| Variable | Value |
|----------|-------|
| `--font-weight-thin` | 100 |
| `--font-weight-extralight` | 200 |
| `--font-weight-light` | 300 |
| `--font-weight-normal` | 400 |
| `--font-weight-medium` | 500 |
| `--font-weight-semibold` | 600 |
| `--font-weight-bold` | 700 |
| `--font-weight-extrabold` | 800 |
| `--font-weight-black` | 900 |

## Border Radius

| Variable | Value |
|----------|-------|
| `--radius-xs` | 0.125rem |
| `--radius-sm` | 0.25rem |
| `--radius-md` | 0.375rem |
| `--radius-lg` | 0.5rem |
| `--radius-xl` | 0.75rem |
| `--radius-2xl` | 1rem |
| `--radius-3xl` | 1.5rem |
| `--radius-4xl` | 2rem |

## Blur Values

| Variable | Value |
|----------|-------|
| `--blur-xs` | 4px |
| `--blur-sm` | **8px** |
| `--blur-md` | 12px |
| `--blur-lg` | 16px |
| `--blur-xl` | 24px |
| `--blur-2xl` | 40px |
| `--blur-3xl` | 64px |

**Note**: `--blur-sm` is 8px, NOT 4px.
The deprecated `--blur` (bare) was 8px.

## Shadows

### Box Shadows

| Variable | Value |
|----------|-------|
| `--shadow-2xs` | `0 1px rgb(0 0 0 / 0.05)` |
| `--shadow-xs` | `0 1px 2px 0 rgb(0 0 0 / 0.05)` |
| `--shadow-sm` | `0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)` |
| `--shadow-md` | `0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)` |
| `--shadow-lg` | `0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)` |
| `--shadow-xl` | `0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)` |
| `--shadow-2xl` | `0 25px 50px -12px rgb(0 0 0 / 0.25)` |

### Drop Shadows

| Variable | Value |
|----------|-------|
| `--drop-shadow-xs` | `0 1px 1px rgb(0 0 0 / 0.05)` |
| `--drop-shadow-sm` | `0 1px 2px rgb(0 0 0 / 0.15)` |
| `--drop-shadow-md` | `0 3px 3px rgb(0 0 0 / 0.12)` |
| `--drop-shadow-lg` | `0 4px 4px rgb(0 0 0 / 0.15)` |
| `--drop-shadow-xl` | `0 9px 7px rgb(0 0 0 / 0.1)` |
| `--drop-shadow-2xl` | `0 25px 25px rgb(0 0 0 / 0.15)` |

## Breakpoints

| Variable | Value |
|----------|-------|
| `--breakpoint-sm` | 40rem (640px) |
| `--breakpoint-md` | 48rem (768px) |
| `--breakpoint-lg` | 64rem (1024px) |
| `--breakpoint-xl` | 80rem (1280px) |
| `--breakpoint-2xl` | 96rem (1536px) |

## Container Sizes

| Variable | Value |
|----------|-------|
| `--container-3xs` | 16rem |
| `--container-2xs` | 18rem |
| `--container-xs` | 20rem |
| `--container-sm` | 24rem |
| `--container-md` | 28rem |
| `--container-lg` | 32rem |
| `--container-xl` | 36rem |
| `--container-2xl` | 42rem |
| `--container-3xl` | 48rem |
| `--container-4xl` | 56rem |
| `--container-5xl` | 64rem |
| `--container-6xl` | 72rem |
| `--container-7xl` | 80rem |

## Easing Functions

| Variable | Value |
|----------|-------|
| `--ease-in` | `cubic-bezier(0.4, 0, 1, 1)` |
| `--ease-out` | `cubic-bezier(0, 0, 0.2, 1)` |
| `--ease-in-out` | `cubic-bezier(0.4, 0, 0.2, 1)` |

## Animations

| Variable | Value |
|----------|-------|
| `--animate-spin` | `spin 1s linear infinite` |
| `--animate-ping` | `ping 1s cubic-bezier(0, 0, 0.2, 1) infinite` |
| `--animate-pulse` | `pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite` |
| `--animate-bounce` | `bounce 1s infinite` |

## Perspective

| Variable | Value |
|----------|-------|
| `--perspective-dramatic` | 100px |
| `--perspective-near` | 300px |
| `--perspective-normal` | 500px |
| `--perspective-midrange` | 800px |
| `--perspective-distant` | 1200px |

## Defaults

| Variable | Value |
|----------|-------|
| `--default-transition-duration` | 150ms |
| `--default-transition-timing-function` | `cubic-bezier(0.4, 0, 0.2, 1)` |
| `--aspect-video` | 16 / 9 |
