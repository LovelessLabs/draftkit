# Display & Visibility (Tailwind CSS v4)

## Display

### Block & Inline
```
block        → display: block
inline-block → display: inline-block
inline       → display: inline
```

### Flex & Grid
```
flex        → display: flex
inline-flex → display: inline-flex
grid        → display: grid
inline-grid → display: inline-grid
```

### Table
```
table              → display: table
inline-table       → display: inline-table
table-caption      → display: table-caption
table-cell         → display: table-cell
table-column       → display: table-column
table-column-group → display: table-column-group
table-footer-group → display: table-footer-group
table-header-group → display: table-header-group
table-row-group    → display: table-row-group
table-row          → display: table-row
```

### Other
```
flow-root → display: flow-root (clearfix alternative)
contents  → display: contents (removes box, children act as direct children of parent)
list-item → display: list-item
hidden    → display: none
```

## Visibility

```
visible   → visibility: visible
invisible → visibility: hidden (still takes space)
collapse  → visibility: collapse (for table rows/columns)
```

## Opacity (for hiding)

```
opacity-0   → opacity: 0 (invisible but interactive)
opacity-100 → opacity: 1 (fully visible)
```

## Overflow

### All Sides
```
overflow-auto    → overflow: auto
overflow-hidden  → overflow: hidden
overflow-clip    → overflow: clip (v4)
overflow-visible → overflow: visible
overflow-scroll  → overflow: scroll
```

### X-Axis
```
overflow-x-auto    → overflow-x: auto
overflow-x-hidden  → overflow-x: hidden
overflow-x-clip    → overflow-x: clip
overflow-x-visible → overflow-x: visible
overflow-x-scroll  → overflow-x: scroll
```

### Y-Axis
```
overflow-y-auto    → overflow-y: auto
overflow-y-hidden  → overflow-y: hidden
overflow-y-clip    → overflow-y: clip
overflow-y-visible → overflow-y: visible
overflow-y-scroll  → overflow-y: scroll
```

## Overscroll Behavior

```
overscroll-auto    → overscroll-behavior: auto
overscroll-contain → overscroll-behavior: contain (prevents scroll chaining)
overscroll-none    → overscroll-behavior: none

overscroll-x-auto    → overscroll-behavior-x: auto
overscroll-x-contain → overscroll-behavior-x: contain
overscroll-x-none    → overscroll-behavior-x: none

overscroll-y-auto    → overscroll-behavior-y: auto
overscroll-y-contain → overscroll-behavior-y: contain
overscroll-y-none    → overscroll-behavior-y: none
```

## Box Sizing

```
box-border  → box-sizing: border-box (default in Tailwind)
box-content → box-sizing: content-box
```

## Box Decoration Break

```
box-decoration-clone → box-decoration-break: clone
box-decoration-slice → box-decoration-break: slice
```

## Break

### Break Before
```
break-before-auto   → break-before: auto
break-before-avoid  → break-before: avoid
break-before-all    → break-before: all
break-before-avoid-page → break-before: avoid-page
break-before-page   → break-before: page
break-before-left   → break-before: left
break-before-right  → break-before: right
break-before-column → break-before: column
```

### Break After
```
break-after-auto   → break-after: auto
break-after-avoid  → break-after: avoid
break-after-all    → break-after: all
break-after-avoid-page → break-after: avoid-page
break-after-page   → break-after: page
break-after-left   → break-after: left
break-after-right  → break-after: right
break-after-column → break-after: column
```

### Break Inside
```
break-inside-auto        → break-inside: auto
break-inside-avoid       → break-inside: avoid
break-inside-avoid-page  → break-inside: avoid-page
break-inside-avoid-column → break-inside: avoid-column
```

## Columns

```
columns-1   → columns: 1
columns-2   → columns: 2
columns-3   → columns: 3
columns-4   → columns: 4
columns-5   → columns: 5
columns-6   → columns: 6
columns-7   → columns: 7
columns-8   → columns: 8
columns-9   → columns: 9
columns-10  → columns: 10
columns-11  → columns: 11
columns-12  → columns: 12
columns-auto → columns: auto
columns-3xs → columns: 16rem
columns-2xs → columns: 18rem
columns-xs  → columns: 20rem
columns-sm  → columns: 24rem
columns-md  → columns: 28rem
columns-lg  → columns: 32rem
columns-xl  → columns: 36rem
columns-2xl → columns: 42rem
columns-3xl → columns: 48rem
columns-4xl → columns: 56rem
columns-5xl → columns: 64rem
columns-6xl → columns: 72rem
columns-7xl → columns: 80rem
```

## Aspect Ratio

```
aspect-auto    → aspect-ratio: auto
aspect-square  → aspect-ratio: 1 / 1
aspect-video   → aspect-ratio: 16 / 9
aspect-[4/3]   → aspect-ratio: 4 / 3
```

## Container

```
container → max-width at each breakpoint
  sm: max-width: 640px
  md: max-width: 768px
  lg: max-width: 1024px
  xl: max-width: 1280px
  2xl: max-width: 1536px
```

## Common Patterns

### Show/Hide on Breakpoints
```html
<div class="hidden md:block">
  <!-- Hidden on mobile, visible on md+ -->
</div>

<div class="block md:hidden">
  <!-- Visible on mobile, hidden on md+ -->
</div>
```

### Screen Reader Only
```html
<span class="sr-only">
  <!-- Visually hidden but accessible -->
</span>
```

### Clearfix Alternative
```html
<div class="flow-root">
  <!-- Contains floated children -->
</div>
```

### Contents for Layout
```html
<div class="contents">
  <!-- Children act as if this div doesn't exist -->
</div>
```

### Scrollable Container
```html
<div class="h-64 overflow-y-auto">
  <!-- Scrollable content area -->
</div>
```

### Prevent Scroll Chaining
```html
<div class="overflow-y-auto overscroll-contain">
  <!-- Scrolling doesn't affect parent -->
</div>
```

### Multi-Column Text
```html
<div class="columns-3 gap-8">
  <p>Text flows across three columns...</p>
</div>
```

### Video Embed
```html
<div class="aspect-video">
  <iframe class="h-full w-full" src="..."></iframe>
</div>
```

### Centered Container
```html
<div class="container mx-auto px-4">
  <!-- Max-width container, centered -->
</div>
```

### Hide Without Removing Space
```html
<div class="invisible">
  <!-- Takes up space but not visible -->
</div>
```
