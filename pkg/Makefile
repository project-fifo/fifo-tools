include ../project.mk

.PHONY: build_info package_list

package: package_list build_info
	pkg_create -i install.sh -k deinstall.sh -D displayfile -B build-info -c comment -d description -f packlist -I /opt/local/ -p staging -U $(PROJECT)-$(VERSION)$(SUFFIX).tgz

stage:
	mkdir -p staging/lib/fifo-tools
	mkdir -p staging/sbin
	mkdir -p staging/man/man1
	sed -e 's/{{VERSION}}/$(VERSION)/' ../py/zfifo > staging/sbin/zfifo
	sed -e 's/{{VERSION}}/$(VERSION)/' ../doc/zfifo.1 > staging/man/man1/zfifo.1
	cp -r ../out/fifo staging/lib/fifo-tools/

package_list: stage
	cat pre_pkg > packlist
	(cd staging/; find * -type f | sort) >> packlist
	cat post_pkg >> packlist

build_info:
	pkg_info -X pkg_install | egrep '^(MACHINE_ARCH|OPSYS|OS_VERSION|PKGTOOLS_VERSION)' >build-info
	echo "CATEGORIES=$(CATEGORIES)" >>build-info
	echo "HOMEPAGE=$(HOMEPAGE)" >>build-info

clean:
	-rm *.tgz build-info packlist

