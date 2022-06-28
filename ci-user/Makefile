RAND := $(shell awk 'BEGIN{srand();printf("%d", 65536*rand())}')
CHAPTER ?=

ifeq ($(CHAPTER), 3)
	LAB := 1
else ifeq ($(CHAPTER), 4)
	LAB := 2
else ifeq ($(CHAPTER), 5)
	INITPROC := 1
	LAB := 3
else ifeq ($(CHAPTER), 6)
	INITPROC := 1
	LAB := 4
else ifeq ($(CHAPTER), 7)
	INITPROC := 1
	LAB := 4
else ifeq ($(CHAPTER), 8)
	INITPROC := 1
	LAB := 5
endif

randomize:
	find user/src/bin -name "*.rs" | xargs sed -i 's/OK/OK$(RAND)/g'
	find user/src/bin -name "*.rs" | xargs sed -i 's/passed/passed$(RAND)/g'
	find check -name "*.py" | xargs sed -i 's/OK/OK$(RAND)/g'
	find check -name "*.py" | xargs sed -i 's/passed/passed$(RAND)/g'

test: randomize
	python3 overwrite.py $(CHAPTER)
	make -C user build BASE=2 TEST=$(CHAPTER) CHAPTER=$(CHAPTER)
ifdef INITPROC
	cp -f user/build/elf/ch$(CHAPTER)_usertest.elf user/build/elf/ch$(CHAPTER)b_initproc.elf
endif
	make -C ../os run | tee stdout-ch$(CHAPTER)
	python3 check/ch$(CHAPTER).py < stdout-ch$(CHAPTER)

ifdef LAB
	@for i in $(shell seq $(LAB)); do \
	if ! [ -f ../reports/lab$$i.pdf -o -f ../reports/lab$$i.md ]; then \
		echo "Report for lab$$i needed. Add your report to reports/lab$$i.pdf or reports/lab$$i.md" ; \
		exit 1 ; \
	else \
		echo "Report for lab$$i found." ; \
	fi; \
	done
endif

.PHONY: test randomize
