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

1. **Global** (`_globals/`) — shared by all agents (soul, user, tools, rules)
2. **Category** (`agents/<category>/_sections/`) — shared by all agents in a category (e.g., all coders)
3. **Specialization** (`agents/<category>/<spec>/_sections/`) — specific to one agent variant (e.g., Python coder)

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
  - [`agents/chat/chat.qmd`](agents/chat/chat.qmd)
- **Coder** — General coding guidelines and practices
  - [`agents/coder/coder.qmd`](agents/coder/coder.qmd)
  - **Python** — Python-specific coding instructions
    - [`agents/coder/python/coder-python.qmd`](agents/coder/python/coder-python.qmd)
  - **Rust** — Rust-specific coding instructions
    - [`agents/coder/rust/coder-rust.qmd`](agents/coder/rust/coder-rust.qmd)
- **Obsidian** — Obsidian note-taking workflows
  - [`agents/obsidian/obsidian.qmd`](agents/obsidian/obsidian.qmd)

## Directory Structure

```
_globals/                          # Global fragments (shared by all agents)
  SOUL.md
  USER.md
  TOOLS.md
  RULES.md

agents/
  <category>/
    _sections/                     # Category-level fragments
      SOUL.md, TOOLS.md, RULES.md
    <specialization>/
      _sections/                   # Specialization-level fragments
        TOOLS.md, RULES.md
      <name>.qmd                   # Composition file → renders to final .md
    <name>.qmd                     # Category-level composition file

_output/                           # Rendered output (gitignored)
  chat.md, coder.md, coder-python.md, ...
```

## Building

```bash
# Render all agents to _output/
make render

# Clean output
make clean
```

## Adding a New Agent

1. Create a directory under `agents/` (e.g., `agents/writer/`)
2. Create `agents/writer/_sections/` with fragment files (`TOOLS.md`, `RULES.md`)
3. Create `agents/writer/writer.qmd` using the composition pattern (see existing `.qmd` files for examples)
4. Run `make render` to build
