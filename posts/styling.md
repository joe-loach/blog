---
title: Supported Markdown Syntax
date: 2025-04-13
---

# Supported Markdown Syntax

## Introduction {#intro}

I generate my blog posts from markdown using `pulldown-cmark` server-side.
Here is a (non-exhaustive) list of all of the syntax that I am able to use, styled by tailwindcss.

Sentences are just normal `<p>` elements.

---

## Lists

1. Ordered
2. List

* Bulleted
* List

- [x] Check boxes
- [ ] Meaning of life

---

## Text

**bold text**

*italics*

~~strikethrough~~

[links](https://www.example.com)

Footnotes [^1]

[^1]: Extra information here.

---

## Blocks

> blockquote, something profound

`a block of code`

```
a longer block of code
```

---

## Images

![an image of a dog](https://picsum.photos/id/237/200/300)
