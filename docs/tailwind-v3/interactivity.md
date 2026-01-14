# Interactivity Utilities (Tailwind CSS v3)

## Cursor
```
cursor-auto          → cursor: auto
cursor-default       → cursor: default
cursor-pointer       → cursor: pointer
cursor-wait          → cursor: wait
cursor-text          → cursor: text
cursor-move          → cursor: move
cursor-help          → cursor: help
cursor-not-allowed   → cursor: not-allowed
cursor-none          → cursor: none
cursor-context-menu  → cursor: context-menu
cursor-progress      → cursor: progress
cursor-cell          → cursor: cell
cursor-crosshair     → cursor: crosshair
cursor-vertical-text → cursor: vertical-text
cursor-alias         → cursor: alias
cursor-copy          → cursor: copy
cursor-no-drop       → cursor: no-drop
cursor-grab          → cursor: grab
cursor-grabbing      → cursor: grabbing
cursor-all-scroll    → cursor: all-scroll
cursor-col-resize    → cursor: col-resize
cursor-row-resize    → cursor: row-resize
cursor-n-resize      → cursor: n-resize
cursor-e-resize      → cursor: e-resize
cursor-s-resize      → cursor: s-resize
cursor-w-resize      → cursor: w-resize
cursor-ne-resize     → cursor: ne-resize
cursor-nw-resize     → cursor: nw-resize
cursor-se-resize     → cursor: se-resize
cursor-sw-resize     → cursor: sw-resize
cursor-ew-resize     → cursor: ew-resize
cursor-ns-resize     → cursor: ns-resize
cursor-nesw-resize   → cursor: nesw-resize
cursor-nwse-resize   → cursor: nwse-resize
cursor-zoom-in       → cursor: zoom-in
cursor-zoom-out      → cursor: zoom-out
```

## Pointer Events
```
pointer-events-none → pointer-events: none
pointer-events-auto → pointer-events: auto
```

## Resize
```
resize-none → resize: none
resize-y    → resize: vertical
resize-x    → resize: horizontal
resize      → resize: both
```

## Scroll Behavior
```
scroll-auto   → scroll-behavior: auto
scroll-smooth → scroll-behavior: smooth
```

## Scroll Margin
```
scroll-m-0    → scroll-margin: 0px
scroll-m-1    → scroll-margin: 0.25rem
scroll-m-2    → scroll-margin: 0.5rem
scroll-m-4    → scroll-margin: 1rem
scroll-m-8    → scroll-margin: 2rem

scroll-mx-{size}, scroll-my-{size}
scroll-mt-{size}, scroll-mr-{size}, scroll-mb-{size}, scroll-ml-{size}
```

## Scroll Padding
```
scroll-p-0    → scroll-padding: 0px
scroll-p-1    → scroll-padding: 0.25rem
scroll-p-2    → scroll-padding: 0.5rem
scroll-p-4    → scroll-padding: 1rem
scroll-p-8    → scroll-padding: 2rem

scroll-px-{size}, scroll-py-{size}
scroll-pt-{size}, scroll-pr-{size}, scroll-pb-{size}, scroll-pl-{size}
```

## Scroll Snap Align
```
snap-start  → scroll-snap-align: start
snap-end    → scroll-snap-align: end
snap-center → scroll-snap-align: center
snap-align-none → scroll-snap-align: none
```

## Scroll Snap Stop
```
snap-normal → scroll-snap-stop: normal
snap-always → scroll-snap-stop: always
```

## Scroll Snap Type
```
snap-none    → scroll-snap-type: none
snap-x       → scroll-snap-type: x var(--tw-scroll-snap-strictness)
snap-y       → scroll-snap-type: y var(--tw-scroll-snap-strictness)
snap-both    → scroll-snap-type: both var(--tw-scroll-snap-strictness)
snap-mandatory → --tw-scroll-snap-strictness: mandatory
snap-proximity → --tw-scroll-snap-strictness: proximity
```

