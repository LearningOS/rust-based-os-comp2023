DOCKER_NAME ?= dinghao188/rcore-tutorial
DIR := workplace
.PHONY: docker build_docker

test: test3 test4 test5 test6 test7 test8

setup:
	rm -rf  ${DIR}
	mkdir ${DIR}
	cp -r easy-fs ${DIR}
	cp -r easy-fs-fuse ${DIR}
	cp -r ci-user ${DIR}
	cp -r bootloader ${DIR}
	cp -r reports ${DIR}
	cp rust-toolchain ${DIR}

test1: setup
	cp -r os1 ${DIR}/os
	cd ${DIR}/ci-user && make test CHAPTER=1


test2: setup
	cp -r os2 ${DIR}/os
	cd ${DIR}/ci-user && make test CHAPTER=2

test3: setup
	cp -r os3 ${DIR}/os
	cd ${DIR}/ci-user && make test CHAPTER=3

test4: setup
	cp -r os4 ${DIR}/os
	cd ${DIR}/ci-user && make test CHAPTER=4

test5: setup
	cp -r os5 ${DIR}/os
	cd ${DIR}/ci-user && make test CHAPTER=5

test6: setup
	cp -r os6 ${DIR}/os
	cd ${DIR}/ci-user && make test CHAPTER=6

test7: setup
	cp -r os7 ${DIR}/os
	cd ${DIR}/ci-user && make test CHAPTER=7

test8: setup
	cp -r os8 ${DIR}/os
	cd ${DIR}/ci-user && make test CHAPTER=8

clean:
	rm -rf ${DIR}

docker:
	docker run --rm -it --mount type=bind,source=$(shell pwd),destination=/mnt ${DOCKER_NAME}

build_docker: 
	docker build -t ${DOCKER_NAME} .

