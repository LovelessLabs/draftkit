# Display Utilities (Tailwind CSS v3)

## Display Values
```
block       → display: block
inline-block → display: inline-block
inline      → display: inline
flex        → display: flex
inline-flex → display: inline-flex
table       → display: table
inline-table → display: inline-table
table-caption → display: table-caption
table-cell   → display: table-cell
table-column → display: table-column
table-column-group → display: table-column-group
table-footer-group → display: table-footer-group
table-header-group → display: table-header-group
table-row-group → display: table-row-group
table-row    → display: table-row
flow-root    → display: flow-root
grid        → display: grid
inline-grid → display: inline-grid
contents    → display: contents
list-item   → display: list-item
hidden      → display: none
```

## Visibility
```
visible   → visibility: visible
invisible → visibility: hidden
collapse  → visibility: collapse
```

## Common Patterns

### Show/Hide Elements
```html
<!-- Completely removed from layout -->
<div class="hidden">Not rendered</div>

<!-- Invisible but takes up space -->
<div class="invisible">Space preserved</div>
```

### Responsive Display
```html
<!-- Hidden on mobile, visible on md+ -->
<div class="hidden md:block">
  Desktop content
</div>

<!-- Visible on mobile, hidden on md+ -->
<div class="block md:hidden">
  Mobile content
</div>
```

### Inline Block for Sizing
```html
<span class="inline-block w-32 h-32 bg-blue-500">
  Sized inline element
</span>
```

### Flow Root (Clearfix Alternative)
```html
<div class="flow-root">
  <div class="float-left">Floated content</div>
  <!-- Container contains floated children -->
</div>
```

### Contents (Remove Wrapper)
```html
<div class="flex gap-4">
  <div class="contents">
    <!-- These become direct flex children -->
    <div>Item 1</div>
    <div>Item 2</div>
  </div>
</div>
```

### Table Layout
```html
<div class="table w-full">
  <div class="table-row">
    <div class="table-cell p-2">Cell 1</div>
    <div class="table-cell p-2">Cell 2</div>
  </div>
  <div class="table-row">
    <div class="table-cell p-2">Cell 3</div>
    <div class="table-cell p-2">Cell 4</div>
  </div>
</div>
```

### List Item
```html
<div class="list-item list-disc ml-4">
  Acts like a list item
</div>
```

### Collapse for Table Rows
```html
<table class="table-auto">
  <tr>
    <td>Visible row</td>
  </tr>
  <tr class="collapse">
    <!-- Row removed without affecting table layout -->
    <td>Collapsed row</td>
  </tr>
</table>
```
