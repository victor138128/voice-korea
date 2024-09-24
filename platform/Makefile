BUILD_ENV ?=

run:
	$(BUILD_ENV) dx serve --platform fullstack

assets/tailwind.css: node_modules
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css --minify

node_modules:
	npm install
