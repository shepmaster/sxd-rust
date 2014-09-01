TESTS:= \
	xpath-expression-test \
	xpath-node-test-test \
	xpath-parser-test \
	xpath-token-deabbreviator-test \
	xpath-token-disambiguator-test \
	xpath-tokenizer-test \
	document

LIBS:= \
	libdocument.rlib \
	libxpath.rlib

all: docs $(LIBS) $(TESTS)

clean:
	rm -f $(LIBS) $(TESTS)

docs: document-docs xpath-docs

document-docs: libdocument.rlib document.rs
	rustdoc -L . document.rs

xpath-docs: libxpath.rlib xpath.rs
	rustdoc -L . xpath.rs

libdocument.rlib: document.rs
	rustc -g --crate-type=lib document.rs

LIBXPATH_SOURCE:= \
	axis.rs \
	deabbreviator.rs \
	disambiguator.rs \
	expression.rs \
	function.rs \
	node_test.rs \
	parser.rs \
	token.rs \
	tokenizer.rs \
	xpath.rs

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

xpath-node-test-test: xpath-node-test-test.rs libxpath.rlib
	rustc -g -L . --test xpath-node-test-test.rs

xpath-parser-test: xpath-parser-test.rs libxpath.rlib
	rustc -g -L . --test xpath-parser-test.rs
