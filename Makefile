.PHONY: rel
dist: target/release/fifo
	make -C rel/pkg package
	rm -rf target/release/build

target/release/fifo: 
	cargo build --release

man:
	nroff -man doc/zfifo.1 | less

test deps version_header:
	true

clean:
	make -C rel/pkg clean
	-rm -rf target 
