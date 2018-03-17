include ../../project.mk

COMPONENT_INTERNAL=$(PROJECT)
COMPONENT=$(COMPONENT_INTERNAL)
TARGET_DIR=/opt/local
DEPS=""

include pkg.mk

.PHONY: package prepare clean

package: prepare $(FILE).tgz

clean: clean-pkg
	-rm *.tgz
	-rm $(STAGE_DIR)

prepare:
	mkdir -p $(STAGE_DIR)/sbin
	mkdir -p $(STAGE_DIR)/fifo-tools
	mkdir -p $(STAGE_DIR)/man/man1
	sed -e 's/{{VERSION}}/$(VERSION)/' ../../doc/zfifo.1 > $(STAGE_DIR)/man/man1/zfifo.1
	cp -r ../../target/release/* $(STAGE_DIR)/fifo-tools/
	chmod +x $(STAGE_DIR)/fifo-tools/fifo
	ln -sfr $(STAGE_DIR)/fifo-tools/fifo $(STAGE_DIR)/sbin/zfifo 
