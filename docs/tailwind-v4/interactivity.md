# Interactivity Utilities (Tailwind CSS v4)

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

## Scroll Snap Type

```
snap-none   → scroll-snap-type: none
snap-x      → scroll-snap-type: x var(--tw-scroll-snap-strictness)
snap-y      → scroll-snap-type: y var(--tw-scroll-snap-strictness)
snap-both   → scroll-snap-type: both var(--tw-scroll-snap-strictness)
snap-mandatory → --tw-scroll-snap-strictness: mandatory
snap-proximity → --tw-scroll-snap-strictness: proximity
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

## Scroll Margin

```
scroll-m-0  → scroll-margin: 0px
scroll-m-1  → scroll-margin: 0.25rem
scroll-m-4  → scroll-margin: 1rem
scroll-mt-4 → scroll-margin-top: 1rem
scroll-mr-4 → scroll-margin-right: 1rem
scroll-mb-4 → scroll-margin-bottom: 1rem
scroll-ml-4 → scroll-margin-left: 1rem
scroll-mx-4 → scroll-margin-left: 1rem; scroll-margin-right: 1rem
scroll-my-4 → scroll-margin-top: 1rem; scroll-margin-bottom: 1rem
```

## Scroll Padding

```
scroll-p-0  → scroll-padding: 0px
scroll-p-4  → scroll-padding: 1rem
scroll-pt-4 → scroll-padding-top: 1rem
scroll-pr-4 → scroll-padding-right: 1rem
scroll-pb-4 → scroll-padding-bottom: 1rem
scroll-pl-4 → scroll-padding-left: 1rem
scroll-px-4 → scroll-padding-left: 1rem; scroll-padding-right: 1rem
scroll-py-4 → scroll-padding-top: 1rem; scroll-padding-bottom: 1rem
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

## Caret Color

```
caret-inherit     → caret-color: inherit
caret-current     → caret-color: currentColor
caret-transparent → caret-color: transparent
caret-black       → caret-color: #000
caret-white       → caret-color: #fff
caret-blue-500    → caret-color: rgb(59 130 246)
```

## Accent Color

For form elements like checkboxes and radio buttons:
```
accent-inherit   → accent-color: inherit
accent-current   → accent-color: currentColor
accent-auto      → accent-color: auto
accent-blue-500  → accent-color: rgb(59 130 246)
accent-pink-500  → accent-color: rgb(236 72 153)
```

## Appearance

```
appearance-none → appearance: none
appearance-auto → appearance: auto
```

## Common Patterns

### Disabled Button
```html
<button class="cursor-not-allowed opacity-50" disabled>
  Disabled
</button>
```

### Draggable Element
```html
<div class="cursor-grab active:cursor-grabbing">
  Drag me
</div>
```

### Non-Selectable Text
```html
<div class="select-none">
  This text cannot be selected
</div>
```

### Click-Through Overlay
```html
<div class="pointer-events-none fixed inset-0">
  <!-- Visual overlay that doesn't block clicks -->
</div>
```

### Smooth Scroll Container
```html
<html class="scroll-smooth">
  <a href="#section">Jump to section</a>
  <section id="section">...</section>
</html>
```

### Horizontal Scroll Snap
```html
<div class="flex snap-x snap-mandatory overflow-x-auto">
  <div class="snap-center shrink-0">Slide 1</div>
  <div class="snap-center shrink-0">Slide 2</div>
  <div class="snap-center shrink-0">Slide 3</div>
</div>
```

### Vertical Scroll Snap (Full Page)
```html
<div class="h-screen snap-y snap-mandatory overflow-y-auto">
  <section class="h-screen snap-start">Page 1</section>
  <section class="h-screen snap-start">Page 2</section>
  <section class="h-screen snap-start">Page 3</section>
</div>
```

### Sticky Header with Scroll Padding
```html
<html class="scroll-pt-16">
  <!-- Accounts for fixed header when jumping to anchors -->
  <header class="fixed top-0 h-16">...</header>
</html>
```

### Custom Checkbox Color
```html
<input type="checkbox" class="accent-pink-500" />
```

### Resizable Textarea
```html
<textarea class="resize-y">
  Can resize vertically only
</textarea>
```

### Non-Resizable Textarea
```html
<textarea class="resize-none">
  Cannot be resized
</textarea>
```

### Performance Hint
```html
<div class="will-change-transform">
  <!-- Hints browser to optimize for transform animations -->
</div>
```

### Image Gallery with Scroll Snap
```html
<div class="flex snap-x snap-mandatory gap-4 overflow-x-auto pb-4">
  <img class="snap-center shrink-0 w-80" src="..." />
  <img class="snap-center shrink-0 w-80" src="..." />
  <img class="snap-center shrink-0 w-80" src="..." />
</div>
```
