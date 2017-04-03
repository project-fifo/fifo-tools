.PHONY: rel
rel: 
	cargo build --release

package: rel
	make -C rel/pkg package
	rm -rf target/release/build

man:
	nroff -man doc/zfifo.1 | less

test deps version_header:
	true

clean:
	make -C rel/pkg clean
	-rm -rf target 
