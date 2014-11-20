out/fifo: src/fifo.c
	gcc src/fifo.c -o fifo

package: out/fifo
	make -C pkg package
