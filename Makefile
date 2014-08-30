all: xpath-tokenizer-test xpath-token-deabbreviator-test xpath-token-disambiguator-test xpath-expression-test libdocument.rlib

clean:
	rm -f libdocument.rlib libxpath.rlib document

docs:
	rustdoc document.rs

libdocument.rlib: document.rs
	rustc -g --crate-type=lib document.rs

libxpath.rlib: xpath.rs tokenizer.rs deabbreviator.rs token.rs disambiguator.rs axis.rs expression.rs
	rustc -g --crate-type=lib -L . xpath.rs

document: document.rs
	rustc -g --crate-type=lib --test document.rs

# Need to include library in dependency
xpath-tokenizer-test: xpath-tokenizer-test.rs libxpath.rlib
	rustc -g -L . --test xpath-tokenizer-test.rs

xpath-token-deabbreviator-test: xpath-token-deabbreviator-test.rs libxpath.rlib
	rustc -g -L . --test xpath-token-deabbreviator-test.rs

xpath-token-disambiguator-test: xpath-token-disambiguator-test.rs libxpath.rlib
	rustc -g -L . --test xpath-token-disambiguator-test.rs

xpath-expression-test: xpath-expression-test.rs libxpath.rlib
	rustc -g -L . --test xpath-expression-test.rs
