out/fifo: src/fifo.c
	gcc src/fifo.c -o out/fifo

rfifo/target/release/fifo:
	(cd rfifo; cargo build --release)

dist: out/fifo rfifo/target/release/fifo
	make -C rel/pkg package

man:
	nroff -man doc/zfifo.1 | less

test deps version_header:
	true
