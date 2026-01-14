# Form Styling (Tailwind CSS v4)

## Input Fields

### Basic Text Input
```html
<input type="text" class="
  w-full rounded-md border border-gray-300 px-3 py-2
  text-gray-900 placeholder-gray-400
  focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500
" placeholder="Enter text..." />
```

### Input Sizes
```html
<!-- Small -->
<input class="rounded px-2 py-1 text-sm" />

<!-- Default -->
<input class="rounded-md px-3 py-2" />

<!-- Large -->
<input class="rounded-lg px-4 py-3 text-lg" />
```

### Input States
```html
<!-- Disabled -->
<input class="disabled:bg-gray-100 disabled:cursor-not-allowed" disabled />

<!-- Read-only -->
<input class="read-only:bg-gray-50" readonly />

<!-- Error state -->
<input class="border-red-500 focus:border-red-500 focus:ring-red-500" />

<!-- Success state -->
<input class="border-green-500 focus:border-green-500 focus:ring-green-500" />
```

### Input with Icon
```html
<div class="relative">
  <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
    <svg class="h-5 w-5 text-gray-400">...</svg>
  </div>
  <input class="w-full rounded-md border border-gray-300 py-2 pl-10 pr-3" />
</div>
```

### Input with Button
```html
<div class="flex">
  <input class="flex-1 rounded-l-md border border-r-0 border-gray-300 px-3 py-2" />
  <button class="rounded-r-md bg-blue-500 px-4 py-2 text-white">
    Submit
  </button>
</div>
```

## Textarea

```html
<textarea class="
  w-full rounded-md border border-gray-300 px-3 py-2
  focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500
  resize-y
" rows="4" placeholder="Enter message..."></textarea>
```

## Select

### Native Select
```html
<select class="
  w-full rounded-md border border-gray-300 bg-white px-3 py-2
  focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500
">
  <option>Option 1</option>
  <option>Option 2</option>
  <option>Option 3</option>
</select>
```

### Custom Select Arrow
```html
<div class="relative">
  <select class="w-full appearance-none rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10">
    <option>Option 1</option>
  </select>
  <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
    <svg class="h-5 w-5 text-gray-400">▼</svg>
  </div>
</div>
```

## Checkbox

### Basic Checkbox
```html
<label class="flex items-center gap-2">
  <input type="checkbox" class="
    h-4 w-4 rounded border-gray-300 text-blue-500
    focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
  " />
  <span class="text-gray-700">Remember me</span>
</label>
```

### Custom Checkbox with Accent Color
```html
<input type="checkbox" class="
  h-5 w-5 rounded accent-blue-500
" />
```

### Custom Styled Checkbox (CSS-only)
```html
<label class="relative flex cursor-pointer items-center gap-3">
  <input type="checkbox" class="peer sr-only" />
  <div class="
    h-5 w-5 rounded border-2 border-gray-300
    peer-checked:border-blue-500 peer-checked:bg-blue-500
    flex items-center justify-center
  ">
    <svg class="hidden h-3 w-3 text-white peer-checked:block">
      <path d="M1 3l2 2 4-4" stroke="currentColor" stroke-width="2" fill="none" />
    </svg>
  </div>
  <span>Label text</span>
</label>
```

## Radio Buttons

### Basic Radio Group
```html
<fieldset class="space-y-2">
  <legend class="text-sm font-medium text-gray-700">Select option</legend>
  <label class="flex items-center gap-2">
    <input type="radio" name="option" class="
      h-4 w-4 border-gray-300 text-blue-500
      focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
    " />
    <span>Option A</span>
  </label>
  <label class="flex items-center gap-2">
    <input type="radio" name="option" class="h-4 w-4 border-gray-300 text-blue-500" />
    <span>Option B</span>
  </label>
</fieldset>
```

