# SVG Utilities (Tailwind CSS v4)

## Fill

Control SVG fill color:
```
fill-none        → fill: none
fill-inherit     → fill: inherit
fill-current     → fill: currentColor
fill-transparent → fill: transparent
fill-black       → fill: #000
fill-white       → fill: #fff
fill-gray-500    → fill: rgb(107 114 128)
fill-blue-500    → fill: rgb(59 130 246)
```

## Stroke

### Stroke Color
```
stroke-none        → stroke: none
stroke-inherit     → stroke: inherit
stroke-current     → stroke: currentColor
stroke-transparent → stroke: transparent
stroke-black       → stroke: #000
stroke-white       → stroke: #fff
stroke-gray-500    → stroke: rgb(107 114 128)
stroke-blue-500    → stroke: rgb(59 130 246)
```

### Stroke Width
```
stroke-0 → stroke-width: 0
stroke-1 → stroke-width: 1
stroke-2 → stroke-width: 2
```

## Common Patterns

### Icon with currentColor
```html
<button class="text-gray-500 hover:text-gray-700">
  <svg class="h-5 w-5 fill-current">
    <!-- Icon inherits text color -->
  </svg>
</button>
```

### Outlined Icon
```html
<svg class="h-6 w-6 stroke-current stroke-2 fill-none">
  <!-- Outlined icon style -->
</svg>
```

### Icon Button with States
```html
<button class="group text-gray-400 hover:text-blue-500">
  <svg class="h-5 w-5 fill-current transition-colors">
    <!-- Changes color on parent hover -->
  </svg>
</button>
```

### Two-Tone Icon
```html
<svg class="h-6 w-6">
  <path class="fill-blue-200" d="..."/>
  <path class="fill-blue-500" d="..."/>
</svg>
```

### Icon with Background
```html
<div class="flex h-10 w-10 items-center justify-center rounded-full bg-blue-100">
  <svg class="h-5 w-5 fill-blue-500">...</svg>
</div>
```

### Animated Icon
```html
<svg class="h-5 w-5 animate-spin stroke-current fill-none">
  <!-- Loading spinner -->
</svg>
```

### Icon in Dark Mode
```html
<svg class="h-5 w-5 fill-gray-600 dark:fill-gray-300">
  <!-- Adapts to dark mode -->
</svg>
```

### Disabled Icon
```html
<svg class="h-5 w-5 fill-gray-300">
  <!-- Muted/disabled appearance -->
</svg>
```

### Icon Sizes
```html
<!-- Extra small -->
<svg class="h-4 w-4">...</svg>

<!-- Small -->
<svg class="h-5 w-5">...</svg>

<!-- Medium (default) -->
<svg class="h-6 w-6">...</svg>

<!-- Large -->
<svg class="h-8 w-8">...</svg>

<!-- Extra large -->
<svg class="h-10 w-10">...</svg>
```

### Inline Icon with Text
```html
<span class="inline-flex items-center gap-1">
  <svg class="h-4 w-4 fill-current">...</svg>
  Label text
</span>
```

### Social Icons
```html
<a href="#" class="text-gray-400 hover:text-[#1DA1F2]">
  <svg class="h-6 w-6 fill-current">
    <!-- Twitter icon -->
  </svg>
</a>
```

### Status Indicator
```html
<span class="inline-flex items-center gap-1 text-green-600">
  <svg class="h-4 w-4 fill-current">
    <!-- Checkmark icon -->
  </svg>
  Success
</span>
```

### Logo with Hover
```html
<a href="/" class="block text-gray-900 hover:text-blue-600 transition-colors">
  <svg class="h-8 w-auto fill-current">
    <!-- Logo SVG -->
  </svg>
</a>
```

## Size Utility (for square icons)

```html
<!-- Using size utility -->
<svg class="size-5 fill-current">...</svg>
<!-- Equivalent to h-5 w-5 -->

<svg class="size-6 fill-current">...</svg>
<!-- Equivalent to h-6 w-6 -->
```

## Arbitrary Values

```html
<svg class="fill-[#ff5500]">...</svg>
<svg class="stroke-[2.5]">...</svg>
<svg class="stroke-[#custom]">...</svg>
```

## Tips

1. **Use currentColor** - `fill-current` and `stroke-current` inherit from text color
2. **Set viewBox** - Always include viewBox on SVG for proper scaling
3. **Remove inline styles** - Delete fill/stroke attributes from SVG markup to use Tailwind classes
4. **Use size utility** - `size-5` is cleaner than `h-5 w-5` for square icons
5. **Consider accessibility** - Add `aria-hidden="true"` for decorative icons, `aria-label` for meaningful ones
