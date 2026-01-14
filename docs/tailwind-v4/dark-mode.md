# Dark Mode (Tailwind CSS v4)

## Enabling Dark Mode

In Tailwind CSS v4, dark mode works out of the box using the CSS `prefers-color-scheme` media query.

### Default Behavior (Media Strategy)
```css
/* v4 default - uses system preference */
@media (prefers-color-scheme: dark) {
  .dark\:bg-gray-900 { background-color: rgb(17 24 39); }
}
```

### Class Strategy
To control dark mode with a class on the root element:
```css
/* In your CSS */
@import "tailwindcss";

@variant dark (&:where(.dark, .dark *));
```

Then toggle with JavaScript:
```html
<html class="dark">
  <!-- Dark mode enabled -->
</html>
```

### Selector Strategy (v4)
For custom selectors:
```css
@variant dark (&:where([data-theme="dark"], [data-theme="dark"] *));
```

## Using Dark Mode Classes

Prefix any utility with `dark:` to apply it in dark mode:

```html
<div class="bg-white dark:bg-gray-900">
  <!-- White in light mode, dark gray in dark mode -->
</div>

<p class="text-gray-900 dark:text-gray-100">
  <!-- Dark text in light mode, light text in dark mode -->
</p>
```

## Common Dark Mode Patterns

### Basic Card
```html
<div class="rounded-lg bg-white dark:bg-gray-800 shadow dark:shadow-none">
  <h3 class="text-gray-900 dark:text-white">Title</h3>
  <p class="text-gray-600 dark:text-gray-300">Content</p>
</div>
```

### Page Background
```html
<body class="bg-gray-50 dark:bg-gray-900">
  <!-- Light gray in light mode, near-black in dark mode -->
</body>
```

### Borders
```html
<div class="border border-gray-200 dark:border-gray-700">
  <!-- Lighter border in light mode, darker in dark mode -->
</div>
```

### Input Fields
```html
<input class="
  border border-gray-300 dark:border-gray-600
  bg-white dark:bg-gray-800
  text-gray-900 dark:text-gray-100
  placeholder-gray-400 dark:placeholder-gray-500
  focus:ring-blue-500 dark:focus:ring-blue-400
" />
```

### Primary Button
```html
<button class="
  bg-blue-600 hover:bg-blue-700
  dark:bg-blue-500 dark:hover:bg-blue-600
  text-white
">
  Button
</button>
```

### Links
```html
<a class="text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300">
  Link text
</a>
```

### Dividers
```html
<hr class="border-gray-200 dark:border-gray-700" />
```

### Icons/SVG
```html
<svg class="text-gray-500 dark:text-gray-400">
  <!-- Fill inherits from text color via currentColor -->
</svg>
```

## Color Palette for Dark Mode

Recommended color mappings:

### Backgrounds
```
Light Mode          Dark Mode
bg-white         → dark:bg-gray-900  (page)
bg-gray-50       → dark:bg-gray-800  (cards/sections)
bg-gray-100      → dark:bg-gray-700  (elevated elements)
```

### Text
```
Light Mode          Dark Mode
text-gray-900    → dark:text-white   (headings)
text-gray-700    → dark:text-gray-200 (primary text)
text-gray-600    → dark:text-gray-300 (secondary text)
text-gray-500    → dark:text-gray-400 (muted text)
```

### Borders
```
Light Mode          Dark Mode
border-gray-200  → dark:border-gray-700
border-gray-300  → dark:border-gray-600
```

### Shadows
```html
<!-- Shadows often look better removed in dark mode -->
<div class="shadow-lg dark:shadow-none">
  <!-- Or use colored shadows -->
</div>
<div class="shadow-lg dark:shadow-blue-500/10">
  <!-- Subtle colored glow in dark mode -->
</div>
```

## Advanced Patterns

### Ring Focus States
```html
<button class="
  focus:ring-2
  focus:ring-blue-500 dark:focus:ring-blue-400
  focus:ring-offset-2
  focus:ring-offset-white dark:focus:ring-offset-gray-900
">
  Focus me
</button>
```

### Hover States
```html
<div class="
  bg-gray-100 hover:bg-gray-200
  dark:bg-gray-800 dark:hover:bg-gray-700
">
  Hoverable
</div>
```

### Images with Dark Overlay
```html
<div class="relative">
  <img src="..." class="dark:opacity-80" />
  <!-- Slightly dim images in dark mode -->
</div>
```

### Inverted Logo
```html
<!-- Show different logos for each mode -->
<img src="/logo-dark.svg" class="dark:hidden" />
<img src="/logo-light.svg" class="hidden dark:block" />
```

### Dark Mode Toggle (JS)
```html
<button onclick="document.documentElement.classList.toggle('dark')">
  Toggle Dark Mode
</button>
```

### System + Manual Toggle (JS)
```javascript
// Check for saved preference or system preference
if (localStorage.theme === 'dark' ||
    (!localStorage.theme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
  document.documentElement.classList.add('dark');
}

// Toggle
function toggleDarkMode() {
  document.documentElement.classList.toggle('dark');
  localStorage.theme = document.documentElement.classList.contains('dark') ? 'dark' : 'light';
}

// Reset to system preference
function useSystemTheme() {
  localStorage.removeItem('theme');
  // Re-evaluate based on system preference
}
```

## Full Page Example

```html
<body class="bg-gray-50 text-gray-900 dark:bg-gray-900 dark:text-gray-100">
  <header class="border-b border-gray-200 dark:border-gray-800">
    <nav class="mx-auto max-w-7xl px-4">
      <h1 class="text-xl font-bold">Logo</h1>
    </nav>
  </header>

  <main class="mx-auto max-w-7xl px-4 py-8">
    <div class="rounded-lg bg-white p-6 shadow dark:bg-gray-800 dark:shadow-none">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
        Card Title
      </h2>
      <p class="mt-2 text-gray-600 dark:text-gray-300">
        Card content goes here.
      </p>
      <button class="mt-4 rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600">
        Action
      </button>
    </div>
  </main>
</body>
```

## Best Practices

1. **Design both modes simultaneously** - Don't treat dark mode as an afterthought
2. **Test contrast ratios** - Ensure text remains readable in both modes
3. **Use semantic color names** - Consider `bg-surface` instead of `bg-white dark:bg-gray-900`
4. **Reduce harsh contrasts** - Pure white on pure black is hard to read; use off-whites and grays
5. **Consider shadows** - Shadows often don't work well in dark mode; use subtle borders or glows instead
6. **Dim images slightly** - Full brightness images can be jarring in dark mode