### Radio Card Group
```html
<div class="space-y-2">
  <label class="relative flex cursor-pointer rounded-lg border p-4 has-[:checked]:border-blue-500 has-[:checked]:bg-blue-50">
    <input type="radio" name="plan" class="sr-only" />
    <div>
      <span class="font-medium">Basic Plan</span>
      <p class="text-sm text-gray-500">$10/month</p>
    </div>
  </label>
  <label class="relative flex cursor-pointer rounded-lg border p-4 has-[:checked]:border-blue-500 has-[:checked]:bg-blue-50">
    <input type="radio" name="plan" class="sr-only" />
    <div>
      <span class="font-medium">Pro Plan</span>
      <p class="text-sm text-gray-500">$25/month</p>
    </div>
  </label>
</div>
```

## Toggle Switch

```html
<label class="relative inline-flex cursor-pointer items-center">
  <input type="checkbox" class="peer sr-only" />
  <div class="
    h-6 w-11 rounded-full bg-gray-200
    peer-checked:bg-blue-500
    after:absolute after:left-[2px] after:top-[2px]
    after:h-5 after:w-5 after:rounded-full after:bg-white
    after:transition-transform after:content-['']
    peer-checked:after:translate-x-5
  "></div>
  <span class="ml-3">Toggle</span>
</label>
```

## File Input

```html
<input type="file" class="
  text-sm text-gray-500
  file:mr-4 file:rounded file:border-0
  file:bg-blue-500 file:px-4 file:py-2 file:text-white
  file:hover:bg-blue-600
" />
```

## Range Slider

```html
<input type="range" class="
  w-full h-2 rounded-lg appearance-none cursor-pointer bg-gray-200
  accent-blue-500
" />
```

## Labels & Help Text

```html
<div class="space-y-1">
  <label class="block text-sm font-medium text-gray-700">
    Email address
    <span class="text-red-500">*</span>
  </label>
  <input type="email" class="w-full rounded-md border border-gray-300 px-3 py-2" />
  <p class="text-sm text-gray-500">We'll never share your email.</p>
</div>
```

## Error Messages

```html
<div class="space-y-1">
  <label class="block text-sm font-medium text-gray-700">Email</label>
  <input type="email" class="
    w-full rounded-md border border-red-500 px-3 py-2
    focus:border-red-500 focus:ring-1 focus:ring-red-500
  " />
  <p class="text-sm text-red-500">Please enter a valid email address.</p>
</div>
```

## Form Layout

### Stacked Form
```html
<form class="space-y-4">
  <div>
    <label class="block text-sm font-medium">Name</label>
    <input type="text" class="mt-1 w-full rounded-md border px-3 py-2" />
  </div>
  <div>
    <label class="block text-sm font-medium">Email</label>
    <input type="email" class="mt-1 w-full rounded-md border px-3 py-2" />
  </div>
  <button type="submit" class="w-full rounded-md bg-blue-500 py-2 text-white">
    Submit
  </button>
</form>
```

### Inline Form
```html
<form class="flex gap-2">
  <input type="email" class="flex-1 rounded-md border px-3 py-2" placeholder="Enter email" />
  <button type="submit" class="rounded-md bg-blue-500 px-4 py-2 text-white">
    Subscribe
  </button>
</form>
```

### Two-Column Form
```html
<form class="grid grid-cols-1 gap-4 md:grid-cols-2">
  <div>
    <label class="block text-sm font-medium">First name</label>
    <input type="text" class="mt-1 w-full rounded-md border px-3 py-2" />
  </div>
  <div>
    <label class="block text-sm font-medium">Last name</label>
    <input type="text" class="mt-1 w-full rounded-md border px-3 py-2" />
  </div>
  <div class="md:col-span-2">
    <label class="block text-sm font-medium">Email</label>
    <input type="email" class="mt-1 w-full rounded-md border px-3 py-2" />
  </div>
</form>
```

## Dark Mode Forms

```html
<input class="
  border border-gray-300 bg-white text-gray-900
  dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100
  dark:placeholder-gray-400
  focus:border-blue-500 dark:focus:border-blue-400
" />
```

## Fieldset & Legend

```html
<fieldset class="rounded-lg border border-gray-300 p-4">
  <legend class="px-2 text-sm font-medium">Personal Information</legend>
  <div class="space-y-4">
    <input type="text" placeholder="Name" class="w-full rounded border px-3 py-2" />
    <input type="email" placeholder="Email" class="w-full rounded border px-3 py-2" />
  </div>
</fieldset>
```
