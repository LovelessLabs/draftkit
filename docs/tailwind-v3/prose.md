# @tailwindcss/typography Plugin (Tailwind CSS v3)

## Overview

The `@tailwindcss/typography` plugin provides a set of `prose` classes you can use to add beautiful typographic defaults to any vanilla HTML you don't control, like HTML rendered from Markdown or pulled from a CMS.

## Installation

```bash
npm install -D @tailwindcss/typography
```

## Configuration

```js
// tailwind.config.js
module.exports = {
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
```

## Basic Usage

```html
<article class="prose lg:prose-xl">
  <h1>Garlic bread with cheese: What the science tells us</h1>
  <p>
    For years parents have espoused the health benefits of eating garlic bread
    with cheese to their children, with the food earning such an iconic status
    in our culture that kids will often dress up as warm, cheesy loaf for Halloween.
  </p>
  <!-- More content... -->
</article>
```

## Size Modifiers

| Class | Description |
|-------|-------------|
| `prose-sm` | Smaller typography (14px base) |
| `prose` | Default size (16px base) |
| `prose-lg` | Larger typography (18px base) |
| `prose-xl` | Extra large typography (20px base) |
| `prose-2xl` | 2x large typography (24px base) |

```html
<!-- Responsive sizing -->
<article class="prose prose-sm md:prose-base lg:prose-lg xl:prose-xl">
  <!-- Content -->
</article>
```

## Color Modifiers

| Class | Description |
|-------|-------------|
| `prose-gray` | Gray color scheme (default) |
| `prose-slate` | Slate color scheme |
| `prose-zinc` | Zinc color scheme |
| `prose-neutral` | Neutral color scheme |
| `prose-stone` | Stone color scheme |

```html
<article class="prose prose-slate">
  <!-- Content with slate color scheme -->
</article>
```

## Styled Elements

The plugin styles these HTML elements:

### Headings
- `h1`, `h2`, `h3`, `h4` with proper sizing and spacing

### Text
- `p` paragraphs with appropriate line height
- `strong` bold text
- `em` italic text
- `a` links with underlines and hover states

### Lists
- `ul` unordered lists with bullets
- `ol` ordered lists with numbers
- `li` list items with proper spacing

### Code
- `code` inline code with background
- `pre` code blocks with syntax styling

### Blockquotes
- `blockquote` with left border and italic styling

### Tables
- `table`, `thead`, `tbody`, `tr`, `th`, `td`

### Media
- `img` images with proper margins
- `figure` and `figcaption`
- `video` embedded videos

### Horizontal Rule
- `hr` with subtle styling

## Undoing Typography Styles

Use `not-prose` to exclude sections from prose styling:

```html
<article class="prose">
  <h1>My Heading</h1>
  <p>This paragraph is styled by prose.</p>

  <div class="not-prose">
    <!-- This content is NOT styled by prose -->
    <button class="btn">Custom Button</button>
  </div>

  <p>Back to prose styling.</p>
</article>
```

## Element Modifiers

Override default styles for specific elements:

```html
<article class="prose prose-headings:underline prose-a:text-blue-600">
  <!-- Headings are underlined, links are blue -->
</article>
```

| Modifier | Target |
|----------|--------|
| `prose-headings:{utility}` | h1, h2, h3, h4, th |
| `prose-h1:{utility}` | h1 |
| `prose-h2:{utility}` | h2 |
| `prose-h3:{utility}` | h3 |
| `prose-h4:{utility}` | h4 |
| `prose-p:{utility}` | p |
| `prose-a:{utility}` | a |
| `prose-blockquote:{utility}` | blockquote |
| `prose-figure:{utility}` | figure |
| `prose-figcaption:{utility}` | figcaption |
| `prose-strong:{utility}` | strong |
| `prose-em:{utility}` | em |
| `prose-code:{utility}` | code |
| `prose-pre:{utility}` | pre |
| `prose-ol:{utility}` | ol |
| `prose-ul:{utility}` | ul |
| `prose-li:{utility}` | li |
| `prose-table:{utility}` | table |
| `prose-thead:{utility}` | thead |
| `prose-tr:{utility}` | tr |
| `prose-th:{utility}` | th |
| `prose-td:{utility}` | td |
| `prose-img:{utility}` | img |
| `prose-video:{utility}` | video |
| `prose-hr:{utility}` | hr |
| `prose-lead:{utility}` | First paragraph (lead text) |

```html
<article class="
  prose
  prose-headings:font-semibold
  prose-h1:text-3xl
  prose-a:text-blue-600 prose-a:no-underline hover:prose-a:underline
  prose-img:rounded-xl prose-img:shadow-lg
  prose-code:text-pink-600 prose-code:before:content-none prose-code:after:content-none
">
  <!-- Customized prose content -->
</article>
```

## Dark Mode

```html
<article class="prose dark:prose-invert">
  <!-- Light mode with dark mode support -->
</article>
```

Or with custom dark mode colors:

```html
<article class="
  prose prose-slate
  dark:prose-invert
  prose-headings:text-gray-900 dark:prose-headings:text-white
  prose-a:text-blue-600 dark:prose-a:text-blue-400
">
  <!-- Content -->
</article>
```

## Customizing in Config

```js
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            color: '#333',
            a: {
              color: '#3182ce',
              '&:hover': {
                color: '#2c5282',
              },
            },
          },
        },
      },
    },
  },
  plugins: [require('@tailwindcss/typography')],
}
```

## Max Width

By default, prose has a max-width of 65ch for readability:

```html
<!-- Remove max-width constraint -->
<article class="prose max-w-none">
  <!-- Full-width prose content -->
</article>

<!-- Custom max-width -->
<article class="prose max-w-3xl">
  <!-- Content with 3xl max-width -->
</article>
```

## Lead Paragraph

The first paragraph can be styled as lead text:

```html
<article class="prose">
  <p class="lead">
    This is a lead paragraph with larger, lighter text that introduces the article.
  </p>
  <p>Regular paragraph text follows...</p>
</article>
```

Or target it with the modifier:

```html
<article class="prose prose-lead:text-xl prose-lead:text-gray-500">
  <!-- First paragraph gets lead styling -->
</article>
```

## Common Patterns

### Blog Post Layout

```html
<article class="prose prose-lg mx-auto max-w-3xl px-4 py-8">
  <h1>Article Title</h1>
  <p class="lead">Introduction paragraph...</p>
  <!-- Content -->
</article>
```

### Documentation Page

```html
<div class="prose prose-slate max-w-none">
  <!-- Full-width documentation content -->
</div>
```

### CMS Content with Custom Styling

```html
<div class="
  prose prose-sm md:prose-base
  prose-headings:scroll-mt-20
  prose-a:text-primary prose-a:no-underline hover:prose-a:underline
  prose-code:rounded prose-code:bg-gray-100 prose-code:px-1
  prose-pre:bg-gray-900
">
  {{{ cmsContent }}}
</div>
```

## v3 to v4 Notes

The `@tailwindcss/typography` plugin works similarly in both v3 and v4. The main difference is that v4 uses CSS custom properties internally, but the API remains the same.
