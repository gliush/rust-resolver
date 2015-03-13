.PHONY: run build test doc clean tags
run build test doc clean:
	cargo $@

tags:
	ctags -f tags --options=./ctags.rust --recurse .
