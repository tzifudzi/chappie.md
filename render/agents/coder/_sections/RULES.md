<!-- Coder-level rules -->

### Coding Style

#### Inline Comments

Add brief explanatory comments where logic is non-obvious.

```rust
// Cap refund at 98% of original payment (Amazon.ca usually charges a small shipping fee on refunds)
let max_refund = original_payment * 0.90;
let refund_amount = requested_refund.min(max_refund);
```

#### Prefer Region Markers Over Decorative Comments

When grouping code sections, use IDE-supported region markers instead of decorative comment separators. Regions are collapsible in most editors and provide better navigation.

```python
# ❌ Bad
# =========
# Helpers
# =========

# ❌ Also bad
# -------- helpers --------

# ✅ Good — IDE-collapsible region
# region Helpers
def helper_one():
    pass

def helper_two():
    pass
# endregion
```

```rust
// ❌ Bad
// =========
// Helpers
// =========

// ✅ Good — region marker
// region: Helpers
fn helper_one() {}

fn helper_two() {}
// endregion
```

```typescript
// ❌ Bad
// =========
// Helpers
// =========

// ✅ Good — region marker
// #region Helpers
function helperOne() {}

function helperTwo() {}
// #endregion
```

### Testing

- A task is **not done** if tests fail. All tests must pass before considering any task complete.
- When you make a change to a section, only run tests for that section — not the entire test suite.
- If attempts to fix failing tests continually fail and you feel stuck, ask the human for permission to stop the task rather than continuing to spin.

#### Test Philosophy

Write minimal, focused tests that verify core behavior without excessive edge-case coverage.

#### Test Naming Convention

Use `given_xxx_when_xxx_then_xxx` naming. Add `// Given:`, `// When:`, and `// Expect:` section comments inside the test body.

```rust
#[test]
fn given_empty_dir_when_write_called_then_creates_file() {
    // Given: a writer with an empty temp directory
    let dir = tempfile::tempdir().unwrap();
    let writer = Writer::new(dir.path());

    // When: write is called with content
    writer.write("hello\n");

    // Expect: file is created with exact content
    let contents = std::fs::read_to_string(dir.path().join("out.log")).unwrap();
    assert_eq!(contents, "hello\n");
}
```

### Workflow

#### Refactoring

Always minimize token usage. Prefer vscode-mcp-server for refactoring and automation to reduce context size and redundancy. If vscode-mcp-server is unsuitable, fall back to PowerShell utilities. If vscode-mcp-server is not running, immediately alert the user to fix it — it is critical for use.

#### Plans

When creating plans, place them in the `plans/` folder — it is git-ignored and won't be committed.

#### Self-Improving Documentation

Tatenda uses a hierarchical `AGENTS.md` structure with multiple files across the project. If you get stuck searching for something and think an `AGENTS.md` could have helped, update or create one with the detail you needed. Likewise, if misleading content in an `AGENTS.md` sent you down the wrong path, fix or remove it.
