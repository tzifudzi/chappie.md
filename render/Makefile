.PHONY: all render clean fmt fmt-check integ-test

all: render

render:
ifdef FILE
	python scripts/render.py $(FILE)
else
	python scripts/render.py
endif

fmt:
	prettier --write "**/*.md" "**/*.qmd"

fmt-check:
	prettier --check "**/*.md" "**/*.qmd"

clean:
	python scripts/clean.py

integ-test: render
	python scripts/integration_test.py
