# Dark Mode (Tailwind CSS v3)

## Configuration

Dark mode can be configured in `tailwind.config.js`:

```javascript
module.exports = {
  // 'media' - based on OS preference (default)
  darkMode: 'media',

  // 'class' - based on .dark class on html/body
  darkMode: 'class',

  // 'selector' - custom selector (v3.4+)
  darkMode: ['selector', '[data-theme="dark"]'],
}
```

## Usage

The `dark:` variant applies styles when dark mode is active:

```html
<div class="bg-white dark:bg-gray-900 text-gray-900 dark:text-white">
  Adapts to color scheme
</div>
```

## Class Strategy (Recommended)

With `darkMode: 'class'`, toggle the `.dark` class on the `<html>` element:

```html
<!-- Light mode -->
<html>
  <body class="bg-white text-black">...</body>
</html>

<!-- Dark mode -->
<html class="dark">
  <body class="bg-white dark:bg-gray-900 text-black dark:text-white">...</body>
</html>
```

### Toggle with JavaScript
```javascript
// Toggle dark mode
document.documentElement.classList.toggle('dark');

// Check preference and set
if (localStorage.theme === 'dark' ||
    (!('theme' in localStorage) &&
     window.matchMedia('(prefers-color-scheme: dark)').matches)) {
  document.documentElement.classList.add('dark');
} else {
  document.documentElement.classList.remove('dark');
}

// Manually set
localStorage.theme = 'dark';    // Dark
localStorage.theme = 'light';   // Light
localStorage.removeItem('theme'); // System
```

## Media Strategy

With `darkMode: 'media'`, dark mode follows the operating system preference:

```html
<!-- Automatically responds to OS setting -->
<div class="bg-white dark:bg-gray-900">
  Follows system preference
</div>
```

## Common Patterns

### Basic Dark Mode
```html
<body class="bg-white dark:bg-gray-900">
  <h1 class="text-gray-900 dark:text-white">Heading</h1>
  <p class="text-gray-600 dark:text-gray-400">Body text</p>
</body>
```

### Card Component
```html
<div class="bg-white dark:bg-gray-800
            border border-gray-200 dark:border-gray-700
            shadow-md dark:shadow-gray-900/50
            rounded-lg p-6">
  <h2 class="text-gray-900 dark:text-white">Card Title</h2>
  <p class="text-gray-500 dark:text-gray-400">Card content</p>
</div>
```

### Button
```html
<button class="bg-blue-500 dark:bg-blue-600
               hover:bg-blue-600 dark:hover:bg-blue-700
               text-white
               focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400
               focus:ring-offset-2 dark:focus:ring-offset-gray-900">
  Button
</button>
```

### Input Field
```html
<input class="bg-white dark:bg-gray-800
              border border-gray-300 dark:border-gray-600
              text-gray-900 dark:text-white
              placeholder-gray-400 dark:placeholder-gray-500
              focus:border-blue-500 dark:focus:border-blue-400
              focus:ring-blue-500 dark:focus:ring-blue-400">
```

### Navigation
```html
<nav class="bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800">
  <a class="text-gray-700 dark:text-gray-300
            hover:text-gray-900 dark:hover:text-white">
    Link
  </a>
</nav>
```

### Table
```html
<table class="w-full">
  <thead class="bg-gray-50 dark:bg-gray-800">
    <tr>
      <th class="text-gray-900 dark:text-white">Header</th>
    </tr>
  </thead>
  <tbody class="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
    <tr class="hover:bg-gray-50 dark:hover:bg-gray-800">
      <td class="text-gray-700 dark:text-gray-300">Cell</td>
    </tr>
  </tbody>
</table>
```

### Modal
```html
<!-- Backdrop -->
<div class="fixed inset-0 bg-black/50 dark:bg-black/70"></div>

<!-- Modal -->
<div class="bg-white dark:bg-gray-800
            border border-gray-200 dark:border-gray-700
            rounded-lg shadow-xl dark:shadow-gray-900/50">
  <div class="border-b border-gray-200 dark:border-gray-700 p-4">
    <h2 class="text-gray-900 dark:text-white">Modal Title</h2>
  </div>
  <div class="p-4 text-gray-700 dark:text-gray-300">
    Content
  </div>
</div>
```

### Code Block
```html
<pre class="bg-gray-100 dark:bg-gray-800
            text-gray-800 dark:text-gray-200
            border border-gray-200 dark:border-gray-700
            rounded-lg p-4">
  <code>console.log('Hello')</code>
</pre>
```

### Badge/Tag
```html
<span class="bg-blue-100 dark:bg-blue-900
             text-blue-800 dark:text-blue-200
             px-2 py-1 rounded-full text-sm">
  Badge
</span>
```

### Alert
```html
<div class="bg-red-50 dark:bg-red-900/20
            border border-red-200 dark:border-red-800
            text-red-800 dark:text-red-200
            rounded-lg p-4">
  Error message
</div>
```

### SVG/Icons
```html
<svg class="text-gray-500 dark:text-gray-400">
  <path fill="currentColor" d="..."/>
</svg>
```

### Invert Images for Dark Mode
```html
<img class="dark:invert" src="logo-dark.svg" alt="">
```

### Theme Toggle Button
```html
<button id="theme-toggle" class="p-2 rounded-lg
  bg-gray-100 dark:bg-gray-800
  text-gray-800 dark:text-gray-200">
  <!-- Sun icon (shown in dark mode) -->
  <svg class="hidden dark:block w-5 h-5">...</svg>
  <!-- Moon icon (shown in light mode) -->
  <svg class="block dark:hidden w-5 h-5">...</svg>
</button>
```

## Stacking with Other Variants

Dark mode can be combined with other variants:

```html
<!-- Dark mode + hover -->
<button class="bg-white dark:bg-gray-800
               hover:bg-gray-100 dark:hover:bg-gray-700">
  Button
</button>

<!-- Dark mode + focus -->
<input class="focus:ring-blue-500 dark:focus:ring-blue-400">

<!-- Dark mode + responsive -->
<div class="md:bg-white md:dark:bg-gray-800">
  Responsive dark mode
</div>

<!-- Dark mode + group -->
<div class="group">
  <span class="group-hover:text-blue-500 dark:group-hover:text-blue-400">
    Text
  </span>
</div>
```

## Color Recommendations

### Light Mode → Dark Mode Colors
```
white      → gray-900 or gray-950
gray-50    → gray-800
gray-100   → gray-700
gray-200   → gray-600
gray-900   → white or gray-100
black      → white
```

### Semantic Color Adjustments
```
blue-500   → blue-400 (slightly lighter for contrast)
red-500    → red-400
green-500  → green-400
```

### Border Colors
```
gray-200   → gray-700
gray-300   → gray-600
```

### Shadow Adjustments
```
shadow-md             → shadow-md shadow-gray-900/50
shadow-lg             → shadow-lg shadow-black/50
ring-offset-white     → ring-offset-gray-900
```
