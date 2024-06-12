image:
	docker build .

image-release:
	docker build --platform=linux/amd64,linux/arm64 . -t anybote/lighthouse:latest

push-image:
	docker push anybote/lighthouse:latest
