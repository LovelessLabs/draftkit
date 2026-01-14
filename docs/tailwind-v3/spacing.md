# Spacing Utilities (Tailwind CSS v3)

## Spacing Scale

Tailwind uses a consistent spacing scale based on 0.25rem (4px) increments:

```
0     = 0px
px    = 1px
0.5   = 0.125rem (2px)
1     = 0.25rem (4px)
1.5   = 0.375rem (6px)
2     = 0.5rem (8px)
2.5   = 0.625rem (10px)
3     = 0.75rem (12px)
3.5   = 0.875rem (14px)
4     = 1rem (16px)
5     = 1.25rem (20px)
6     = 1.5rem (24px)
7     = 1.75rem (28px)
8     = 2rem (32px)
9     = 2.25rem (36px)
10    = 2.5rem (40px)
11    = 2.75rem (44px)
12    = 3rem (48px)
14    = 3.5rem (56px)
16    = 4rem (64px)
20    = 5rem (80px)
24    = 6rem (96px)
28    = 7rem (112px)
32    = 8rem (128px)
36    = 9rem (144px)
40    = 10rem (160px)
44    = 11rem (176px)
48    = 12rem (192px)
52    = 13rem (208px)
56    = 14rem (224px)
60    = 15rem (240px)
64    = 16rem (256px)
72    = 18rem (288px)
80    = 20rem (320px)
96    = 24rem (384px)
```

## Padding

### All Sides
```
p-0   → padding: 0px
p-px  → padding: 1px
p-1   → padding: 0.25rem (4px)
p-2   → padding: 0.5rem (8px)
p-4   → padding: 1rem (16px)
p-8   → padding: 2rem (32px)
```

### Horizontal & Vertical
```
px-4  → padding-left: 1rem; padding-right: 1rem
py-4  → padding-top: 1rem; padding-bottom: 1rem
```

### Individual Sides
```
pt-4  → padding-top: 1rem
pr-4  → padding-right: 1rem
pb-4  → padding-bottom: 1rem
pl-4  → padding-left: 1rem
```

### Logical Properties (RTL-aware)
```
ps-4  → padding-inline-start: 1rem
pe-4  → padding-inline-end: 1rem
```

## Margin

### All Sides
```
m-0   → margin: 0px
m-px  → margin: 1px
m-1   → margin: 0.25rem (4px)
m-2   → margin: 0.5rem (8px)
m-4   → margin: 1rem (16px)
m-auto → margin: auto
```

### Horizontal & Vertical
```
mx-4   → margin-left: 1rem; margin-right: 1rem
my-4   → margin-top: 1rem; margin-bottom: 1rem
mx-auto → margin-left: auto; margin-right: auto
```

### Individual Sides
```
mt-4  → margin-top: 1rem
mr-4  → margin-right: 1rem
mb-4  → margin-bottom: 1rem
ml-4  → margin-left: 1rem
```

### Logical Properties (RTL-aware)
```
ms-4  → margin-inline-start: 1rem
me-4  → margin-inline-end: 1rem
```

### Negative Margins
```
-m-4   → margin: -1rem
-mt-4  → margin-top: -1rem
-mx-4  → margin-left: -1rem; margin-right: -1rem
```

## Space Between

Adds margin between child elements (not on first/last):

```
space-x-4 → margin-left: 1rem (on all but first child)
space-y-4 → margin-top: 1rem (on all but first child)
space-x-reverse → reverses margin direction
space-y-reverse → reverses margin direction
```

### How Space Between Works
```html
<div class="flex space-x-4">
  <div>No margin</div>
  <div>ml-4</div>
  <div>ml-4</div>
</div>
```

Generates CSS:
```css
.space-x-4 > :not([hidden]) ~ :not([hidden]) {
  margin-left: 1rem;
}
```

### Negative Space
```
-space-x-4 → margin-left: -1rem (overlapping items)
-space-y-4 → margin-top: -1rem
```

## Common Patterns

### Centering with Auto Margins
```html
<div class="mx-auto max-w-md">
  <!-- Horizontally centered container -->
</div>
```

### Card Padding
```html
<div class="p-6 bg-white rounded-lg shadow">
  <h2 class="mb-4">Title</h2>
  <p>Content with consistent spacing</p>
</div>
```

### Stacked List Items
```html
<ul class="space-y-4">
  <li>Item 1</li>
  <li>Item 2</li>
  <li>Item 3</li>
</ul>
```

### Responsive Spacing
```html
<div class="p-4 md:p-6 lg:p-8">
  <!-- More padding on larger screens -->
</div>
```

### Negative Margin for Full-Bleed
```html
<div class="px-4">
  <div class="-mx-4 bg-gray-100 px-4">
    <!-- Extends to edge of parent's padding -->
  </div>
</div>
```

### Inline Horizontal Spacing
```html
<div class="flex space-x-2">
  <button>Save</button>
  <button>Cancel</button>
</div>
```

### Section Spacing
```html
<section class="py-12 md:py-20">
  <!-- Vertical rhythm between sections -->
</section>
```

### Asymmetric Padding
```html
<div class="pt-4 pb-8 px-6">
  <!-- Different padding on different sides -->
</div>
```
