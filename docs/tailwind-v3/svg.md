# SVG Utilities (Tailwind CSS v3)

## Fill
```
fill-none        → fill: none
fill-inherit     → fill: inherit
fill-current     → fill: currentColor
fill-transparent → fill: transparent
fill-black       → fill: #000
fill-white       → fill: #fff
fill-{color}-{shade} → fill: {value}
```

## Stroke
```
stroke-none        → stroke: none
stroke-inherit     → stroke: inherit
stroke-current     → stroke: currentColor
stroke-transparent → stroke: transparent
stroke-black       → stroke: #000
stroke-white       → stroke: #fff
stroke-{color}-{shade} → stroke: {value}
```

## Stroke Width
```
stroke-0 → stroke-width: 0
stroke-1 → stroke-width: 1
stroke-2 → stroke-width: 2
```

## Common Patterns

### Icon with Current Color
```html
<button class="text-blue-500 hover:text-blue-600">
  <svg class="w-5 h-5 fill-current" viewBox="0 0 20 20">
    <path d="..."/>
  </svg>
</button>
```

### Outline Icon
```html
<svg class="w-6 h-6 stroke-current stroke-2 fill-none" viewBox="0 0 24 24">
  <path stroke-linecap="round" stroke-linejoin="round" d="..."/>
</svg>
```

### Icon Button
```html
<button class="p-2 text-gray-500 hover:text-gray-700">
  <svg class="w-5 h-5 fill-current" viewBox="0 0 20 20">
    <path d="..."/>
  </svg>
</button>
```

### Two-Tone Icon
```html
<svg class="w-6 h-6" viewBox="0 0 24 24">
  <path class="fill-blue-200" d="..."/>  <!-- Background layer -->
  <path class="fill-blue-600" d="..."/>  <!-- Foreground layer -->
</svg>
```

### Icon with Hover Color Change
```html
<a href="#" class="group">
  <svg class="w-5 h-5 fill-gray-400 group-hover:fill-blue-500 transition-colors">
    <path d="..."/>
  </svg>
</a>
```

### Sizing Icons
```html
<!-- Common sizes -->
<svg class="w-4 h-4">...</svg>   <!-- 16px -->
<svg class="w-5 h-5">...</svg>   <!-- 20px -->
<svg class="w-6 h-6">...</svg>   <!-- 24px -->
<svg class="w-8 h-8">...</svg>   <!-- 32px -->
<svg class="w-10 h-10">...</svg> <!-- 40px -->
<svg class="w-12 h-12">...</svg> <!-- 48px -->
```

### Icon Next to Text
```html
<button class="inline-flex items-center gap-2">
  <svg class="w-5 h-5 fill-current" viewBox="0 0 20 20">
    <path d="..."/>
  </svg>
  <span>Button text</span>
</button>
```

### Icon with Negative Margin (Optical Alignment)
```html
<button class="inline-flex items-center">
  <svg class="-ml-1 mr-2 w-5 h-5 fill-current">
    <path d="..."/>
  </svg>
  Button
</button>
```

### Loading Spinner
```html
<svg class="animate-spin w-5 h-5 text-white" viewBox="0 0 24 24">
  <circle class="opacity-25" cx="12" cy="12" r="10"
          stroke="currentColor" stroke-width="4" fill="none"/>
  <path class="opacity-75" fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
</svg>
```

### Checkmark Icon
```html
<svg class="w-5 h-5 text-green-500 fill-current" viewBox="0 0 20 20">
  <path fill-rule="evenodd"
        d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
        clip-rule="evenodd"/>
</svg>
```

### Arrow Icon
```html
<svg class="w-4 h-4 stroke-current stroke-2" viewBox="0 0 24 24" fill="none">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
</svg>
```

### Dark Mode Icon
```html
<svg class="w-6 h-6 fill-gray-700 dark:fill-gray-300">
  <path d="..."/>
</svg>
```

### Status Indicator
```html
<span class="relative inline-flex">
  <svg class="w-5 h-5 text-gray-400 fill-current">...</svg>
  <span class="absolute top-0 right-0 w-2 h-2 bg-green-400 rounded-full"></span>
</span>
```

### Social Icons Row
```html
<div class="flex space-x-4">
  <a href="#" class="text-gray-400 hover:text-gray-600">
    <svg class="w-6 h-6 fill-current" viewBox="0 0 24 24">
      <!-- Twitter icon -->
    </svg>
  </a>
  <a href="#" class="text-gray-400 hover:text-gray-600">
    <svg class="w-6 h-6 fill-current" viewBox="0 0 24 24">
      <!-- GitHub icon -->
    </svg>
  </a>
</div>
```

### Inline SVG in Text
```html
<p class="text-gray-600">
  <svg class="inline-block w-4 h-4 mr-1 fill-current" viewBox="0 0 20 20">
    <path d="..."/>
  </svg>
  Text with inline icon
</p>
```

### Custom Bullet Points
```html
<ul class="space-y-2">
  <li class="flex items-start">
    <svg class="w-5 h-5 text-green-500 fill-current shrink-0 mt-0.5 mr-2">
      <path d="..."/>
    </svg>
    <span>List item with custom bullet</span>
  </li>
</ul>
```

### SVG Background Pattern
```html
<div class="relative">
  <svg class="absolute inset-0 w-full h-full" aria-hidden="true">
    <defs>
      <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
        <path d="M 40 0 L 0 0 0 40" fill="none"
              class="stroke-gray-200" stroke-width="1"/>
      </pattern>
    </defs>
    <rect width="100%" height="100%" fill="url(#grid)"/>
  </svg>
  <div class="relative z-10">Content on top of pattern</div>
</div>
```

### Animated Heart Icon
```html
<button class="group">
  <svg class="w-6 h-6 fill-gray-300 group-hover:fill-red-500
              transition-colors group-active:scale-110 transform">
    <path d="..."/>
  </svg>
</button>
```

### Logo with Color Override
```html
<svg class="h-8 w-auto fill-blue-600 dark:fill-blue-400">
  <path d="..."/>
</svg>
```

### Decorative Divider
```html
<svg class="w-full h-12 text-gray-100 fill-current" viewBox="0 0 1200 120">
  <path d="M0,0V46.29c47.79,22.2,103.59,32.17,158,28,70.36-5.37,136.33-33.31,206.8-37.5C438.64,32.43,512.34,53.67,583,72.05c69.27,18,138.3,24.88,209.4,13.08,36.15-6,69.85-17.84,104.45-29.34C989.49,25,1113-14.29,1200,52.47V0Z" opacity=".25"></path>
  <path d="M0,0V15.81C13,36.92,27.64,56.86,47.69,72.05,99.41,111.27,165,111,224.58,91.58c31.15-10.15,60.09-26.07,89.67-39.8,40.92-19,84.73-46,130.83-49.67,36.26-2.85,70.9,9.42,98.6,31.56,31.77,25.39,62.32,62,103.63,73,40.44,10.79,81.35-6.69,119.13-24.28s75.16-39,116.92-43.05c59.73-5.85,113.28,22.88,168.9,38.84,30.2,8.66,59,6.17,87.09-7.5,22.43-10.89,48-26.93,60.65-49.24V0Z" opacity=".5"></path>
  <path d="M0,0V5.63C149.93,59,314.09,71.32,475.83,42.57c43-7.64,84.23-20.12,127.61-26.46,59-8.63,112.48,12.24,165.56,35.4C827.93,77.22,886,95.24,951.2,90c86.53-7,172.46-45.71,248.8-84.81V0Z"></path>
</svg>
```
