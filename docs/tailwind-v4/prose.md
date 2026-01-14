# Typography / Prose Styling (Tailwind CSS v4)

## Overview

The `@tailwindcss/typography` plugin provides `prose` classes for styling vanilla HTML content like Markdown output or CMS content. In v4, it continues to work the same way but uses CSS custom properties for theming.

## Installation

```bash
npm install -D @tailwindcss/typography
```

## Configuration

```css
/* app.css */
@import "tailwindcss";
@plugin "@tailwindcss/typography";
```

Or in JavaScript config:

```js
// tailwind.config.js
export default {
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
```

## Basic Usage

```html
<article class="prose lg:prose-xl">
  <h1>Article Title</h1>
  <p>Content rendered from Markdown or CMS...</p>
</article>
```

## Size Modifiers

| Class | Base Font Size |
|-------|---------------|
| `prose-sm` | 14px (0.875rem) |
| `prose` | 16px (1rem) |
| `prose-lg` | 18px (1.125rem) |
| `prose-xl` | 20px (1.25rem) |
| `prose-2xl` | 24px (1.5rem) |

```html
<!-- Responsive sizing -->
<article class="prose prose-sm md:prose-base lg:prose-lg">
  <!-- Scales with viewport -->
</article>
```

## Color Themes

| Class | Color Palette |
|-------|--------------|
| `prose-gray` | Default gray |
| `prose-slate` | Slate tones |
| `prose-zinc` | Zinc tones |
| `prose-neutral` | Neutral tones |
| `prose-stone` | Stone tones |

```html
<article class="prose prose-slate">
  <!-- Slate-colored typography -->
</article>
```

## Dark Mode

```html
<!-- Simple inversion -->
<article class="prose dark:prose-invert">
  <!-- Auto-inverts for dark mode -->
</article>

<!-- Custom dark styling -->
<article class="
  prose prose-slate
  dark:prose-invert
  prose-a:text-blue-600 dark:prose-a:text-blue-400
">
  <!-- Custom link colors in dark mode -->
</article>
```

## Element Modifiers

Target specific elements within prose:

| Modifier | Targets |
|----------|---------|
| `prose-headings:{utility}` | h1–h4, th |
| `prose-h1:{utility}` | h1 only |
| `prose-h2:{utility}` | h2 only |
| `prose-h3:{utility}` | h3 only |
| `prose-h4:{utility}` | h4 only |
| `prose-p:{utility}` | Paragraphs |
| `prose-a:{utility}` | Links |
| `prose-blockquote:{utility}` | Blockquotes |
| `prose-figure:{utility}` | Figures |
| `prose-figcaption:{utility}` | Figure captions |
| `prose-strong:{utility}` | Bold text |
| `prose-em:{utility}` | Italic text |
| `prose-code:{utility}` | Inline code |
| `prose-pre:{utility}` | Code blocks |
| `prose-ol:{utility}` | Ordered lists |
| `prose-ul:{utility}` | Unordered lists |
| `prose-li:{utility}` | List items |
| `prose-table:{utility}` | Tables |
| `prose-thead:{utility}` | Table headers |
| `prose-tr:{utility}` | Table rows |
| `prose-th:{utility}` | Header cells |
| `prose-td:{utility}` | Data cells |
| `prose-img:{utility}` | Images |
| `prose-video:{utility}` | Videos |
| `prose-hr:{utility}` | Horizontal rules |
| `prose-lead:{utility}` | Lead paragraph |

```html
<article class="
  prose
  prose-headings:font-bold prose-headings:tracking-tight
  prose-a:text-blue-600 prose-a:no-underline hover:prose-a:underline
  prose-code:rounded prose-code:bg-gray-100 prose-code:px-1
  prose-img:rounded-xl prose-img:shadow-md
">
  <!-- Customized styling -->
</article>
```

## Excluding Content

Use `not-prose` to escape prose styling:

