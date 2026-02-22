.PHONY: all render clean fmt fmt-check

all: render

render:
	quarto render
	@echo "Flattening output..."
	@find _output -name '*.md' -mindepth 2 -exec mv {} _output/ \;
	@find _output -type d -empty -delete

fmt:
	prettier --write "**/*.md" "**/*.qmd"

fmt-check:
	prettier --check "**/*.md" "**/*.qmd"

clean:
	rm -rf _output
