"""Remove the rendered output directory.

Uses Python instead of shell commands for cross-platform compatibility.
"""

import shutil
import sys
from pathlib import Path

OUTPUT_DIR = Path("_output")


def clean(output_dir: Path) -> None:
    """Remove the output directory."""
    if output_dir.exists():
        shutil.rmtree(output_dir)
        print(f"Removed {output_dir}/")
    else:
        print(f"{output_dir}/ does not exist, nothing to clean.")


if __name__ == "__main__":
    clean(OUTPUT_DIR)
