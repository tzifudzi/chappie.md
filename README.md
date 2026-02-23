# Chappie

Chappie is a collection of reusable AI agent instruction files. You write small markdown fragments (personality, user info, rules, tools), compose them into `.qmd` files using Quarto's `{{< include >}}` shortcode, and render self-contained `.md` files you can paste into any LLM system prompt.

The project has two modules:

- **`render/`** — Quarto-based composition that generates agent `.md` files from layered fragments.
- **`apply/`** — Rust CLI that copies rendered `.md` files to tool-specific locations (Claude Code, Roo Code, Codex, etc.).

## Usage

### Render all agents

Build every `.qmd` file under `render/agents/` and output the resulting `.md` files to `render/_output/`:

```bash
cd render
make render
```

Render a specific agent:

```bash
cd render
make render FILE=agents/coder/python/coder-python.qmd
```

### Apply rendered agents

Copy rendered `.md` files to their target locations:

```bash
cd apply
cargo run --manifest-path cli/Cargo.toml -- local
```
