.PHONY: run build test doc clean
run build test doc clean:
	cargo $@

tags:
	ctags -f tags --options=./ctags.rust --recurse .
