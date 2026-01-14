# Responsive Design (Tailwind CSS v4)

## Breakpoints

Tailwind uses mobile-first breakpoints. Classes without a prefix apply to all screen sizes. Prefixed classes apply at that breakpoint and above.

### Default Breakpoints
```
sm  → @media (min-width: 640px)  { ... }
md  → @media (min-width: 768px)  { ... }
lg  → @media (min-width: 1024px) { ... }
xl  → @media (min-width: 1280px) { ... }
2xl → @media (min-width: 1536px) { ... }
```

## Mobile-First Approach

Always design for mobile first, then add larger breakpoint overrides:

```html
<!-- Mobile: 1 column, Tablet: 2 columns, Desktop: 4 columns -->
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4">
  ...
</div>
```

The unprefixed class (`grid-cols-1`) applies to all sizes, then `sm:` overrides at 640px+, and `lg:` overrides at 1024px+.

## Common Responsive Patterns

### Stack to Row
```html
<div class="flex flex-col md:flex-row">
  <!-- Column on mobile, row on desktop -->
</div>
```

### Hide/Show by Breakpoint
```html
<div class="hidden md:block">
  <!-- Hidden on mobile, visible on md+ -->
</div>

<div class="block md:hidden">
  <!-- Visible on mobile, hidden on md+ -->
</div>
```

### Responsive Text Size
```html
<h1 class="text-2xl md:text-4xl lg:text-6xl">
  Scales with screen size
</h1>
```

### Responsive Spacing
```html
<div class="p-4 md:p-8 lg:p-12">
  <!-- More padding on larger screens -->
</div>

<section class="py-12 md:py-24">
  <!-- Responsive vertical padding -->
</section>
```

### Responsive Width
```html
<div class="w-full md:w-1/2 lg:w-1/3">
  <!-- Full width mobile, half on tablet, third on desktop -->
</div>
```

### Responsive Grid
```html
<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
  <!-- Responsive column count -->
</div>
```

### Responsive Container
```html
<div class="mx-auto max-w-sm md:max-w-xl lg:max-w-4xl">
  <!-- Grows with screen size -->
</div>
```

### Responsive Gap
```html
<div class="grid gap-4 md:gap-6 lg:gap-8">
  <!-- Larger gaps on larger screens -->
</div>
```

## Max-Width Breakpoints (v4)

Use `max-*` prefix for maximum-width media queries:
```
max-sm  → @media (max-width: 639px)  { ... }
max-md  → @media (max-width: 767px)  { ... }
max-lg  → @media (max-width: 1023px) { ... }
max-xl  → @media (max-width: 1279px) { ... }
max-2xl → @media (max-width: 1535px) { ... }
```

Example:
```html
<div class="max-md:hidden">
  <!-- Hidden below md breakpoint -->
</div>
```

## Range Breakpoints (v4)

Target specific breakpoint ranges:
```html
<div class="md:max-lg:text-center">
  <!-- Only centered between md and lg -->
</div>
```

## Arbitrary Breakpoints

Use arbitrary values for custom breakpoints:
```html
<div class="min-[320px]:text-center min-[1140px]:max-w-5xl">
  <!-- Custom breakpoints -->
</div>
```

## Container Queries (v4)

Style based on parent container size instead of viewport:

### Container Setup
```html
<div class="@container">
  <!-- Children can use @-prefixed responsive classes -->
</div>
```

### Container Query Classes
```
@sm  → @container (min-width: 20rem)
@md  → @container (min-width: 28rem)
@lg  → @container (min-width: 32rem)
@xl  → @container (min-width: 36rem)
@2xl → @container (min-width: 42rem)
```

### Usage
```html
<div class="@container">
  <div class="flex flex-col @md:flex-row @lg:gap-8">
    <!-- Responsive to container, not viewport -->
  </div>
</div>
```

### Named Containers
```html
<div class="@container/main">
  <div class="@md/main:flex">
    <!-- Responsive to specifically named container -->
  </div>
</div>
```

## Responsive Utility Examples

### Navigation
```html
<nav class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
  <div class="text-xl font-bold">Logo</div>
  <div class="mt-4 flex gap-4 sm:mt-0">
    <a href="#">Link 1</a>
    <a href="#">Link 2</a>
  </div>
</nav>
```

### Card Grid
```html
<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
  <div class="rounded-lg bg-white p-6 shadow">Card 1</div>
  <div class="rounded-lg bg-white p-6 shadow">Card 2</div>
  <div class="rounded-lg bg-white p-6 shadow">Card 3</div>
</div>
```

### Sidebar Layout
```html
<div class="flex flex-col lg:flex-row">
  <aside class="w-full shrink-0 lg:w-64">
    Sidebar
  </aside>
  <main class="flex-1">
    Content
  </main>
</div>
```

### Responsive Table
```html
<!-- Table on desktop, cards on mobile -->
<div class="hidden md:block">
  <table>...</table>
</div>
<div class="space-y-4 md:hidden">
  <!-- Card layout for mobile -->
</div>
```

### Hero Section
```html
<section class="px-4 py-12 text-center md:px-8 md:py-24 lg:py-32">
  <h1 class="text-3xl font-bold md:text-5xl lg:text-6xl">
    Welcome
  </h1>
  <p class="mx-auto mt-4 max-w-xl text-gray-600 md:mt-6 md:text-lg">
    Description text
  </p>
</section>
```

## Best Practices

1. **Start mobile** - Write base styles for mobile, add breakpoints for larger screens
2. **Use fewer breakpoints** - Often you only need `sm` and `lg`, not all five
3. **Group related changes** - If changing flex direction, also adjust margins at same breakpoint
4. **Test at each breakpoint** - Check designs at 640px, 768px, 1024px, etc.
5. **Consider container queries** - For component-level responsiveness in v4
