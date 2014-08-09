all: xpath-tokenizer-test xpath-token-deabbreviator-test xpath-token-disambiguator-test

libxpath.rlib: xpath.rs tokenizer.rs deabbreviator.rs token.rs
	rustc -g --crate-type=lib xpath.rs

# Need to include library in dependency
xpath-tokenizer-test: xpath-tokenizer-test.rs libxpath.rlib
	rustc -g -L . --test xpath-tokenizer-test.rs

xpath-token-deabbreviator-test: xpath-token-deabbreviator-test.rs libxpath.rlib
	rustc -g -L . --test xpath-token-deabbreviator-test.rs

xpath-token-disambiguator-test: xpath-token-disambiguator-test.rs libxpath.rlib
	rustc -g -L . --test xpath-token-disambiguator-test.rs
