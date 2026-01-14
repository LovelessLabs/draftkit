# State Modifiers (Tailwind CSS v4)

## Pseudo-Class Modifiers

### Interactive States

```
hover:    → :hover
focus:    → :focus
focus-within: → :focus-within
focus-visible: → :focus-visible
active:   → :active
visited:  → :visited (links only)
target:   → :target (URL hash target)
```

### Form States

```
disabled:      → :disabled
enabled:       → :enabled
checked:       → :checked
indeterminate: → :indeterminate
default:       → :default
required:      → :required
valid:         → :valid
invalid:       → :invalid
in-range:      → :in-range
out-of-range:  → :out-of-range
placeholder-shown: → :placeholder-shown
autofill:      → :autofill
read-only:     → :read-only
```

### Structural States

```
first:       → :first-child
last:        → :last-child
only:        → :only-child
odd:         → :nth-child(odd)
even:        → :nth-child(even)
first-of-type: → :first-of-type
last-of-type:  → :last-of-type
only-of-type:  → :only-of-type
empty:       → :empty
```

### Open State

```
open: → :open (for <details> and <dialog> elements)
```

## Pseudo-Element Modifiers

```
before:      → ::before
after:       → ::after
placeholder: → ::placeholder
file:        → ::file-selector-button
marker:      → ::marker (list bullets)
selection:   → ::selection
first-line:  → ::first-line
first-letter: → ::first-letter
backdrop:    → ::backdrop (for dialogs)
```

## Group & Peer Modifiers

### Group (Parent-Based)

Style children based on parent state:
```html
<div class="group">
  <span class="group-hover:text-blue-500">
    Changes when parent is hovered
  </span>
</div>
```

Group variants:
```
group-hover:
group-focus:
group-focus-within:
group-focus-visible:
group-active:
group-visited:
group-disabled:
group-checked:
group-open:
group-first:
group-last:
group-odd:
group-even:
```

### Named Groups

```html
<div class="group/sidebar">
  <div class="group-hover/sidebar:bg-gray-100">...</div>
</div>
```

### Peer (Sibling-Based)

Style elements based on sibling state:
```html
<input class="peer" />
<p class="peer-invalid:text-red-500">
  Shows when input is invalid
</p>
```

Peer variants:
```
peer-hover:
peer-focus:
peer-focus-visible:
peer-active:
peer-disabled:
peer-checked:
peer-invalid:
peer-valid:
peer-placeholder-shown:
peer-required:
```

### Named Peers

```html
<input class="peer/email" type="email" />
<p class="peer-invalid/email:visible">Invalid email</p>
```

## Has Modifier (v4)

Style parent based on child state:
```html
<div class="has-[:focus]:ring-2">
  <input type="text" />
  <!-- Parent gets ring when child is focused -->
</div>
```

Variants:
```
has-[:checked]:
has-[:focus]:
has-[:disabled]:
has-[img]:
has-[>img]:
```

## Not Modifier (v4)

Apply styles when condition is NOT met:
```html
<div class="not-[:first-child]:mt-4">
  <!-- Margin on all except first child -->
</div>
```

## Dark Mode

```html
<div class="dark:bg-gray-900">
  Dark background in dark mode
</div>
```

## Responsive Breakpoints

```html
<div class="sm:flex md:grid lg:block">
  <!-- Different layout at each breakpoint -->
</div>
```

## Combining Modifiers

Stack modifiers left-to-right:
```html
<button class="dark:hover:bg-gray-700">
  <!-- Dark mode + hover -->
</button>

<button class="md:dark:hover:bg-gray-700">
  <!-- md breakpoint + dark mode + hover -->
</button>
```

## Common Patterns

### Button States
```html
<button class="
  bg-blue-500 text-white
  hover:bg-blue-600
  focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
  active:bg-blue-700
  disabled:opacity-50 disabled:cursor-not-allowed
">
  Button
</button>
```

### Input with Validation
```html
<input class="
  border border-gray-300
  focus:border-blue-500 focus:ring-1 focus:ring-blue-500
  invalid:border-red-500 invalid:focus:ring-red-500
" />
```

### Card Hover Effect
```html
<div class="group rounded-lg bg-white p-6 shadow hover:shadow-lg transition-shadow">
  <h3 class="group-hover:text-blue-500">Title</h3>
  <p class="text-gray-600">Content</p>
</div>
```

### Dropdown on Hover
```html
<div class="group relative">
  <button>Menu</button>
  <div class="invisible group-hover:visible absolute top-full">
    Dropdown content
  </div>
</div>
```

### Floating Label Input
```html
<div class="relative">
  <input class="peer pt-6" placeholder=" " />
  <label class="
    absolute left-3 top-4 text-gray-500
    transition-all
    peer-placeholder-shown:top-4 peer-placeholder-shown:text-base
    peer-focus:top-1 peer-focus:text-xs peer-focus:text-blue-500
  ">
    Label
  </label>
</div>
```

### Striped Table
```html
<tbody>
  <tr class="odd:bg-gray-50 even:bg-white">...</tr>
  <tr class="odd:bg-gray-50 even:bg-white">...</tr>
</tbody>
```

### First/Last Child Spacing
```html
<ul>
  <li class="border-b last:border-b-0">Item 1</li>
  <li class="border-b last:border-b-0">Item 2</li>
  <li class="border-b last:border-b-0">Item 3</li>
</ul>
```

### Custom Checkbox
```html
<label class="flex items-center gap-2">
  <input type="checkbox" class="peer sr-only" />
  <div class="
    h-5 w-5 rounded border border-gray-300
    peer-checked:bg-blue-500 peer-checked:border-blue-500
  ">
    <svg class="hidden peer-checked:block">✓</svg>
  </div>
  Label text
</label>
```

### Before/After Decorations
```html
<span class="
  relative
  before:absolute before:inset-x-0 before:-bottom-1
  before:h-0.5 before:bg-blue-500
">
  Underlined text
</span>
```

### Selection Styling
```html
<p class="selection:bg-pink-200 selection:text-pink-900">
  Select this text to see custom selection colors
</p>
```

### Placeholder Styling
```html
<input class="placeholder:text-gray-400 placeholder:italic" placeholder="Enter text..." />
```

### File Input Button
```html
<input type="file" class="
  file:mr-4 file:py-2 file:px-4
  file:rounded file:border-0
  file:bg-blue-500 file:text-white
  file:hover:bg-blue-600
" />
```

### Details/Summary Open State
```html
<details class="group">
  <summary>Click to expand</summary>
  <div class="group-open:animate-fadeIn">
    Content shown when open
  </div>
</details>
```

### Has Modifier Example
```html
<div class="has-[:checked]:bg-blue-50 p-4 rounded border">
  <input type="checkbox" />
  <span>Check to highlight parent</span>
</div>
```
