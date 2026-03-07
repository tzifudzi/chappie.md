<!-- Coder-level rules -->

### Coding Style

#### Inline Comments

Add brief explanatory comments where logic is non-obvious.

```rust
// Cap refund at 98% of original payment (Amazon.ca usually charges a small shipping fee on refunds)
let max_refund = original_payment * 0.90;
let refund_amount = requested_refund.min(max_refund);
```

#### Prefer Swappable Implementations

Prefer designs that allow swapping implementations instead of hardcoding one concrete dependency everywhere. For example, inject a URL shortener interface/trait rather than directly coupling all call sites to one provider. This is a preference, not a hard requirement. 

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

#### Bug Fixes — Test First

When fixing a bug, use a TDD approach: write a failing test that reproduces the bug *before* writing the fix. Run the test to confirm it fails for the expected reason, then implement the fix and verify the test passes. This prevents fixing the wrong thing — if the test doesn't fail, your understanding of the bug may be wrong and you should reassess before touching production code.

#### Test Philosophy

Write minimal, focused tests that verify core behavior without excessive edge-case coverage.

#### Test Naming Convention

Use `given_xxx_when_xxx_then_xxx` naming. Long, descriptive method names are fine — clarity matters more than brevity. Add `// Given:`, `// When:`, and `// Expect:` section comments inside the test body.

```rust
// ❌ Bad — vague, no structure
fn test_write();

// ✅ Good — descriptive given/when/then
fn given_empty_dir_when_write_called_then_creates_file();
```

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

#### Approval Before Writing Tests

Before implementing unit tests, summarize which tests you plan to add and ask Tatenda for approval. List the test names or scenarios at a high level — do not include implementation details. This avoids wasting tokens on tests the human may not want.

Example — after implementing a `Notifier` module:

```
❌ Bad: "I'll write a test that creates a mock SmtpClient, injects it into
   Notifier::new(), calls send(), and asserts the mock received exactly
   one call with subject 'Hello'."
✅ Good: "I'll add these tests for Notifier:
   - given_valid_recipient_when_send_then_succeeds
   - given_empty_recipient_when_send_then_fails
   - given_transient_error_when_send_then_retries
   OK to proceed?"
```

### Git

#### Commit Messages

Write plain, human-readable commit messages. Do not use conventional commit format (`feat(scope):`, `fix:`, etc.). Start with an imperative verb, capitalize the first word, and keep it concise.

```
❌ Bad: "feat(instacart): add 1.00 authorization hold pre-match rule"
✅ Good: "Add $1.00 hold pre-match rule"
```

### Workflow

#### Approval Before Coding

Before writing or modifying any code, briefly explain the high-level plan and explicitly ask Tatenda for approval to proceed. Do not ask about low-level details (specific lines, variable names, etc.) — summarize the overall approach in a few sentences and ask for permission to proceed. If
you proceed without permission, you risk spending tokens on an implementation plan the human does not agree with.

Example — adding email notification support across multiple files:

```
❌ Bad: "Should I rename the `notify` variable on line 42 of handler.rs?"
✅ Good: "I'll use the decorator pattern to wrap the existing Notifier with
   an EmailNotifier that adds a send() method. This touches handler.rs,
   notifier.rs, and the config module. OK to proceed?"
```

#### Unanswered Questions — Do Not Proceed

If you asked multiple required questions, do not proceed until all are answered. If even one required answer is missing, ask for the missing answer or ask explicit permission to proceed with partial information.

Example:

```
You asked 3 required questions and Tatenda answered only 2.
Do not start implementation.
Ask for the remaining answer, or ask: "Do you want me to proceed with the missing detail unresolved?"
```

#### Costly Fixes — Escalate, Don't Decide

When the proper fix for an issue requires significant effort (large refactor, new abstraction, cross-cutting changes), do not silently commit to that effort. Describe the problem, explain why the fix is non-trivial, and let Tatenda decide whether to proceed, defer, or take a different approach. Quick fixes are fine to act on; expensive ones need human sign-off.

#### Competing Approaches — Ask, Don't Choose

When two or more implementation approaches are similarly viable, summarize them briefly and ask Tatenda to pick. For each option, include exactly one advantage. When there is one obvious approach, proceed without asking.

```
"Two ways to implement the custom discount logic:

1. Decorator pattern — wrap the existing PriceCalculator.
   - Advantage: keeps existing constructor signatures unchanged.

2. Strategy pattern — inject a DiscountStrategy at construction.
   - Advantage: makes discount behavior explicit and easy to test.

Both fit here. Which do you prefer?"
```

#### Refactoring

Always minimize token usage. Prefer vscode-mcp-server for refactoring and automation to reduce context size and redundancy. If vscode-mcp-server is unsuitable, fall back to PowerShell utilities. If vscode-mcp-server is not running, immediately alert the user to fix it — it is critical for use.

#### Plans

When creating plans, place them in the `.agents/plans/` folder at the project root — it is git-ignored and won't be committed.

Start planning at a high level first. This is critical: diving into low-level implementation details too early usually wastes tokens on plans that may change after alignment.

#### Self-Improving Documentation

Tatenda uses a hierarchical `AGENTS.md` structure with multiple files across the project. If you get stuck searching for something and think an `AGENTS.md` could have helped, update or create one with the detail you needed. Likewise, if misleading content in an `AGENTS.md` sent you down the wrong path, fix or remove it.

#### Post-Task Reflection

After completing a coding task, include a brief reflection: summarize one challenge you experienced while navigating the codebase or problem solving, and what could be improved (in the codebase, documentation, tooling, or your own approach). Keep it to 2–3 sentences. The goal is continuous improvement — surfacing friction points while they're fresh. If the improvement is actionable (e.g., a misleadingly named variable, missing documentation), suggest the specific fix.
