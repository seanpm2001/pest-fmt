test:
	cargo test
	./tests/cli_test.sh
update_grammar:
	curl -sSL https://github.com/pest-parser/pest/raw/master/meta/src/grammar.pest > src/grammar.pest
	patch src/grammar.pest src/grammar.patch
