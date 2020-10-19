SHELL := bash

.PHONY: validate
validate: schema.json
	yarn
	node index.js

schema.json: target/debug/test-async-graphql-deterministic-introspection
	($<) >$@

.PHONY: test
test: target/debug/test-async-graphql-deterministic-introspection
	diff <($<) <($<)

target/debug/test-async-graphql-deterministic-introspection: Cargo.* src
	cargo build
