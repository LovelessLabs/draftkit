# Effects Utilities (Tailwind CSS v4)

## Box Shadow

```
shadow-sm   → box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)
shadow      → box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)
shadow-md   → box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)
shadow-lg   → box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)
shadow-xl   → box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)
shadow-2xl  → box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25)
shadow-inner → box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05)
shadow-none → box-shadow: 0 0 #0000
```

## Shadow Color

```
shadow-inherit     → --tw-shadow-color: inherit
shadow-current     → --tw-shadow-color: currentColor
shadow-transparent → --tw-shadow-color: transparent
shadow-black       → --tw-shadow-color: #000
shadow-white       → --tw-shadow-color: #fff
shadow-gray-500    → --tw-shadow-color: rgb(107 114 128)
shadow-blue-500    → --tw-shadow-color: rgb(59 130 246)
shadow-red-500/50  → --tw-shadow-color: rgb(239 68 68 / 0.5)
```

## Opacity

```
opacity-0   → opacity: 0
opacity-5   → opacity: 0.05
opacity-10  → opacity: 0.1
opacity-15  → opacity: 0.15
opacity-20  → opacity: 0.2
opacity-25  → opacity: 0.25
opacity-30  → opacity: 0.3
opacity-35  → opacity: 0.35
opacity-40  → opacity: 0.4
opacity-45  → opacity: 0.45
opacity-50  → opacity: 0.5
opacity-55  → opacity: 0.55
opacity-60  → opacity: 0.6
opacity-65  → opacity: 0.65
opacity-70  → opacity: 0.7
opacity-75  → opacity: 0.75
opacity-80  → opacity: 0.8
opacity-85  → opacity: 0.85
opacity-90  → opacity: 0.9
opacity-95  → opacity: 0.95
opacity-100 → opacity: 1
```

## Mix Blend Mode

```
mix-blend-normal      → mix-blend-mode: normal
mix-blend-multiply    → mix-blend-mode: multiply
mix-blend-screen      → mix-blend-mode: screen
mix-blend-overlay     → mix-blend-mode: overlay
mix-blend-darken      → mix-blend-mode: darken
mix-blend-lighten     → mix-blend-mode: lighten
mix-blend-color-dodge → mix-blend-mode: color-dodge
mix-blend-color-burn  → mix-blend-mode: color-burn
mix-blend-hard-light  → mix-blend-mode: hard-light
mix-blend-soft-light  → mix-blend-mode: soft-light
mix-blend-difference  → mix-blend-mode: difference
mix-blend-exclusion   → mix-blend-mode: exclusion
mix-blend-hue         → mix-blend-mode: hue
mix-blend-saturation  → mix-blend-mode: saturation
mix-blend-color       → mix-blend-mode: color
mix-blend-luminosity  → mix-blend-mode: luminosity
mix-blend-plus-darker → mix-blend-mode: plus-darker (v4)
mix-blend-plus-lighter → mix-blend-mode: plus-lighter (v4)
```

## Background Blend Mode

```
bg-blend-normal      → background-blend-mode: normal
bg-blend-multiply    → background-blend-mode: multiply
bg-blend-screen      → background-blend-mode: screen
bg-blend-overlay     → background-blend-mode: overlay
bg-blend-darken      → background-blend-mode: darken
bg-blend-lighten     → background-blend-mode: lighten
bg-blend-color-dodge → background-blend-mode: color-dodge
bg-blend-color-burn  → background-blend-mode: color-burn
bg-blend-hard-light  → background-blend-mode: hard-light
bg-blend-soft-light  → background-blend-mode: soft-light
bg-blend-difference  → background-blend-mode: difference
bg-blend-exclusion   → background-blend-mode: exclusion
bg-blend-hue         → background-blend-mode: hue
bg-blend-saturation  → background-blend-mode: saturation
bg-blend-color       → background-blend-mode: color
bg-blend-luminosity  → background-blend-mode: luminosity
```

## Drop Shadow (Filter)

For elements with transparency (images, SVGs):
```
drop-shadow-sm   → filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))
drop-shadow      → filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))
drop-shadow-md   → filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))
drop-shadow-lg   → filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))
drop-shadow-xl   → filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))
drop-shadow-2xl  → filter: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))
drop-shadow-none → filter: drop-shadow(0 0 #0000)
```

## Arbitrary Values

```
shadow-[0_35px_60px_-15px_rgba(0,0,0,0.3)] → custom box-shadow
opacity-[0.67] → opacity: 0.67
drop-shadow-[0_35px_35px_rgba(0,0,0,0.25)] → custom drop-shadow
```

## Common Patterns

### Elevated Card
```html
<div class="rounded-lg bg-white p-6 shadow-lg">
  <!-- Card floats above the page -->
</div>
```

### Subtle Card Shadow
```html
<div class="rounded-lg bg-white p-6 shadow-sm hover:shadow-md transition-shadow">
  <!-- Subtle shadow that grows on hover -->
</div>
```

### Colored Shadow
```html
<button class="bg-blue-500 text-white shadow-lg shadow-blue-500/50">
  <!-- Blue glow effect -->
</button>
```

### Inset Shadow (Pressed State)
```html
<button class="shadow-inner bg-gray-100">
  <!-- Pressed/recessed appearance -->
</button>
```

### Image with Drop Shadow
```html
<img class="drop-shadow-xl" src="..." />
<!-- Drop shadow respects image transparency -->
```

### Faded Disabled State
```html
<button class="opacity-50 cursor-not-allowed" disabled>
  Disabled
</button>
```

### Overlay
```html
<div class="fixed inset-0 bg-black opacity-50">
  <!-- Semi-transparent overlay -->
</div>
<!-- Or use bg-black/50 which is simpler -->
```

### Glass Effect (v4)
```html
<div class="bg-white/30 backdrop-blur-md shadow-lg">
  <!-- Frosted glass appearance -->
</div>
```

### Multiply Blend
```html
<div class="relative">
  <img src="..." />
  <div class="absolute inset-0 bg-blue-500 mix-blend-multiply">
    <!-- Color overlay effect -->
  </div>
</div>
```
