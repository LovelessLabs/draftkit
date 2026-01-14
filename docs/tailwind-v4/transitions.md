# Transitions & Animation (Tailwind CSS v4)

## Transition Property

```
transition-none → transition-property: none
transition-all  → transition-property: all
transition      → transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter
transition-colors → transition-property: color, background-color, border-color, text-decoration-color, fill, stroke
transition-opacity → transition-property: opacity
transition-shadow → transition-property: box-shadow
transition-transform → transition-property: transform
```

## Transition Duration

```
duration-0    → transition-duration: 0s
duration-75   → transition-duration: 75ms
duration-100  → transition-duration: 100ms
duration-150  → transition-duration: 150ms
duration-200  → transition-duration: 200ms
duration-300  → transition-duration: 300ms
duration-500  → transition-duration: 500ms
duration-700  → transition-duration: 700ms
duration-1000 → transition-duration: 1000ms
```

## Transition Timing Function

```
ease-linear → transition-timing-function: linear
ease-in     → transition-timing-function: cubic-bezier(0.4, 0, 1, 1)
ease-out    → transition-timing-function: cubic-bezier(0, 0, 0.2, 1)
ease-in-out → transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)
```

## Transition Delay

```
delay-0    → transition-delay: 0s
delay-75   → transition-delay: 75ms
delay-100  → transition-delay: 100ms
delay-150  → transition-delay: 150ms
delay-200  → transition-delay: 200ms
delay-300  → transition-delay: 300ms
delay-500  → transition-delay: 500ms
delay-700  → transition-delay: 700ms
delay-1000 → transition-delay: 1000ms
```

## Animation

### Built-in Animations
```
animate-none    → animation: none
animate-spin    → animation: spin 1s linear infinite
animate-ping    → animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite
animate-pulse   → animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite
animate-bounce  → animation: bounce 1s infinite
```

### Animation Keyframes (built-in)

**spin** - Continuous rotation:
```css
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
```

**ping** - Radar ping effect:
```css
@keyframes ping {
  75%, 100% { transform: scale(2); opacity: 0; }
}
```

**pulse** - Gentle fade:
```css
@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: .5; }
}
```

**bounce** - Bouncing effect:
```css
@keyframes bounce {
  0%, 100% { transform: translateY(-25%); animation-timing-function: cubic-bezier(0.8, 0, 1, 1); }
  50% { transform: translateY(0); animation-timing-function: cubic-bezier(0, 0, 0.2, 1); }
}
```

## Motion Preferences

Respect user's reduced motion preference:
```
motion-safe:animate-spin   → Only animate if user hasn't requested reduced motion
motion-reduce:animate-none → Remove animation if reduced motion requested
```

## Arbitrary Values

```
duration-[2000ms]       → transition-duration: 2000ms
delay-[2s]              → transition-delay: 2s
ease-[cubic-bezier(0.95,0.05,0.795,0.035)] → custom easing
animate-[wiggle_1s_ease-in-out_infinite] → custom animation
```

## Common Patterns

### Hover Transition
```html
<button class="bg-blue-500 transition-colors duration-200 hover:bg-blue-600">
  Smooth color change
</button>
```

### Multiple Properties
```html
<div class="transition-all duration-300 hover:scale-105 hover:shadow-lg">
  <!-- Scale and shadow transition together -->
</div>
```

### Loading Spinner
```html
<svg class="animate-spin h-5 w-5 text-white" viewBox="0 0 24 24">
  <!-- Spinner SVG -->
</svg>
```

### Notification Ping
```html
<span class="relative flex h-3 w-3">
  <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-sky-400 opacity-75"></span>
  <span class="relative inline-flex rounded-full h-3 w-3 bg-sky-500"></span>
</span>
```

### Skeleton Loading
```html
<div class="animate-pulse">
  <div class="h-4 bg-gray-200 rounded w-3/4"></div>
  <div class="h-4 bg-gray-200 rounded w-1/2 mt-2"></div>
</div>
```

### Staggered Animations
```html
<div class="transition-opacity duration-300 delay-0">Item 1</div>
<div class="transition-opacity duration-300 delay-100">Item 2</div>
<div class="transition-opacity duration-300 delay-200">Item 3</div>
```

### Button with Transform
```html
<button class="transform transition-transform duration-150 active:scale-95">
  Press me
</button>
```

### Accessible Animation
```html
<div class="motion-safe:animate-bounce motion-reduce:animate-none">
  <!-- Only bounces if user allows motion -->
</div>
```

### Fade In on Hover
```html
<div class="group">
  <img src="..." />
  <div class="opacity-0 transition-opacity duration-300 group-hover:opacity-100">
    Overlay content
  </div>
</div>
```

### Slide Down Menu
```html
<div class="transform transition-all duration-200 origin-top scale-y-0 group-hover:scale-y-100">
  <!-- Dropdown content -->
</div>
```
