PYTHON ?= python

all:
	$(PYTHON) setup.py build_ext -i

.PHONY:

clean:
	$(PYTHON) setup.py clean --all
	cd doc && make clean

install: all
	$(PYTHON) setup.py install

dist: FORCE
	$(PYTHON) setup.py sdist

test:
	tox -e py37

doc: .PHONY
	tox -e docs

coverage: all
	tox -e coverage

benchmark: all
	asv continuous master

wheel:
	export DOCKER_IMAGE=quay.io/pypa/manylinux1_x86_64; \
	docker pull $${DOCKER_IMAGE} ; \
	docker run --rm -v `pwd`:/io $${DOCKER_IMAGE} /io/travis/build-wheels.sh

last-tag:
	git fetch -p -q; git tag -l | sort -V | tail -1

FORCE:
