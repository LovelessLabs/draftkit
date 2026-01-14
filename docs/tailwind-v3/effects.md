# Effects Utilities (Tailwind CSS v3)

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
shadow-{color}-{shade} → --tw-shadow-color: {value}
shadow-inherit
shadow-current
shadow-transparent
shadow-black
shadow-white
```

## Opacity
```
opacity-0   → opacity: 0
opacity-5   → opacity: 0.05
opacity-10  → opacity: 0.1
opacity-20  → opacity: 0.2
opacity-25  → opacity: 0.25
opacity-30  → opacity: 0.3
opacity-40  → opacity: 0.4
opacity-50  → opacity: 0.5
opacity-60  → opacity: 0.6
opacity-70  → opacity: 0.7
opacity-75  → opacity: 0.75
opacity-80  → opacity: 0.8
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
mix-blend-plus-darker → mix-blend-mode: plus-darker
mix-blend-plus-lighter → mix-blend-mode: plus-lighter
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

## Common Patterns

### Card with Shadow
```html
<div class="bg-white rounded-lg shadow-lg p-6">
  Elevated card
</div>
```

### Colored Shadow
```html
<button class="bg-blue-500 shadow-lg shadow-blue-500/50">
  Button with colored shadow
</button>
```

### Hover Shadow Effect
```html
<div class="shadow hover:shadow-lg transition-shadow">
  Hover for larger shadow
</div>
```

### Faded Element
```html
<div class="opacity-50">
  50% opacity
</div>
```

### Disabled State
```html
<button disabled class="opacity-50 cursor-not-allowed">
  Disabled
</button>
```

### Overlay Effect
```html
<div class="relative">
  <img src="photo.jpg" alt="">
  <div class="absolute inset-0 bg-black opacity-50"></div>
</div>
```

### Inner Shadow (Inset)
```html
<input class="shadow-inner bg-gray-50">
```

### Layered Shadows
```html
<div class="shadow-sm hover:shadow-md active:shadow-inner transition-shadow">
  Interactive element
</div>
```

### Blend Mode Effect
```html
<div class="relative">
  <img src="photo.jpg" alt="">
  <div class="absolute inset-0 bg-purple-500 mix-blend-multiply"></div>
</div>
```

### Text Over Image
```html
<div class="relative">
  <img src="hero.jpg" alt="" class="brightness-50">
  <h1 class="absolute inset-0 flex items-center justify-center text-white">
    Overlaid Text
  </h1>
</div>
```
