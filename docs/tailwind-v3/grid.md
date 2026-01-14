# Grid Utilities (Tailwind CSS v3)

## Display Grid
```
grid        → display: grid
inline-grid → display: inline-grid
```

## Grid Template Columns
```
grid-cols-1  → grid-template-columns: repeat(1, minmax(0, 1fr))
grid-cols-2  → grid-template-columns: repeat(2, minmax(0, 1fr))
grid-cols-3  → grid-template-columns: repeat(3, minmax(0, 1fr))
grid-cols-4  → grid-template-columns: repeat(4, minmax(0, 1fr))
grid-cols-5  → grid-template-columns: repeat(5, minmax(0, 1fr))
grid-cols-6  → grid-template-columns: repeat(6, minmax(0, 1fr))
grid-cols-7  → grid-template-columns: repeat(7, minmax(0, 1fr))
grid-cols-8  → grid-template-columns: repeat(8, minmax(0, 1fr))
grid-cols-9  → grid-template-columns: repeat(9, minmax(0, 1fr))
grid-cols-10 → grid-template-columns: repeat(10, minmax(0, 1fr))
grid-cols-11 → grid-template-columns: repeat(11, minmax(0, 1fr))
grid-cols-12 → grid-template-columns: repeat(12, minmax(0, 1fr))
grid-cols-none → grid-template-columns: none
```

## Grid Template Rows
```
grid-rows-1  → grid-template-rows: repeat(1, minmax(0, 1fr))
grid-rows-2  → grid-template-rows: repeat(2, minmax(0, 1fr))
grid-rows-3  → grid-template-rows: repeat(3, minmax(0, 1fr))
grid-rows-4  → grid-template-rows: repeat(4, minmax(0, 1fr))
grid-rows-5  → grid-template-rows: repeat(5, minmax(0, 1fr))
grid-rows-6  → grid-template-rows: repeat(6, minmax(0, 1fr))
grid-rows-none → grid-template-rows: none
```

## Grid Column Span
```
col-auto     → grid-column: auto
col-span-1   → grid-column: span 1 / span 1
col-span-2   → grid-column: span 2 / span 2
col-span-3   → grid-column: span 3 / span 3
...
col-span-12  → grid-column: span 12 / span 12
col-span-full → grid-column: 1 / -1
```

## Grid Column Start/End
```
col-start-1  → grid-column-start: 1
col-start-2  → grid-column-start: 2
...
col-start-13 → grid-column-start: 13
col-start-auto → grid-column-start: auto

col-end-1    → grid-column-end: 1
col-end-2    → grid-column-end: 2
...
col-end-13   → grid-column-end: 13
col-end-auto → grid-column-end: auto
```

## Grid Row Span
```
row-auto     → grid-row: auto
row-span-1   → grid-row: span 1 / span 1
row-span-2   → grid-row: span 2 / span 2
row-span-3   → grid-row: span 3 / span 3
...
row-span-6   → grid-row: span 6 / span 6
row-span-full → grid-row: 1 / -1
```

## Grid Row Start/End
```
row-start-1  → grid-row-start: 1
row-start-2  → grid-row-start: 2
...
row-start-7  → grid-row-start: 7
row-start-auto → grid-row-start: auto

row-end-1    → grid-row-end: 1
row-end-2    → grid-row-end: 2
...
row-end-7    → grid-row-end: 7
row-end-auto → grid-row-end: auto
```

## Grid Auto Flow
```
grid-flow-row       → grid-auto-flow: row
grid-flow-col       → grid-auto-flow: column
grid-flow-dense     → grid-auto-flow: dense
grid-flow-row-dense → grid-auto-flow: row dense
grid-flow-col-dense → grid-auto-flow: column dense
```

## Grid Auto Columns
```
auto-cols-auto → grid-auto-columns: auto
auto-cols-min  → grid-auto-columns: min-content
auto-cols-max  → grid-auto-columns: max-content
auto-cols-fr   → grid-auto-columns: minmax(0, 1fr)
```

## Grid Auto Rows
```
auto-rows-auto → grid-auto-rows: auto
auto-rows-min  → grid-auto-rows: min-content
auto-rows-max  → grid-auto-rows: max-content
auto-rows-fr   → grid-auto-rows: minmax(0, 1fr)
```

## Gap
```
gap-0    → gap: 0px
gap-px   → gap: 1px
gap-0.5  → gap: 0.125rem (2px)
gap-1    → gap: 0.25rem (4px)
gap-2    → gap: 0.5rem (8px)
gap-4    → gap: 1rem (16px)
gap-6    → gap: 1.5rem (24px)
gap-8    → gap: 2rem (32px)
gap-x-4  → column-gap: 1rem
gap-y-4  → row-gap: 1rem
```

## Place Content
```
place-content-center  → place-content: center
place-content-start   → place-content: start
place-content-end     → place-content: end
place-content-between → place-content: space-between
place-content-around  → place-content: space-around
place-content-evenly  → place-content: space-evenly
place-content-baseline → place-content: baseline
place-content-stretch → place-content: stretch
```

## Place Items
```
place-items-start    → place-items: start
place-items-end      → place-items: end
place-items-center   → place-items: center
place-items-baseline → place-items: baseline
place-items-stretch  → place-items: stretch
```

## Place Self
```
place-self-auto    → place-self: auto
place-self-start   → place-self: start
place-self-end     → place-self: end
place-self-center  → place-self: center
place-self-stretch → place-self: stretch
```

## Justify Items
```
justify-items-start   → justify-items: start
justify-items-end     → justify-items: end
justify-items-center  → justify-items: center
justify-items-stretch → justify-items: stretch
```

## Justify Self
```
justify-self-auto    → justify-self: auto
justify-self-start   → justify-self: start
justify-self-end     → justify-self: end
justify-self-center  → justify-self: center
justify-self-stretch → justify-self: stretch
```

## Common Patterns

### Basic Grid Layout
```html
<div class="grid grid-cols-3 gap-4">
  <div>1</div>
  <div>2</div>
  <div>3</div>
</div>
```

### Responsive Grid
```html
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
  <!-- 1 column on mobile, 2 on small, 4 on large -->
</div>
```

### Spanning Columns
```html
<div class="grid grid-cols-4 gap-4">
  <div class="col-span-2">Spans 2 columns</div>
  <div>Single</div>
  <div>Single</div>
</div>
```

### Complex Layout
```html
<div class="grid grid-cols-3 grid-rows-3 gap-4">
  <div class="col-span-2 row-span-2">Main content</div>
  <div>Sidebar top</div>
  <div>Sidebar bottom</div>
  <div class="col-span-3">Footer</div>
</div>
```

### Auto-fill vs Auto-fit
```html
<!-- Auto-fill: Creates empty columns if space available -->
<div class="grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] gap-4">

<!-- Auto-fit: Stretches items to fill available space -->
<div class="grid grid-cols-[repeat(auto-fit,minmax(200px,1fr))] gap-4">
```

### Centering Grid Items
```html
<div class="grid place-items-center h-screen">
  <!-- Content centered both horizontally and vertically -->
</div>
```
