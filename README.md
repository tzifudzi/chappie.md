# Chappie

Chappie is a collection of reusable AI agent instruction files. You write small markdown fragments (personality, user info, rules, tools), compose them into `.qmd` files using Quarto's `{{< include >}}` shortcode, and render self-contained `.md` files you can paste into any LLM system prompt.

## Usage

### Render all agents

Build every `.qmd` file under `agents/` and output the resulting `.md` files to `_output/`:

```bash
make render
```

Render a specific agent:

```bash
make render FILE=agents/coder/python/coder-python.qmd
```