```html
<article class="prose">
  <h1>Article Title</h1>
  <p>Styled paragraph.</p>

  <div class="not-prose">
    <!-- Custom component, not affected by prose -->
    <div class="flex gap-4 rounded-lg bg-blue-50 p-4">
      <button class="btn-primary">Action</button>
    </div>
  </div>

  <p>Back to prose styling.</p>
</article>
```

## Max Width Control

Prose defaults to `max-width: 65ch` for optimal readability:

```html
<!-- Remove max-width -->
<article class="prose max-w-none">
  <!-- Full width -->
</article>

<!-- Custom max-width -->
<article class="prose max-w-4xl">
  <!-- 4xl width -->
</article>
```

## Lead Paragraphs

Style the introductory paragraph differently:

```html
<article class="prose prose-lead:text-xl prose-lead:text-gray-500">
  <p>This first paragraph is the lead—larger and lighter.</p>
  <p>Regular paragraphs follow...</p>
</article>
```

## Common Patterns

### Blog Post

```html
<article class="prose prose-lg mx-auto max-w-3xl px-6 py-12">
  <header class="not-prose mb-8">
    <time class="text-sm text-gray-500">January 13, 2026</time>
    <h1 class="mt-2 text-4xl font-bold">Post Title</h1>
  </header>
  <!-- Rendered markdown content -->
</article>
```

### Documentation

```html
<div class="prose prose-slate max-w-none">
  <div class="prose-headings:scroll-mt-24">
    <!-- Full-width docs with scroll margin for fixed headers -->
  </div>
</div>
```

### CMS Content

```html
<div class="
  prose prose-sm md:prose-base
  prose-img:rounded-lg prose-img:shadow-sm
  prose-pre:bg-gray-900 prose-pre:text-gray-100
  prose-code:before:content-[''] prose-code:after:content-['']
">
  {{{ richTextContent }}}
</div>
```

### Prose in Cards

```html
<div class="rounded-xl border p-6">
  <div class="prose prose-sm">
    <h3>Card Title</h3>
    <p>Card content with prose styling...</p>
  </div>
</div>
```

## CSS Custom Properties (v4)

In v4, typography values use CSS custom properties:

```css
/* These are set by the plugin */
--tw-prose-body: var(--color-gray-700);
--tw-prose-headings: var(--color-gray-900);
--tw-prose-lead: var(--color-gray-600);
--tw-prose-links: var(--color-gray-900);
--tw-prose-bold: var(--color-gray-900);
--tw-prose-counters: var(--color-gray-500);
--tw-prose-bullets: var(--color-gray-300);
--tw-prose-hr: var(--color-gray-200);
--tw-prose-quotes: var(--color-gray-900);
--tw-prose-quote-borders: var(--color-gray-200);
--tw-prose-captions: var(--color-gray-500);
--tw-prose-code: var(--color-gray-900);
--tw-prose-pre-code: var(--color-gray-200);
--tw-prose-pre-bg: var(--color-gray-800);
--tw-prose-th-borders: var(--color-gray-300);
--tw-prose-td-borders: var(--color-gray-200);

/* Inverted for dark mode */
--tw-prose-invert-body: var(--color-gray-300);
--tw-prose-invert-headings: var(--color-white);
/* ... etc */
```

You can override these directly:

```html
<article class="prose" style="--tw-prose-links: var(--color-blue-600);">
  <!-- Custom link color via CSS variable -->
</article>
```

## Customizing in CSS (v4)

```css
@layer components {
  .prose-brand {
    --tw-prose-links: var(--color-brand-600);
    --tw-prose-headings: var(--color-brand-900);
  }
}
```

```html
<article class="prose prose-brand">
  <!-- Brand-colored prose -->
</article>
```

## v3 Compatibility

The `@tailwindcss/typography` plugin API is the same in v3 and v4. The main difference is v4's use of CSS custom properties for theming, making runtime customization easier.
