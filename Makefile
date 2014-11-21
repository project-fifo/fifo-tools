out/fifo: src/fifo.c
	gcc src/fifo.c -o out/fifo

package: out/fifo
	make -C pkg package

man:
	nroff -man doc/fifo-tool.1 | less
