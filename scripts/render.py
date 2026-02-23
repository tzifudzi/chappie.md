"""Render all Quarto agents, flatten output, and print a build summary.

Uses Python instead of shell commands for cross-platform compatibility.
"""

import shutil
import subprocess
import sys
from pathlib import Path

OUTPUT_DIR = Path("_output")


def render(file: str | None = None) -> None:
    """Call quarto render, optionally for a single file."""
    cmd = ["quarto", "render"]
    if file:
        cmd.append(file)
    result = subprocess.run(cmd, check=False)
    if result.returncode != 0:
        print("ERROR: quarto render failed", file=sys.stderr)
        print("ERROR: quarto render failed", file=sys.stderr)
        sys.exit(result.returncode)


def flatten(output_dir: Path) -> None:
    """Move all .md files from subdirectories into the top-level output dir."""
    for md_file in list(output_dir.rglob("*.md")):
        if md_file.parent != output_dir:
            shutil.move(str(md_file), str(output_dir / md_file.name))

    # Remove now-empty subdirectories
    for d in sorted(output_dir.rglob("*"), key=lambda p: -len(p.parts)):
        if d.is_dir() and not any(d.iterdir()):
            d.rmdir()


def summary(output_dir: Path) -> None:
    """Print a summary of generated files with line and word counts."""
    files = sorted(output_dir.glob("*.md"))
    if not files:
        print("No files generated.")
        return

    print("Build Summary")
    total_lines = 0
    total_words = 0
    for f in files:
        text = f.read_text(encoding="utf-8")
        lines = text.count("\n")
        words = len(text.split())
        total_lines += lines
        total_words += words
        print(f"  - {f.name}: {lines:,} lines, {words:,} words")
    n = len(files)
    print(f"  Avg: {total_lines // n:,} lines, {total_words // n:,} words")


def main() -> None:
    file = sys.argv[1] if len(sys.argv) > 1 else None
    render(file)
    flatten(OUTPUT_DIR)
    summary(OUTPUT_DIR)


if __name__ == "__main__":
    main()
