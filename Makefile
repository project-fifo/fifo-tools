REBAR = $(shell pwd)/rebar

out/fifo: utils/fifo.c
	gcc utils/fifo.c -o fifo
