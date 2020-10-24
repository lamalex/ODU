# CS 722 Semester Capstone

## Presentation
Paper summary presentation slide deck is written using [Marp](https://marp.app).
Presentation available [here](presentation/README.md).

[VS Code](https://code.visualstudio.com) with the [Marp plugin](https://marketplace.visualstudio.com/items?itemName=marp-team.marp-vscode)
is recommended workflow for authoring.

### Markdown
Markdown is simple, and easy to learn. A quick intro is available at [CommonMark](https://commonmark.org).

### Gotchas
- Image urls are wonky:
  - marp compiles `index.html` to the root, but we are writing the slides in `/presentation`. When developing slides it makes sense to use the relative path `images/<whatever>.<ext>`, but this path breaks on deployment.
  - The path at deploy time needs to be `presentation/images/<whatever>.<ext>`
  - Possible workarounds:
    - use a commit hook to rewrite urls at commit time
    - rewrite image urls at build time
      - 👆 this is probably the right choice

## Code
* `todo`: Python project inside of project 
