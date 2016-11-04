include ../../project.mk

COMPONENT_INTERNAL=$(PROJECT)
COMPONENT=$(COMPONENT_INTERNAL)
TARGET_DIR=/opt/local
DEPS="python33"

include pkg.mk

.PHONY: package prepare clean

package: prepare $(FILE).tgz

clean: clean-pkg
	-rm *.tgz

prepare:
	mkdir -p $(STAGE_DIR)/lib/fifo-tools
	mkdir -p $(STAGE_DIR)/sbin
	mkdir -p $(STAGE_DIR)/man/man1
	sed -e 's/{{VERSION}}/$(VERSION)/' ../../py/zfifo > $(STAGE_DIR)/sbin/zfifo
	sed -e 's/{{VERSION}}/$(VERSION)/' ../../doc/zfifo.1 > $(STAGE_DIR)/man/man1/zfifo.1
	chmod +x $(STAGE_DIR)/sbin/zfifo
	cp -r ../../out/fifo $(STAGE_DIR)/lib/fifo-tools/
