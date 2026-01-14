# @tailwindcss/forms Plugin (Tailwind CSS v3)

## Overview

The `@tailwindcss/forms` plugin provides a basic reset for form styles that makes form elements easy to override with utilities. Without this plugin, form elements like inputs, selects, and checkboxes retain browser-specific styling that's difficult to customize.

## Installation

```bash
npm install -D @tailwindcss/forms
```

## Configuration

```js
// tailwind.config.js
module.exports = {
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
```

### Strategy Options

```js
// tailwind.config.js
module.exports = {
  plugins: [
    require('@tailwindcss/forms')({
      strategy: 'base', // default: applies to all form elements
      // strategy: 'class', // only applies when using form-* classes
    }),
  ],
}
```

| Strategy | Description |
|----------|-------------|
| `base` (default) | Applies reset styles to all form elements globally |
| `class` | Only applies styles when using explicit `form-*` classes |

## Base Strategy (Default)

With `strategy: 'base'`, all form elements are automatically styled:

```html
<!-- All inputs get the reset styles automatically -->
<input type="text" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
```

## Class Strategy

With `strategy: 'class'`, you must add explicit classes:

| Class | Applies To |
|-------|------------|
| `form-input` | `input[type='text']`, `input[type='password']`, `input[type='email']`, etc. |
| `form-textarea` | `textarea` |
| `form-select` | `select` |
| `form-multiselect` | `select[multiple]` |
| `form-checkbox` | `input[type='checkbox']` |
| `form-radio` | `input[type='radio']` |

```html
<!-- Must use form-* classes explicitly -->
<input type="text" class="form-input mt-1 block w-full rounded-md border-gray-300">
<select class="form-select mt-1 block w-full rounded-md border-gray-300">
  <option>Option 1</option>
</select>
<input type="checkbox" class="form-checkbox rounded text-indigo-600">
```

## Styled Form Elements

### Text Input

```html
<input type="text" class="
  block w-full rounded-md border-gray-300 shadow-sm
  focus:border-indigo-500 focus:ring-indigo-500
  sm:text-sm
">
```

### Select

```html
<select class="
  block w-full rounded-md border-gray-300 shadow-sm
  focus:border-indigo-500 focus:ring-indigo-500
  sm:text-sm
">
  <option>Option 1</option>
  <option>Option 2</option>
</select>
```

### Textarea

```html
<textarea rows="4" class="
  block w-full rounded-md border-gray-300 shadow-sm
  focus:border-indigo-500 focus:ring-indigo-500
  sm:text-sm
"></textarea>
```

### Checkbox

```html
<input type="checkbox" class="
  h-4 w-4 rounded border-gray-300 text-indigo-600
  focus:ring-indigo-500
">
```

### Radio

```html
<input type="radio" class="
  h-4 w-4 border-gray-300 text-indigo-600
  focus:ring-indigo-500
">
```

## Focus Ring Customization

The plugin uses Tailwind's `ring` utilities for focus states:

```html
<!-- Default focus ring -->
<input class="focus:ring focus:ring-indigo-200 focus:ring-opacity-50">

<!-- Custom focus ring -->
<input class="focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">

<!-- No focus ring (use border instead) -->
<input class="focus:ring-0 focus:border-indigo-500">
```

## Disabled & Read-only States

```html
<!-- Disabled -->
<input type="text" disabled class="
  bg-gray-100 cursor-not-allowed opacity-50
">

<!-- Read-only -->
<input type="text" readonly class="bg-gray-50">
```

## Custom Checkbox Pattern

```html
<label class="inline-flex items-center">
  <input type="checkbox" class="
    rounded border-gray-300 text-indigo-600 shadow-sm
    focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50
  ">
  <span class="ml-2">Remember me</span>
</label>
```

## Custom Radio Group

```html
<div class="space-y-2">
  <label class="inline-flex items-center">
    <input type="radio" name="option" value="1" class="
      border-gray-300 text-indigo-600
      focus:ring-indigo-500
    ">
    <span class="ml-2">Option 1</span>
  </label>
  <label class="inline-flex items-center">
    <input type="radio" name="option" value="2" class="
      border-gray-300 text-indigo-600
      focus:ring-indigo-500
    ">
    <span class="ml-2">Option 2</span>
  </label>
</div>
```

## Toggle Switch Pattern

```html
<button type="button"
  class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent bg-gray-200 transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
  role="switch"
  aria-checked="false"
>
  <span class="translate-x-0 pointer-events-none relative inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out">
    <!-- Icons for on/off states -->
  </span>
</button>
```

## Form Layout Patterns

### Stacked Form

```html
<form class="space-y-6">
  <div>
    <label class="block text-sm font-medium text-gray-700">Email</label>
    <input type="email" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
  </div>
  <div>
    <label class="block text-sm font-medium text-gray-700">Password</label>
    <input type="password" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
  </div>
</form>
```

### Inline Form

```html
<form class="flex items-end space-x-4">
  <div class="flex-1">
    <label class="block text-sm font-medium text-gray-700">Email</label>
    <input type="email" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
  </div>
  <button type="submit" class="rounded-md bg-indigo-600 px-4 py-2 text-white">
    Subscribe
  </button>
</form>
```

## Dark Mode Support

```html
<input type="text" class="
  border-gray-300 bg-white text-gray-900
  dark:border-gray-600 dark:bg-gray-700 dark:text-white
  focus:border-indigo-500 focus:ring-indigo-500
  dark:focus:border-indigo-400 dark:focus:ring-indigo-400
">
```

## v3 to v4 Migration Notes

| v3 (@tailwindcss/forms plugin) | v4 (built-in) |
|-------------------------------|---------------|
| Requires plugin installation | No plugin needed |
| `form-input`, `form-select` classes | Use utilities directly |
| `strategy: 'base'` or `'class'` | No strategy concept |
| `ring` utilities for focus | Same `ring` utilities |
| `text-{color}-600` for checkbox color | `accent-{color}` also available |

In v4, form elements work with utilities out of the box. The `@tailwindcss/forms` plugin is primarily needed for v3 projects.
