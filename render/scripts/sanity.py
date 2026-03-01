"""Verify all expected agent outputs exist and are non-empty after rendering."""

from pathlib import Path

AGENTS_DIR = Path("agents")
OUTPUT_DIR = Path("_out/md")


def discover_expected_files() -> set[str]:
    """Derive expected .md filenames from .qmd source files."""
    return {qmd.stem + ".md" for qmd in AGENTS_DIR.rglob("*.qmd")}


def test_all_outputs_exist() -> None:
    expected = discover_expected_files()
    actual = {f.name for f in OUTPUT_DIR.glob("*.md")}

    missing = expected - actual
    unexpected = actual - expected

    errors: list[str] = []
    if missing:
        errors.append(f"Missing outputs: {sorted(missing)}")
    if unexpected:
        errors.append(f"Unexpected outputs: {sorted(unexpected)}")

    # Check each file is non-empty
    for name in sorted(expected & actual):
        path = OUTPUT_DIR / name
        if path.stat().st_size == 0:
            errors.append(f"Empty output: {name}")

    if errors:
        raise AssertionError("\n".join(errors))

    print(f"OK: {len(actual)} output files verified")


if __name__ == "__main__":
    test_all_outputs_exist()
