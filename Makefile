all: xpath-tokenizer-test

libxpath.rlib: xpath.rs tokenizer.rs
	rustc -g --crate-type=lib xpath.rs

# Need to include library in dependency
xpath-tokenizer-test: xpath-tokenizer-test.rs libxpath.rlib
	rustc -g -L . --test xpath-tokenizer-test.rs

xpath-token-deabbreviator-test: xpath-token-deabbreviator-test.rs libxpath.rlib
	rustc -g -L . --test xpath-token-deabbreviator-test.rs
