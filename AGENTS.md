# Chappie — Agent Instructions

This repository is a collection of reusable agent instruction files organized by use case. It uses **Quarto** to compose self-contained markdown files per agent from layered fragments.

## How It Works

### Fragment-Based Composition

Content is authored in **fragment files** (small `.md` files), then composed into final agent documents using Quarto's `{{< include >}}` shortcode.

```
Fragment files = content without top-level structure
Composition files (.qmd) = top-level structure (## headings) without content
```

Category and specialization fragments should use `###` subsections to group related items (e.g., `### Coding Background`, `### Coding Preferences`). Top-level `##` headings are reserved for the composition files.

### Inheritance Hierarchy

Each agent inherits from up to 3 levels:

1. **Global** (`render/_globals/`) — shared by all agents (soul, user, tools, rules)
2. **Category** (`render/agents/<category>/_sections/`) — shared by all agents in a category (e.g., all coders)
3. **Specialization** (`render/agents/<category>/<spec>/_sections/`) — specific to one agent variant (e.g., Python coder)

### Standard Sections

Every agent document has these sections (in order):

| Section       | Purpose                                                  | Layered? |
|---------------|----------------------------------------------------------|----------|
| **Soul**      | Core personality, values, boundaries, vibe               | Yes      |
| **User**      | Info about the human                                     | No       |
| **Tools**     | Available tools and capabilities                         | Yes      |
| **Rules**     | Behavioral rules, constraints, standards                 | Yes      |

**Layered = Yes** means fragments from multiple levels merge under one heading.

### Soul Point Format

Each point in a `SOUL.md` file follows a two-line pattern:

```
Trait, brief description.
- Detailed explanation expanding on the trait.
```

- **Line 1**: The trait name followed by a comma and a short plain-text description (no bold, no em dashes).
- **Line 2**: A bullet (`-`) with the full behavioral explanation.
- Separate each point with a blank line.

Example:

```
Opinionated, not a search engine.
- You have opinions. You disagree, have preferences, find things amusing or boring. You're not a search engine.
```

## Available Agents

- **Chat** — General conversational chat
  - [`render/agents/chat/chat.qmd`](render/agents/chat/chat.qmd)
- **Coder** — General coding guidelines and practices
  - [`render/agents/coder/coder.qmd`](render/agents/coder/coder.qmd)
  - **Academic** — Source-grounded coding guidance for academic work
    - [`render/agents/coder/academic/coder-academic.qmd`](render/agents/coder/academic/coder-academic.qmd)
  - **Python** — Python-specific coding instructions
    - [`render/agents/coder/python/coder-python.qmd`](render/agents/coder/python/coder-python.qmd)
  - **Rust** — Rust-specific coding instructions
    - [`render/agents/coder/rust/coder-rust.qmd`](render/agents/coder/rust/coder-rust.qmd)
- **Obsidian** — Obsidian note-taking workflows
  - [`render/agents/obsidian/obsidian.qmd`](render/agents/obsidian/obsidian.qmd)
- **Investing** — Personal investing preferences and guidance
  - [`render/agents/investing/investing.qmd`](render/agents/investing/investing.qmd)

## Directory Structure

```
render/                            # Generates agent .md files from fragments
  _quarto.yml
  Makefile
  _globals/                        # Global fragments (shared by all agents)
    SOUL.md, USER.md, TOOLS.md, RULES.md
  agents/
    <category>/
      _sections/                   # Category-level fragments
        SOUL.md, TOOLS.md, RULES.md
      <specialization>/
        _sections/                 # Specialization-level fragments
          TOOLS.md, RULES.md
        <name>.qmd                 # Composition file → renders to final .md
      <name>.qmd                   # Category-level composition file
  scripts/
    render.py, clean.py
  _output/                         # Rendered output (gitignored)
    chat.md, coder.md, coder-python.md, ...

apply/                             # Copies rendered .md to tool-specific locations
  chappie.toml                     # Apply configuration
  cli/                             # Rust CLI
    Cargo.toml
    src/
```

## Building

```bash
# Render all agents to render/_output/
cd render
make render

# Clean output
make clean

# Apply rendered agents to local targets
cd ../apply
cargo run --manifest-path cli/Cargo.toml -- local
```

## Writing Style

Write statements that are concise but detailed. Every token in the rendered output costs context — less context means higher model accuracy. Cut filler and redundancy, but don't sacrifice clarity or precision.

```
❌ Bloated: "When you are working on fixing a bug, it is very important
   that you make sure to always write a test before you start working
   on the actual fix itself, so that you can be confident that the bug
   is actually reproduced and that your fix addresses it correctly."
```

Short and concise:
```
"Run cargo check after every change to catch compilation errors early."
```

Long and concise — every word necessary:
```
"When fixing a bug, write a failing test before the fix. Run it to
confirm it fails for the expected reason, then implement the fix and
verify the test passes. This prevents fixing the wrong thing — if the
test doesn't fail, your understanding of the bug may be wrong and you
should reassess before touching production code."
```

## Adding a New Agent

1. Create a directory under `render/agents/` (e.g., `render/agents/writer/`)
2. Create `render/agents/writer/_sections/` with fragment files (`TOOLS.md`, `RULES.md`)
3. Create `render/agents/writer/writer.qmd` using the composition pattern (see existing `.qmd` files for examples)
4. Run `cd render && make render` to build
