css:
	tailwindcss -i ./assets/styles/input.css -o ./assets/styles/output.css

css-clean:
	rm assets/styles/output.css

image: css
	docker build .

image-release: css
	docker build --platform=linux/amd64,linux/arm64 . -t anybote/lighthouse:latest

push-image:
	docker push anybote/lighthouse:latest
