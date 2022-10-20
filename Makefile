DOCKER_NAME ?= rust-os-camp-2022
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
	cp rust-toolchain.toml ${DIR}
#	export PATH=${PATH}:${HOME}/qemu-7.0.0:${HOME}/qemu-7.0.0/riscv64-softmmu

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
	docker run --rm -it -v ${PWD}:/mnt -w /mnt ${DOCKER_NAME} bash

build_docker:
	docker build -t ${DOCKER_NAME} .

setupclassroom_test1:
	mkdir -p .github/classroom/
	mkdir -p .github/workflows/
	touch .github/.keep
	cp scripts/autograding-test1.json .github/classroom/autograding.json
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add .github/classroom/autograding.json
	git add .github/workflows/classroom.yml
	git add .github/.keep
	git commit -m"update classroom.yml .keep autograding.json for classroom CI test"
	git push

setupclassroom_test2:
	mkdir -p .github/classroom/
	mkdir -p .github/workflows/
	touch .github/.keep
	cp scripts/autograding-test2.json .github/classroom/autograding.json
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add .github/classroom/autograding.json
	git add  .github/workflows/classroom.yml
	git add  .github/.keep
	git commit -m"update classroom.yml .keep autograding.json for classroom CI test"
	git push

setupclassroom_test3:
	mkdir -p .github/classroom/
	mkdir -p .github/workflows/
	touch .github/.keep
	cp scripts/autograding-test3.json .github/classroom/autograding.json
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add .github/classroom/autograding.json
	git add .github/workflows/classroom.yml
	git add .github/.keep
	git commit -m"update classroom.yml .keep autograding.json for classroom CI test"
	git push

setupclassroom_test4:
	mkdir -p .github/classroom/
	mkdir -p .github/workflows/
	touch .github/.keep
	cp scripts/autograding-test4.json .github/classroom/autograding.json
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add .github/classroom/autograding.json
	git add .github/workflows/classroom.yml
	git add .github/.keep
	git commit -m"update classroom.yml .keep autograding.json for classroom CI test"
	git push

setupclassroom_test5:
	mkdir -p .github/classroom/
	mkdir -p .github/workflows/
	touch .github/.keep
	cp scripts/autograding-test5.json .github/classroom/autograding.json
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add .github/classroom/autograding.json
	git add .github/workflows/classroom.yml
	git add .github/.keep
	git commit -m"update classroom.yml .keep autograding.json for classroom CI test"
	git push

setupclassroom_test6:
	mkdir -p .github/classroom/
	mkdir -p .github/workflows/
	touch .github/.keep
	cp scripts/autograding-test6.json .github/classroom/autograding.json
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add .github/classroom/autograding.json
	git add .github/workflows/classroom.yml
	git add .github/.keep
	git commit -m"update classroom.yml .keep autograding.json for classroom CI test"
	git push

setupclassroom_test7:
	mkdir -p .github/classroom/
	mkdir -p .github/workflows/
	touch .github/.keep
	cp scripts/autograding-test7.json .github/classroom/autograding.json
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add .github/classroom/autograding.json
	git add .github/workflows/classroom.yml
	git add .github/.keep
	git commit -m"update classroom.yml .keep autograding.json for classroom CI test"
	git push

setupclassroom_test8:
	mkdir -p .github/classroom/
	mkdir -p .github/workflows/
	touch .github/.keep
	cp scripts/autograding-test8.json .github/classroom/autograding.json
	cp scripts/classroom.yml .github/workflows/classroom.yml
	git add .github/classroom/autograding.json
	git add .github/workflows/classroom.yml
	git add .github/.keep
	git commit -m"update classroom.yml .keep autograding.json for classroom CI test"
	git push

# for local ubuntu with zsh shell SHELL, need root for sudo
ubuntu_local_setenv:
	sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 ninja-build zsh -y
	cd ${HOME} && wget https://download.qemu.org/qemu-7.0.0.tar.xz
	cd ${HOME} && tar xvJf qemu-7.0.0.tar.xz
	cd ${HOME}/qemu-7.0.0 && ./configure --target-list=riscv64-softmmu,riscv64-linux-user
	cd ${HOME}/qemu-7.0.0 && make -j$(nproc)
	cd ${HOME}/qemu-7.0.0 && sudo make install
	qemu-system-riscv64 --version
	qemu-riscv64 --version
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	source ${HOME}/.cargo/env
	rustc --version

# for github codespaces ubuntu with zsh SHELL, need root for sudo
codespaces_setenv:
	sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 ninja-build zsh -y
	cd .. && wget https://download.qemu.org/qemu-7.0.0.tar.xz
	cd .. && tar xvJf qemu-7.0.0.tar.xz
	cd ../qemu-7.0.0 && ./configure --target-list=riscv64-softmmu,riscv64-linux-user
	cd ../qemu-7.0.0 && make -j$(nproc)
	cd ../qemu-7.0.0 && sudo make install
	qemu-system-riscv64 --version
	qemu-riscv64 --version
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	/bin/zsh && source /home/codespace/.cargo/env
	rustc --version
