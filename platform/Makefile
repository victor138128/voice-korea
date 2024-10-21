BUILD_ENV ?=

setup:
	npm i -g tailwindcss

.PHONY: run
run: public/tailwind.css
	$(BUILD_ENV) dx serve --platform fullstack -i false

public/tailwind.css:
	tailwindcss -i ./input.css -o ./public/tailwind.css --minify

