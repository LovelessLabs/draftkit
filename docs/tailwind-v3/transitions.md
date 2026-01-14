# Transitions & Animation Utilities (Tailwind CSS v3)

## Transition Property
```
transition-none → transition-property: none
transition-all  → transition-property: all; duration: 150ms; timing: cubic-bezier(0.4, 0, 0.2, 1)
transition      → transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; duration: 150ms
transition-colors  → transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; duration: 150ms
transition-opacity → transition-property: opacity; duration: 150ms
transition-shadow  → transition-property: box-shadow; duration: 150ms
transition-transform → transition-property: transform; duration: 150ms
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

## Built-in Animations
```
animate-none   → animation: none
animate-spin   → animation: spin 1s linear infinite
animate-ping   → animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite
animate-pulse  → animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite
animate-bounce → animation: bounce 1s infinite
```

### Spin Animation
```css
@keyframes spin {
  to { transform: rotate(360deg); }
}
```

### Ping Animation
```css
@keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}
```

### Pulse Animation
```css
@keyframes pulse {
  50% { opacity: .5; }
}
```

### Bounce Animation
```css
@keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    animation-timing-function: cubic-bezier(0.8,0,1,1);
  }
  50% {
    transform: none;
    animation-timing-function: cubic-bezier(0,0,0.2,1);
  }
}
```

## Common Patterns

### Button Hover Effect
```html
<button class="bg-blue-500 hover:bg-blue-600 transition-colors duration-200">
  Smooth color transition
</button>
```

### Scale on Hover
```html
<div class="transform hover:scale-105 transition-transform duration-300">
  Grows smoothly
</div>
```

### Fade In
```html
<div class="opacity-0 hover:opacity-100 transition-opacity duration-300">
  Fades in on hover
</div>
```

### Multiple Properties
```html
<button class="bg-blue-500 hover:bg-blue-600 hover:shadow-lg
               transform hover:scale-105 transition-all duration-200">
  Multiple transitions
</button>
```

### Loading Spinner
```html
<svg class="animate-spin h-5 w-5 text-white" viewBox="0 0 24 24">
  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
</svg>
```

### Notification Badge
```html
<span class="relative">
  <span class="animate-ping absolute h-3 w-3 rounded-full bg-red-400 opacity-75"></span>
  <span class="relative h-3 w-3 rounded-full bg-red-500"></span>
</span>
```

### Skeleton Loading
```html
<div class="animate-pulse flex space-x-4">
  <div class="rounded-full bg-gray-200 h-10 w-10"></div>
  <div class="flex-1 space-y-4 py-1">
    <div class="h-4 bg-gray-200 rounded w-3/4"></div>
    <div class="h-4 bg-gray-200 rounded"></div>
  </div>
</div>
```

### Bouncing Arrow
```html
<svg class="animate-bounce w-6 h-6">
  <!-- Down arrow icon -->
</svg>
```

### Staggered Delay
```html
<div class="opacity-0 animate-fade-in delay-0">Item 1</div>
<div class="opacity-0 animate-fade-in delay-100">Item 2</div>
<div class="opacity-0 animate-fade-in delay-200">Item 3</div>
```

### Transition with Ease
```html
<div class="transform translate-y-0 hover:-translate-y-1
            transition-transform duration-300 ease-out">
  Lifts up smoothly
</div>
```

### Reduced Motion
```html
<div class="motion-reduce:transition-none motion-reduce:transform-none">
  Respects user's motion preferences
</div>
```

### Card Hover Effect
```html
<div class="group">
  <div class="bg-white shadow-md group-hover:shadow-xl
              transition-shadow duration-300">
    <img class="group-hover:scale-105 transition-transform duration-300" src="...">
  </div>
</div>
```

### Menu Slide
```html
<!-- Closed -->
<nav class="transform -translate-x-full transition-transform duration-300">

<!-- Open -->
<nav class="transform translate-x-0 transition-transform duration-300">
```
