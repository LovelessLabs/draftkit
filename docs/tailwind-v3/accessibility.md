# Accessibility Utilities (Tailwind CSS v3)

## Screen Reader Utilities

### Screen Reader Only
```
sr-only     → Visually hidden but accessible to screen readers
not-sr-only → Reverses sr-only (makes visible again)
```

Implementation:
```css
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}
```

### Focus-Visible Screen Reader
```html
<a class="sr-only focus:not-sr-only">
  Skip to main content
</a>
```

## Focus Indicators

### Focus Ring
```
focus:ring          → box-shadow ring (3px default)
focus:ring-2        → 2px ring
focus:ring-4        → 4px ring
focus:ring-{color}  → ring color
focus:ring-offset-2 → space between element and ring
```

### Focus Visible (Keyboard Only)
```
focus-visible:ring-2         → ring only on keyboard focus
focus-visible:outline-2      → outline only on keyboard focus
focus-visible:ring-blue-500  → colored ring on keyboard focus
```

### Focus Within (Child Focus)
```
focus-within:ring-2          → ring when any child is focused
focus-within:border-blue-500 → border when child focused
```

## Outline Utilities
```
outline-none   → outline: 2px solid transparent; outline-offset: 2px
outline        → outline-style: solid
outline-dashed → outline-style: dashed
outline-dotted → outline-style: dotted
outline-double → outline-style: double

outline-0, outline-1, outline-2, outline-4, outline-8
outline-offset-0, outline-offset-1, outline-offset-2, outline-offset-4, outline-offset-8
outline-{color}
```

## Reduced Motion

### Motion Safe
```
motion-safe:animate-spin    → Only animate if user allows motion
motion-safe:transition-all  → Only transition if user allows motion
```

### Motion Reduce
```
motion-reduce:animate-none      → Disable animation for users who prefer reduced motion
motion-reduce:transition-none   → Disable transitions for users who prefer reduced motion
motion-reduce:transform-none    → Disable transforms for users who prefer reduced motion
```

## Contrast Preferences
```
contrast-more:border-black → Higher contrast for users who prefer it
contrast-less:border-gray-300 → Lower contrast when preferred
```

## Forced Colors Mode
```
forced-colors:border → Adjustments for Windows High Contrast Mode
```

## Common Patterns

### Skip Link
```html
<a href="#main-content"
   class="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4
          focus:z-50 focus:px-4 focus:py-2 focus:bg-white focus:text-black
          focus:ring-2 focus:ring-blue-500">
  Skip to main content
</a>
```

### Icon Button with Label
```html
<button aria-label="Close modal" class="p-2">
  <svg class="w-6 h-6" aria-hidden="true">...</svg>
  <span class="sr-only">Close modal</span>
</button>
```

### Accessible Form Field
```html
<div>
  <label for="email" class="block text-sm font-medium">
    Email address
  </label>
  <input
    id="email"
    type="email"
    aria-describedby="email-hint"
    class="mt-1 block w-full rounded-md border-gray-300
           focus:border-blue-500 focus:ring-blue-500">
  <p id="email-hint" class="mt-1 text-sm text-gray-500">
    We'll never share your email.
  </p>
</div>
```

### Error State with Accessibility
```html
<div>
  <label for="password">Password</label>
  <input
    id="password"
    type="password"
    aria-invalid="true"
    aria-describedby="password-error"
    class="border-red-500 focus:border-red-500 focus:ring-red-500">
  <p id="password-error" class="text-red-600 text-sm" role="alert">
    Password must be at least 8 characters.
  </p>
</div>
```

### Focus-Visible Button
```html
<button class="px-4 py-2 bg-blue-500 text-white rounded
               focus:outline-none
               focus-visible:ring-2 focus-visible:ring-blue-500
               focus-visible:ring-offset-2">
  Keyboard-friendly focus
</button>
```

### Card with Focus Within
```html
<div class="p-4 border rounded-lg
            focus-within:ring-2 focus-within:ring-blue-500
            focus-within:border-blue-500">
  <input type="text" class="w-full border-none focus:ring-0">
  <button>Submit</button>
</div>
```

### Reduced Motion Animation
```html
<div class="animate-bounce motion-reduce:animate-none">
  <svg><!-- Arrow icon --></svg>
</div>

<button class="transition-transform hover:scale-105
               motion-reduce:transform-none motion-reduce:transition-none">
  Button
</button>
```

### Loading Spinner with Reduced Motion
```html
<svg class="animate-spin motion-reduce:animate-none" aria-hidden="true">
  ...
</svg>
<span class="sr-only">Loading...</span>
```

### High Contrast Support
```html
<button class="bg-blue-500 text-white
               contrast-more:bg-blue-700 contrast-more:border-2 contrast-more:border-black">
  High contrast button
</button>

<p class="text-gray-500 contrast-more:text-gray-900">
  Better contrast when needed
</p>
```

### Keyboard Navigation List
```html
<ul role="menu" class="divide-y">
  <li role="menuitem" tabindex="0"
      class="p-2 focus:bg-gray-100 focus:outline-none
             focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-blue-500">
    Menu Item 1
  </li>
  <li role="menuitem" tabindex="0"
      class="p-2 focus:bg-gray-100 focus:outline-none
             focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-blue-500">
    Menu Item 2
  </li>
</ul>
```

### Descriptive Link
```html
<p>
  Read our
  <a href="/privacy" class="text-blue-500 underline
                            focus:outline-none focus:ring-2 focus:ring-blue-500">
    privacy policy
    <span class="sr-only">(opens in same window)</span>
  </a>
</p>
```

### External Link Indicator
```html
<a href="https://example.com" target="_blank" rel="noopener"
   class="inline-flex items-center text-blue-500">
  External Site
  <svg class="ml-1 w-4 h-4" aria-hidden="true">...</svg>
  <span class="sr-only">(opens in new tab)</span>
</a>
```

### Required Field Indicator
```html
<label for="name">
  Name
  <span class="text-red-500" aria-hidden="true">*</span>
  <span class="sr-only">(required)</span>
</label>
<input id="name" required>
```

### Tab Panel
```html
<div role="tablist" class="flex border-b">
  <button role="tab" aria-selected="true" aria-controls="panel-1"
          class="px-4 py-2 border-b-2 border-blue-500
                 focus:outline-none focus-visible:ring-2">
    Tab 1
  </button>
  <button role="tab" aria-selected="false" aria-controls="panel-2"
          class="px-4 py-2 border-b-2 border-transparent
                 focus:outline-none focus-visible:ring-2">
    Tab 2
  </button>
</div>

<div id="panel-1" role="tabpanel" tabindex="0"
     class="p-4 focus:outline-none focus-visible:ring-2 focus-visible:ring-inset">
  Panel 1 content
</div>
```

### Modal with Focus Trap
```html
<div role="dialog" aria-modal="true" aria-labelledby="modal-title"
     class="fixed inset-0 z-50 flex items-center justify-center">
  <div class="bg-white rounded-lg shadow-xl p-6">
    <h2 id="modal-title" class="text-xl font-bold">Modal Title</h2>
    <div class="mt-4">Content</div>
    <button class="mt-4 px-4 py-2 bg-blue-500 text-white rounded
                   focus:outline-none focus-visible:ring-2
                   focus-visible:ring-blue-500 focus-visible:ring-offset-2">
      Close
    </button>
  </div>
</div>
```
