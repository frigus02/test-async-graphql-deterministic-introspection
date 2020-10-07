SHELL := bash

.PHONY: test
test: target/debug/test-async-graphql-deterministic-introspection
	diff <($<) <($<)

target/debug/test-async-graphql-deterministic-introspection: src
	cargo build
