.PHONY: all render clean

all: render

render:
	quarto render
	@echo "Flattening output..."
	@find _output -name '*.md' -mindepth 2 -exec mv {} _output/ \;
	@find _output -type d -empty -delete

clean:
	rm -rf _output
