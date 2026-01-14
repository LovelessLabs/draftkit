# Typography Utilities (Tailwind CSS v3)

## Font Family
```
font-sans  → font-family: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"
font-serif → font-family: ui-serif, Georgia, Cambria, "Times New Roman", Times, serif
font-mono  → font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace
```

## Font Size
```
text-xs   → font-size: 0.75rem (12px); line-height: 1rem
text-sm   → font-size: 0.875rem (14px); line-height: 1.25rem
text-base → font-size: 1rem (16px); line-height: 1.5rem
text-lg   → font-size: 1.125rem (18px); line-height: 1.75rem
text-xl   → font-size: 1.25rem (20px); line-height: 1.75rem
text-2xl  → font-size: 1.5rem (24px); line-height: 2rem
text-3xl  → font-size: 1.875rem (30px); line-height: 2.25rem
text-4xl  → font-size: 2.25rem (36px); line-height: 2.5rem
text-5xl  → font-size: 3rem (48px); line-height: 1
text-6xl  → font-size: 3.75rem (60px); line-height: 1
text-7xl  → font-size: 4.5rem (72px); line-height: 1
text-8xl  → font-size: 6rem (96px); line-height: 1
text-9xl  → font-size: 8rem (128px); line-height: 1
```

## Font Weight
```
font-thin       → font-weight: 100
font-extralight → font-weight: 200
font-light      → font-weight: 300
font-normal     → font-weight: 400
font-medium     → font-weight: 500
font-semibold   → font-weight: 600
font-bold       → font-weight: 700
font-extrabold  → font-weight: 800
font-black      → font-weight: 900
```

## Font Style
```
italic     → font-style: italic
not-italic → font-style: normal
```

## Line Height
```
leading-none    → line-height: 1
leading-tight   → line-height: 1.25
leading-snug    → line-height: 1.375
leading-normal  → line-height: 1.5
leading-relaxed → line-height: 1.625
leading-loose   → line-height: 2

# Fixed line heights
leading-3  → line-height: 0.75rem (12px)
leading-4  → line-height: 1rem (16px)
leading-5  → line-height: 1.25rem (20px)
leading-6  → line-height: 1.5rem (24px)
leading-7  → line-height: 1.75rem (28px)
leading-8  → line-height: 2rem (32px)
leading-9  → line-height: 2.25rem (36px)
leading-10 → line-height: 2.5rem (40px)
```

## Letter Spacing
```
tracking-tighter → letter-spacing: -0.05em
tracking-tight   → letter-spacing: -0.025em
tracking-normal  → letter-spacing: 0em
tracking-wide    → letter-spacing: 0.025em
tracking-wider   → letter-spacing: 0.05em
tracking-widest  → letter-spacing: 0.1em
```

## Text Alignment
```
text-left    → text-align: left
text-center  → text-align: center
text-right   → text-align: right
text-justify → text-align: justify
text-start   → text-align: start
text-end     → text-align: end
```

## Text Color

Uses the color palette with the `text-` prefix:
```
text-black       → color: #000
text-white       → color: #fff
text-gray-500    → color: #6b7280
text-red-500     → color: #ef4444
text-blue-500    → color: #3b82f6
text-inherit     → color: inherit
text-current     → color: currentColor
text-transparent → color: transparent
```

## Text Decoration
```
underline    → text-decoration-line: underline
overline     → text-decoration-line: overline
line-through → text-decoration-line: line-through
no-underline → text-decoration-line: none
```

## Text Decoration Style
```
decoration-solid  → text-decoration-style: solid
decoration-double → text-decoration-style: double
decoration-dotted → text-decoration-style: dotted
decoration-dashed → text-decoration-style: dashed
decoration-wavy   → text-decoration-style: wavy
```

## Text Decoration Thickness
```
decoration-auto      → text-decoration-thickness: auto
decoration-from-font → text-decoration-thickness: from-font
decoration-0         → text-decoration-thickness: 0px
decoration-1         → text-decoration-thickness: 1px
decoration-2         → text-decoration-thickness: 2px
decoration-4         → text-decoration-thickness: 4px
decoration-8         → text-decoration-thickness: 8px
```

## Text Underline Offset
```
underline-offset-auto → text-underline-offset: auto
underline-offset-0    → text-underline-offset: 0px
underline-offset-1    → text-underline-offset: 1px
underline-offset-2    → text-underline-offset: 2px
underline-offset-4    → text-underline-offset: 4px
underline-offset-8    → text-underline-offset: 8px
```

