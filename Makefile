DOCKER_NAME ?= dinghao188/rcore-tutorial
DIR := workplace
.PHONY: docker build_docker


test: test3 test4 test5 test6 test7 test8

lab1: test3

lab2: test4

lab3: test5

lab4: test6 test7

lab5: test8

setup:
	rm -rf  ${DIR}
	mkdir ${DIR}
	cp -r easy-fs ${DIR}
	cp -r easy-fs-fuse ${DIR}
	cp -r ci-user ${DIR}
	cp -r bootloader ${DIR}
	cp -r reports ${DIR}
	cp rust-toolchain ${DIR}
	export PATH=$PATH:$HOME/qemu-7.0.0
	export PATH=$PATH:$HOME/qemu-7.0.0/riscv64-softmmu

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

setupclassroom:
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add  .github/workflows/classroom.yml
	git commit -m"update classroom.yml for classroom CI test"
	git push	
# setupenv:
# 	sudo apt-get update
# 	sudo apt-get install -y curl wget autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc xz-utils zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3
# 	curl https://sh.rustup.rs -sSf > $RUSTUP && chmod +x $RUSTUP
# 	$RUSTUP -y --default-toolchain nightly-2022-04-11 --profile minimal
# 	cd $HOME
# 	wget https://download.qemu.org/qemu-7.0.0.tar.xz
# 	tar xvJf qemu-7.0.0.tar.xz
# 	cd qemu-7.0.0
# 	./configure --target-list=riscv64-softmmu
# 	make install

