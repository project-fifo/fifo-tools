out/fifo: src/fifo.c
	gcc src/fifo.c -o out/fifo

package: out/fifo
	make -C rel/pkg package

man:
	nroff -man doc/zfifo.1 | less

test deps version_header:
	true
