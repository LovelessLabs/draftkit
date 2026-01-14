# State Variants (Tailwind CSS v3)

## Pseudo-class Variants

### Interactive States
```
hover:    → :hover
focus:    → :focus
focus-within: → :focus-within
focus-visible: → :focus-visible
active:   → :active
visited:  → :visited
target:   → :target
```

### Form States
```
disabled:     → :disabled
enabled:      → :enabled
checked:      → :checked
indeterminate: → :indeterminate
default:      → :default
required:     → :required
valid:        → :valid
invalid:      → :invalid
in-range:     → :in-range
out-of-range: → :out-of-range
placeholder-shown: → :placeholder-shown
autofill:     → :autofill
read-only:    → :read-only
```

### Structural Selectors
```
first:      → :first-child
last:       → :last-child
only:       → :only-child
odd:        → :nth-child(odd)
even:       → :nth-child(even)
first-of-type: → :first-of-type
last-of-type:  → :last-of-type
only-of-type:  → :only-of-type
empty:      → :empty
```

## Pseudo-element Variants

### Content Pseudo-elements
```
before: → ::before (requires content property)
after:  → ::after (requires content property)
```

### Form Pseudo-elements
```
placeholder: → ::placeholder
file:        → ::file-selector-button
```

### Text Pseudo-elements
```
first-letter: → ::first-letter
first-line:   → ::first-line
marker:       → ::marker (list markers)
selection:    → ::selection
```

### Backdrop
```
backdrop: → ::backdrop (for dialogs/fullscreen)
```

## Parent/Sibling Variants

### Group (Parent State)
```html
<div class="group">
  <span class="group-hover:text-blue-500">Changes when parent hovered</span>
</div>
```

Available group variants:
```
group-hover:
group-focus:
group-focus-within:
group-focus-visible:
group-active:
group-visited:
group-target:
group-first:
group-last:
group-odd:
group-even:
group-disabled:
group-enabled:
group-checked:
group-indeterminate:
group-default:
group-required:
group-valid:
group-invalid:
group-in-range:
group-out-of-range:
group-placeholder-shown:
group-autofill:
group-read-only:
```

### Named Groups
```html
<div class="group/sidebar">
  <div class="group/item">
    <span class="group-hover/item:text-blue-500">Item hover</span>
    <span class="group-hover/sidebar:text-red-500">Sidebar hover</span>
  </div>
</div>
```

### Peer (Sibling State)
```html
<input class="peer" type="checkbox">
<label class="peer-checked:text-blue-500">Styled based on sibling</label>
```

Available peer variants:
```
peer-hover:
peer-focus:
peer-focus-within:
peer-focus-visible:
peer-active:
peer-visited:
peer-target:
peer-first:
peer-last:
peer-disabled:
peer-enabled:
peer-checked:
peer-indeterminate:
peer-default:
peer-required:
peer-valid:
peer-invalid:
peer-in-range:
peer-out-of-range:
peer-placeholder-shown:
peer-autofill:
peer-read-only:
```

### Named Peers
```html
<input class="peer/email" type="email">
<input class="peer/password" type="password">
<p class="peer-invalid/email:block hidden">Invalid email</p>
<p class="peer-invalid/password:block hidden">Invalid password</p>
```

## Has Variant (Parent Has Child)
```html
<div class="has-[:checked]:bg-blue-50">
  <input type="checkbox">
  <label>Check me to change parent</label>
</div>

<!-- Common patterns -->
has-[:focus]:ring-2
has-[input]:p-4
has-[img]:aspect-video
```

## Responsive Variants
```
sm:   → @media (min-width: 640px)
md:   → @media (min-width: 768px)
lg:   → @media (min-width: 1024px)
xl:   → @media (min-width: 1280px)
2xl:  → @media (min-width: 1536px)
```

## Dark Mode
```
dark: → .dark & or @media (prefers-color-scheme: dark)
```

