REBAR = $(shell pwd)/rebar

out/fifo: src/fifo.c
	gcc src/fifo.c -o fifo

package: out/fifo
	make -c pkg package
