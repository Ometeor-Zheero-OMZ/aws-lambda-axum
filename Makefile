.PHONY: build
build:
	cargo lambda build --release

.PHONY: deploy
deploy:
	cargo lambda deploy --enable-function-url rust-lambda --iam-role arn:aws:iam::047719615141:role/AWSLambdaRole