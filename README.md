# Chappie

Chappie is a collection of reusable AI agent instruction files. You write small markdown fragments (personality, user info, rules, tools), compose them into `.qmd` files using Quarto's `{{< include >}}` shortcode, and render self-contained `.md` files you can paste into any LLM system prompt.

## Examples

Generate all agent instrunction files

```bash
quarto render agents/coder/coder.qmd
```

Generate a single agent instrunction file

```bash
quarto render agents/coder/coder.qmd
```
