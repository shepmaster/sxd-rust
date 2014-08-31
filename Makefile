TESTS:= \
	xpath-tokenizer-test \
	xpath-token-deabbreviator-test \
	xpath-token-disambiguator-test \
	xpath-expression-test \
	xpath-parser-test \
	document

LIBS:= \
	libdocument.rlib \
	libxpath.rlib

all: $(LIBS) $(TESTS)

clean:
	rm -f $(LIBS) $(TESTS)

docs:
	rustdoc document.rs

libdocument.rlib: document.rs
	rustc -g --crate-type=lib document.rs

LIBXPATH_SOURCE:=xpath.rs tokenizer.rs deabbreviator.rs token.rs disambiguator.rs axis.rs expression.rs parser.rs

libxpath.rlib: $(LIBXPATH_SOURCE) libdocument.rlib
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

xpath-parser-test: xpath-parser-test.rs libxpath.rlib
	rustc -g -L . --test xpath-parser-test.rs