## Text Transform
```
uppercase  → text-transform: uppercase
lowercase  → text-transform: lowercase
capitalize → text-transform: capitalize
normal-case → text-transform: none
```

## Text Overflow
```
truncate        → overflow: hidden; text-overflow: ellipsis; white-space: nowrap
text-ellipsis   → text-overflow: ellipsis
text-clip       → text-overflow: clip
overflow-ellipsis → text-overflow: ellipsis (deprecated)
```

## Text Wrap (v3.3+)
```
text-wrap    → text-wrap: wrap
text-nowrap  → text-wrap: nowrap
text-balance → text-wrap: balance
text-pretty  → text-wrap: pretty
```

## Text Indent
```
indent-0   → text-indent: 0px
indent-px  → text-indent: 1px
indent-1   → text-indent: 0.25rem (4px)
indent-2   → text-indent: 0.5rem (8px)
indent-4   → text-indent: 1rem (16px)
indent-8   → text-indent: 2rem (32px)
```

## Vertical Align
```
align-baseline    → vertical-align: baseline
align-top         → vertical-align: top
align-middle      → vertical-align: middle
align-bottom      → vertical-align: bottom
align-text-top    → vertical-align: text-top
align-text-bottom → vertical-align: text-bottom
align-sub         → vertical-align: sub
align-super       → vertical-align: super
```

## Whitespace
```
whitespace-normal      → white-space: normal
whitespace-nowrap      → white-space: nowrap
whitespace-pre         → white-space: pre
whitespace-pre-line    → white-space: pre-line
whitespace-pre-wrap    → white-space: pre-wrap
whitespace-break-spaces → white-space: break-spaces
```

## Word Break
```
break-normal → overflow-wrap: normal; word-break: normal
break-words  → overflow-wrap: break-word
break-all    → word-break: break-all
break-keep   → word-break: keep-all
```

## Hyphens
```
hyphens-none   → hyphens: none
hyphens-manual → hyphens: manual
hyphens-auto   → hyphens: auto
```

## Font Smoothing
```
antialiased         → -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale
subpixel-antialiased → -webkit-font-smoothing: auto; -moz-osx-font-smoothing: auto
```

## Font Variant Numeric
```
normal-nums        → font-variant-numeric: normal
ordinal            → font-variant-numeric: ordinal
slashed-zero       → font-variant-numeric: slashed-zero
lining-nums        → font-variant-numeric: lining-nums
oldstyle-nums      → font-variant-numeric: oldstyle-nums
proportional-nums  → font-variant-numeric: proportional-nums
tabular-nums       → font-variant-numeric: tabular-nums
diagonal-fractions → font-variant-numeric: diagonal-fractions
stacked-fractions  → font-variant-numeric: stacked-fractions
```

## Line Clamp
```
line-clamp-1    → overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 1
line-clamp-2    → ... -webkit-line-clamp: 2
line-clamp-3    → ... -webkit-line-clamp: 3
line-clamp-4    → ... -webkit-line-clamp: 4
line-clamp-5    → ... -webkit-line-clamp: 5
line-clamp-6    → ... -webkit-line-clamp: 6
line-clamp-none → overflow: visible; display: block; -webkit-box-orient: horizontal; -webkit-line-clamp: none
```

## Common Patterns

### Heading Hierarchy
```html
<h1 class="text-4xl font-bold">Main Title</h1>
<h2 class="text-2xl font-semibold">Section Title</h2>
<h3 class="text-xl font-medium">Subsection</h3>
```

### Body Text
```html
<p class="text-base leading-relaxed text-gray-700">
  Well-spaced paragraph text for comfortable reading.
</p>
```

### Truncated Text
```html
<p class="truncate">
  This text will be truncated with an ellipsis...
</p>
```

### Multi-line Truncation
```html
<p class="line-clamp-3">
  This text will be truncated after 3 lines with an ellipsis...
</p>
```

### Link Styling
```html
<a class="text-blue-600 underline hover:text-blue-800">
  Clickable link
</a>
```

### Balanced Headings
```html
<h1 class="text-balance text-4xl font-bold">
  A Long Headline That Should Be Balanced Across Lines
</h1>
```

### Code Block
```html
<code class="font-mono text-sm bg-gray-100 px-1 rounded">
  code snippet
</code>
```
