# Chappie — Agent Instructions

This repository is a collection of reusable agent instruction files organized by use case. It uses **Quarto** to compose self-contained markdown files per agent from layered fragments.

## How It Works

### Fragment-Based Composition

Content is authored in **fragment files** (small `.md` files with no headings), then composed into final agent documents using Quarto's `{{< include >}}` shortcode.

```
Fragment files = content without structure
Composition files (.qmd) = structure without content
```

### Inheritance Hierarchy

Each agent inherits from up to 3 levels:

1. **Global** (`_globals/`) — shared by all agents (soul, identity, user, bootstrap, tools, rules)
2. **Category** (`agents/<category>/_sections/`) — shared by all agents in a category (e.g., all coders)
3. **Specialization** (`agents/<category>/<spec>/_sections/`) — specific to one agent variant (e.g., Python coder)

### Standard Sections

Every agent document has these sections (in order):

| Section       | Purpose                                                  | Layered? |
|---------------|----------------------------------------------------------|----------|
| **Soul**      | Core personality, values, boundaries, vibe               | Yes      |
| **Identity**  | Name, creature type, vibe, emoji, avatar                 | No       |
| **User**      | Info about the human                                     | No       |
| **Bootstrap** | First-run conversation script                            | Yes      |
| **Tools**     | Available tools and capabilities                         | Yes      |
| **Rules**     | Behavioral rules, constraints, standards                 | Yes      |

**Layered = Yes** means fragments from multiple levels merge under one heading.

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
- **Software Architect** — Software architecture and design
  - [`agents/software-architect/software-architect.qmd`](agents/software-architect/software-architect.qmd)

## Directory Structure

```
_globals/                          # Global fragments (shared by all agents)
  SOUL.md
  IDENTITY.md
  USER.md
  BOOTSTRAP.md
  TOOLS.md
  RULES.md

agents/
  <category>/
    _sections/                     # Category-level fragments
      SOUL.md, BOOTSTRAP.md, TOOLS.md, RULES.md
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
