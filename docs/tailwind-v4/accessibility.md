# Accessibility Utilities (Tailwind CSS v4)

## Screen Reader Only

Hide content visually but keep it accessible to screen readers:
```
sr-only     → Hides element visually, remains in accessibility tree
not-sr-only → Reverses sr-only, makes element visible again
```

### Implementation
```html
<button>
  <svg>...</svg>
  <span class="sr-only">Close menu</span>
</button>
```

### Focus-Visible SR-Only
```html
<a class="sr-only focus:not-sr-only" href="#main-content">
  Skip to main content
</a>
```

## Forced Colors Mode

Utilities for Windows High Contrast Mode:
```
forced-color-adjust-auto → forced-color-adjust: auto
forced-color-adjust-none → forced-color-adjust: none
```

## Motion Preferences

Respect user's motion preferences:
```
motion-safe:   → @media (prefers-reduced-motion: no-preference)
motion-reduce: → @media (prefers-reduced-motion: reduce)
```

### Usage
```html
<!-- Only animate if user allows motion -->
<div class="motion-safe:animate-bounce motion-reduce:animate-none">
  Bouncing element
</div>

<!-- Shorter transitions for reduced motion -->
<div class="transition-transform duration-300 motion-reduce:duration-0">
  Animated element
</div>
```

## Print Styles

```
print: → @media print
```

### Usage
```html
<nav class="print:hidden">
  <!-- Hide navigation when printing -->
</nav>

<article class="print:text-black print:bg-white">
  <!-- Ensure readability in print -->
</article>
```

## Focus Styles

### Focus Ring (Modern)
```html
<button class="
  focus:outline-none
  focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2
">
  <!-- Only shows ring on keyboard focus, not mouse clicks -->
</button>
```

### Focus Within
```html
<div class="focus-within:ring-2 focus-within:ring-blue-500">
  <input type="text" />
  <!-- Parent shows ring when child is focused -->
</div>
```

## Color Contrast

Use sufficient contrast ratios. Recommended combinations:

### WCAG AA (4.5:1 for normal text)
```html
<!-- Good contrast -->
<p class="text-gray-900 bg-white">Dark on light</p>
<p class="text-white bg-gray-900">Light on dark</p>
<p class="text-gray-700 bg-gray-100">Sufficient contrast</p>

<!-- Poor contrast - avoid -->
<p class="text-gray-400 bg-gray-100">Low contrast</p>
```

### Large Text (3:1 ratio acceptable)
```html
<h1 class="text-4xl text-gray-600">
  Large text can have slightly lower contrast
</h1>
```

## Touch Targets

Ensure touch targets are at least 44x44px:
```html
<button class="min-h-[44px] min-w-[44px] p-2">
  <!-- Adequate touch target size -->
</button>

<a class="inline-block p-2">
  <svg class="h-6 w-6">...</svg>
  <!-- Link with padding for larger touch area -->
</a>
```

## Common Patterns

### Skip Link
```html
<a href="#main" class="
  sr-only focus:not-sr-only
  focus:absolute focus:top-4 focus:left-4
  focus:z-50 focus:rounded focus:bg-white focus:px-4 focus:py-2
  focus:ring-2 focus:ring-blue-500
">
  Skip to main content
</a>
```

### Icon Button with Label
```html
<button aria-label="Close menu" class="p-2">
  <svg class="h-6 w-6" aria-hidden="true">
    <!-- X icon -->
  </svg>
  <span class="sr-only">Close menu</span>
</button>
```

### Form with Labels
```html
<div>
  <label for="email" class="block text-sm font-medium">Email</label>
  <input
    id="email"
    type="email"
    aria-describedby="email-help"
    class="mt-1 w-full rounded border px-3 py-2"
  />
  <p id="email-help" class="mt-1 text-sm text-gray-500">
    We'll never share your email.
  </p>
</div>
```

### Error State with ARIA
```html
<div>
  <label for="password">Password</label>
  <input
    id="password"
    type="password"
    aria-invalid="true"
    aria-describedby="password-error"
    class="border-red-500"
  />
  <p id="password-error" class="text-red-500" role="alert">
    Password must be at least 8 characters
  </p>
</div>
```

### Accessible Modal
```html
<div
  role="dialog"
  aria-modal="true"
  aria-labelledby="modal-title"
  class="fixed inset-0 z-50 flex items-center justify-center"
>
  <div class="fixed inset-0 bg-black/50" aria-hidden="true"></div>
  <div class="relative rounded-lg bg-white p-6">
    <h2 id="modal-title" class="text-lg font-semibold">Modal Title</h2>
    <p>Modal content...</p>
  </div>
</div>
```

### Loading State
```html
<button disabled class="opacity-50" aria-busy="true">
  <svg class="animate-spin motion-reduce:hidden" aria-hidden="true">...</svg>
  <span>Loading...</span>
</button>
```

### Decorative Images
```html
<!-- Decorative - hide from screen readers -->
<img src="decoration.svg" alt="" aria-hidden="true" class="..." />

<!-- Meaningful - provide alt text -->
<img src="chart.png" alt="Sales increased 25% in Q4" class="..." />
```

### Live Region
```html
<div aria-live="polite" class="sr-only">
  <!-- Screen reader will announce changes here -->
</div>
```

### Tab Panel
```html
<div role="tablist">
  <button
    role="tab"
    aria-selected="true"
    aria-controls="panel-1"
    class="border-b-2 border-blue-500 px-4 py-2"
  >
    Tab 1
  </button>
  <button
    role="tab"
    aria-selected="false"
    aria-controls="panel-2"
    class="px-4 py-2 text-gray-500"
  >
    Tab 2
  </button>
</div>
<div id="panel-1" role="tabpanel">Content 1</div>
<div id="panel-2" role="tabpanel" hidden>Content 2</div>
```

### Reduced Motion Example
```html
<div class="
  transition-all duration-300
  motion-safe:hover:scale-105
  motion-reduce:transition-none motion-reduce:hover:scale-100
">
  Card that respects motion preferences
</div>
```

### Print-Optimized Page
```html
<body class="print:bg-white print:text-black">
  <header class="print:hidden">Navigation</header>
  <main class="print:max-w-none">
    <article class="prose print:prose-sm">
      Content
    </article>
  </main>
  <footer class="print:hidden">Footer</footer>
</body>
```

## Best Practices

1. **Always provide focus states** - Use `focus-visible:` for keyboard-only focus indicators
2. **Use semantic HTML** - `<button>` not `<div onclick>`, `<nav>` not `<div class="nav">`
3. **Test with screen readers** - VoiceOver (Mac), NVDA (Windows), TalkBack (Android)
4. **Check color contrast** - Use tools like WebAIM Contrast Checker
5. **Respect motion preferences** - Use `motion-safe:` for animations
6. **Ensure adequate touch targets** - 44x44px minimum
7. **Provide text alternatives** - `sr-only` text for icon-only buttons
8. **Test keyboard navigation** - Tab through your entire interface