## Motion Preferences
```
motion-safe:   → @media (prefers-reduced-motion: no-preference)
motion-reduce: → @media (prefers-reduced-motion: reduce)
```

## Print
```
print: → @media print
```

## Contrast Preferences
```
contrast-more: → @media (prefers-contrast: more)
contrast-less: → @media (prefers-contrast: less)
```

## Portrait/Landscape
```
portrait:  → @media (orientation: portrait)
landscape: → @media (orientation: landscape)
```

## Forced Colors
```
forced-colors: → @media (forced-colors: active)
```

## LTR/RTL
```
ltr: → [dir="ltr"] &
rtl: → [dir="rtl"] &
```

## Open State (Details/Dialog)
```
open: → [open] (for <details> and <dialog>)
```

## Arbitrary Variants
```html
<!-- Custom selector -->
<div class="[&:nth-child(3)]:bg-red-500">Third child</div>

<!-- Targeting children -->
<div class="[&>*]:p-4">All direct children have padding</div>

<!-- Complex selectors -->
<div class="[&_.icon]:w-4">Icons inside have width</div>
```

## Common Patterns

### Button States
```html
<button class="bg-blue-500 hover:bg-blue-600 focus:ring-2
               focus:ring-blue-500 focus:ring-offset-2
               active:bg-blue-700 disabled:opacity-50
               disabled:cursor-not-allowed">
  Button
</button>
```

### Input States
```html
<input class="border border-gray-300
              focus:border-blue-500 focus:ring-1 focus:ring-blue-500
              invalid:border-red-500 invalid:text-red-600
              disabled:bg-gray-100 disabled:cursor-not-allowed
              placeholder:text-gray-400">
```

### Card Hover Effect
```html
<div class="group cursor-pointer">
  <img class="group-hover:scale-105 transition-transform" src="...">
  <h3 class="group-hover:text-blue-500">Title</h3>
</div>
```

### Checkbox Label
```html
<label class="flex items-center">
  <input type="checkbox" class="peer hidden">
  <span class="w-5 h-5 border rounded peer-checked:bg-blue-500
               peer-checked:border-blue-500"></span>
  <span class="ml-2 peer-checked:text-blue-500">Option</span>
</label>
```

### Floating Label
```html
<div class="relative">
  <input class="peer placeholder-transparent" placeholder="Email">
  <label class="absolute left-0 -top-3.5 text-sm
                peer-placeholder-shown:top-2 peer-placeholder-shown:text-base
                peer-focus:-top-3.5 peer-focus:text-sm
                transition-all">
    Email
  </label>
</div>
```

### Error Message
```html
<input class="peer" required>
<p class="hidden peer-invalid:block text-red-500 text-sm">
  This field is required
</p>
```

### Striped Table
```html
<tr class="odd:bg-gray-50 even:bg-white hover:bg-gray-100">
  <td>Row content</td>
</tr>
```

### First/Last Item Styling
```html
<li class="border-b last:border-b-0 first:pt-0 last:pb-0">
  List item
</li>
```

### Before/After Content
```html
<span class="before:content-['$'] before:text-gray-500">
  99.99
</span>

<a class="after:content-['_↗'] after:text-xs">
  External link
</a>
```

### Custom List Markers
```html
<ul class="marker:text-blue-500 list-disc pl-5">
  <li>Blue bullet point</li>
</ul>
```

### Selection Color
```html
<p class="selection:bg-pink-300 selection:text-pink-900">
  Select this text
</p>
```

### Reduced Motion
```html
<div class="motion-safe:animate-bounce motion-reduce:animate-none">
  Respects user preference
</div>
```

### Stacked Variants
```html
<!-- Multiple conditions -->
<button class="dark:hover:bg-gray-700">
  Dark mode + hover
</button>

<div class="md:hover:scale-105">
  Responsive + hover
</div>

<input class="dark:focus:ring-blue-400">
  Dark mode + focus
</input>
```
