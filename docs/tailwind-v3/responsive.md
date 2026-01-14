# Responsive Design (Tailwind CSS v3)

## Breakpoints

Tailwind uses a mobile-first approach. Utilities without a prefix apply at all screen sizes, while prefixed utilities apply at the specified breakpoint and above.

```
sm:   → @media (min-width: 640px)
md:   → @media (min-width: 768px)
lg:   → @media (min-width: 1024px)
xl:   → @media (min-width: 1280px)
2xl:  → @media (min-width: 1536px)
```

## Mobile-First Philosophy

Start with mobile styles, then add responsive modifiers for larger screens:

```html
<!-- Mobile: 1 column, Tablet: 2 columns, Desktop: 4 columns -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4">
  ...
</div>
```

## Container

The container class sets max-width at each breakpoint:

```
container → width: 100%
           sm:max-width: 640px
           md:max-width: 768px
           lg:max-width: 1024px
           xl:max-width: 1280px
           2xl:max-width: 1536px
```

```html
<div class="container mx-auto px-4">
  Centered, responsive container
</div>
```

## Common Patterns

### Responsive Grid
```html
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
  <div>Item 1</div>
  <div>Item 2</div>
  <div>Item 3</div>
  <div>Item 4</div>
</div>
```

### Responsive Flexbox
```html
<!-- Stack on mobile, row on tablet+ -->
<div class="flex flex-col md:flex-row gap-4">
  <div>Sidebar</div>
  <div>Main content</div>
</div>
```

### Responsive Typography
```html
<h1 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl">
  Responsive heading
</h1>
```

### Responsive Spacing
```html
<section class="py-8 md:py-12 lg:py-16">
  <div class="px-4 md:px-8 lg:px-16">
    Content with responsive padding
  </div>
</section>
```

### Show/Hide at Breakpoints
```html
<!-- Mobile only -->
<div class="block md:hidden">
  Mobile navigation
</div>

<!-- Tablet and up -->
<div class="hidden md:block">
  Desktop navigation
</div>

<!-- Only on specific range -->
<div class="hidden md:block lg:hidden">
  Tablet only
</div>
```

### Responsive Width
```html
<div class="w-full md:w-1/2 lg:w-1/3">
  Full width on mobile, half on tablet, third on desktop
</div>
```

### Responsive Sidebar Layout
```html
<div class="flex flex-col lg:flex-row">
  <!-- Sidebar: full width on mobile, fixed width on desktop -->
  <aside class="w-full lg:w-64 lg:shrink-0">
    Sidebar
  </aside>

  <!-- Main: takes remaining space -->
  <main class="flex-1">
    Main content
  </main>
</div>
```

### Responsive Card Layout
```html
<div class="grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
  <div class="p-4 sm:p-6 lg:p-8 rounded-lg shadow">
    Card with responsive padding
  </div>
</div>
```

### Responsive Image
```html
<!-- Different aspect ratios at breakpoints -->
<div class="aspect-square md:aspect-video lg:aspect-[21/9]">
  <img class="object-cover w-full h-full" src="..." alt="">
</div>
```

### Responsive Navigation
```html
<nav class="flex flex-col md:flex-row md:items-center md:justify-between">
  <div class="text-xl font-bold">Logo</div>

  <ul class="flex flex-col md:flex-row gap-2 md:gap-4 mt-4 md:mt-0">
    <li><a href="#">Home</a></li>
    <li><a href="#">About</a></li>
    <li><a href="#">Contact</a></li>
  </ul>
</nav>
```

### Responsive Table
```html
<!-- Stacked cards on mobile, table on desktop -->
<div class="block lg:hidden">
  <!-- Mobile card view -->
  <div class="space-y-4">
    <div class="border p-4 rounded">
      <div class="font-bold">Name: John</div>
      <div>Email: john@example.com</div>
    </div>
  </div>
</div>

<table class="hidden lg:table w-full">
  <!-- Desktop table view -->
  <thead>
    <tr>
      <th>Name</th>
      <th>Email</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>John</td>
      <td>john@example.com</td>
    </tr>
  </tbody>
</table>
```

### Responsive Font Sizes
```html
<article class="prose prose-sm md:prose-base lg:prose-lg xl:prose-xl">
  <!-- Typography scales with viewport -->
</article>
```

### Responsive Gap
```html
<div class="flex flex-wrap gap-2 sm:gap-4 md:gap-6 lg:gap-8">
  <div>Item</div>
  <div>Item</div>
</div>
```

### Hero Section
```html
<section class="min-h-[50vh] md:min-h-[70vh] lg:min-h-screen
                py-12 md:py-20 lg:py-32
                px-4 md:px-8 lg:px-16">
  <h1 class="text-3xl md:text-5xl lg:text-7xl font-bold">
    Hero Title
  </h1>
  <p class="text-lg md:text-xl lg:text-2xl mt-4 md:mt-6">
    Subtitle text
  </p>
</section>
```

### Two-Column with Reversed Order
```html
<div class="flex flex-col md:flex-row">
  <!-- Image first on mobile, second on desktop -->
  <div class="order-1 md:order-2">Image</div>
  <div class="order-2 md:order-1">Text</div>
</div>
```

## Max-Width Breakpoints

For "down" breakpoints (max-width), use arbitrary values:

```html
<!-- Only below md -->
<div class="max-md:hidden">Hidden below 768px</div>

<!-- Only below lg -->
<div class="max-lg:flex-col">Column below 1024px</div>
```

## Custom Breakpoints

Configure in tailwind.config.js:

```javascript
module.exports = {
  theme: {
    screens: {
      'xs': '475px',
      'sm': '640px',
      'md': '768px',
      'lg': '1024px',
      'xl': '1280px',
      '2xl': '1536px',
      '3xl': '1920px',
    }
  }
}
```