## Touch Action
```
touch-auto        → touch-action: auto
touch-none        → touch-action: none
touch-pan-x       → touch-action: pan-x
touch-pan-left    → touch-action: pan-left
touch-pan-right   → touch-action: pan-right
touch-pan-y       → touch-action: pan-y
touch-pan-up      → touch-action: pan-up
touch-pan-down    → touch-action: pan-down
touch-pinch-zoom  → touch-action: pinch-zoom
touch-manipulation → touch-action: manipulation
```

## User Select
```
select-none → user-select: none
select-text → user-select: text
select-all  → user-select: all
select-auto → user-select: auto
```

## Will Change
```
will-change-auto      → will-change: auto
will-change-scroll    → will-change: scroll-position
will-change-contents  → will-change: contents
will-change-transform → will-change: transform
```

## Appearance
```
appearance-none → appearance: none
appearance-auto → appearance: auto
```

## Caret Color
```
caret-{color}-{shade} → caret-color: {value}
caret-inherit
caret-current
caret-transparent
```

## Accent Color
```
accent-{color}-{shade} → accent-color: {value}
accent-inherit
accent-current
accent-transparent
accent-auto → accent-color: auto
```

## Common Patterns

### Clickable Element
```html
<div class="cursor-pointer hover:bg-gray-100">
  Click me
</div>
```

### Disabled Button
```html
<button disabled class="cursor-not-allowed opacity-50">
  Disabled
</button>
```

### Draggable Element
```html
<div class="cursor-grab active:cursor-grabbing">
  Drag me
</div>
```

### Non-Interactive Overlay
```html
<div class="pointer-events-none absolute inset-0">
  <span class="pointer-events-auto">Only this is clickable</span>
</div>
```

### Resizable Textarea
```html
<textarea class="resize-y min-h-[100px]"></textarea>
```

### Smooth Scroll Container
```html
<div class="scroll-smooth overflow-y-auto h-96">
  <a href="#section1">Jump to Section 1</a>
  <div id="section1">Section 1</div>
</div>
```

### Horizontal Scroll Snap
```html
<div class="flex overflow-x-auto snap-x snap-mandatory">
  <div class="snap-center shrink-0 w-80">Slide 1</div>
  <div class="snap-center shrink-0 w-80">Slide 2</div>
  <div class="snap-center shrink-0 w-80">Slide 3</div>
</div>
```

### Vertical Scroll Snap
```html
<div class="overflow-y-auto snap-y snap-mandatory h-screen">
  <section class="snap-start h-screen">Page 1</section>
  <section class="snap-start h-screen">Page 2</section>
  <section class="snap-start h-screen">Page 3</section>
</div>
```

### Scroll Snap with Padding
```html
<div class="snap-y snap-mandatory scroll-pt-16 overflow-y-auto">
  <!-- Accounts for fixed header -->
  <div class="snap-start">Content</div>
</div>
```

### Prevent Text Selection
```html
<div class="select-none">
  Cannot select this text
</div>
```

### Select All on Click
```html
<code class="select-all">
  npm install tailwindcss
</code>
```

### Mobile Touch Optimization
```html
<div class="touch-manipulation">
  No double-tap zoom delay
</div>
```

### Prevent Vertical Scroll on Carousel
```html
<div class="touch-pan-x overflow-x-auto">
  Horizontal-only touch scrolling
</div>
```

### Custom Form Accent
```html
<input type="checkbox" class="accent-pink-500">
<input type="range" class="accent-blue-600">
```

### Custom Caret Color
```html
<input type="text" class="caret-pink-500">
```

### Performance Hint
```html
<div class="will-change-transform hover:scale-105 transition-transform">
  Optimized for animation
</div>
```

### Custom Checkbox/Radio
```html
<input type="checkbox" class="appearance-none w-5 h-5 border-2 rounded checked:bg-blue-500">
```
