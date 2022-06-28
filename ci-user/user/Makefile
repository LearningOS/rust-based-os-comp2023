TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
BUILD_DIR := build
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
PY := python3

BASE ?= 0
CHAPTER ?= 0
TEST ?= $(CHAPTER)

ifeq ($(TEST), 0) # No test, deprecated, previously used in v3
	APPS :=  $(filter-out $(wildcard $(APP_DIR)/ch*.rs), $(wildcard $(APP_DIR)/*.rs))
else ifeq ($(TEST), 1) # All test
	APPS :=  $(wildcard $(APP_DIR)/ch*.rs)
else
	TESTS := $(shell seq $(BASE) $(TEST))
	ifeq ($(BASE), 0) # Normal tests only
		APPS := $(foreach T, $(TESTS), $(wildcard $(APP_DIR)/ch$(T)_*.rs))
	else ifeq ($(BASE), 1) # Basic tests only
		APPS := $(foreach T, $(TESTS), $(wildcard $(APP_DIR)/ch$(T)b_*.rs))
	else # Basic and normal
		APPS := $(foreach T, $(TESTS), $(wildcard $(APP_DIR)/ch$(T)*.rs))
	endif
endif

ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

binary:
	@echo $(ELFS)
	@if [ ${CHAPTER} -gt 3 ]; then \
		cargo build --release ;\
	else \
		CHAPTER=$(CHAPTER) python3 build.py ;\
	fi
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf)); \
		cp $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.elf, $(elf));)

disasm:
	@$(foreach elf, $(ELFS), \
		$(OBJDUMP) $(elf) -S > $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.asm, $(elf));)
	@$(foreach t, $(ELFS), cp $(t).asm $(BUILD_DIR)/asm/;)

pre:
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@mkdir -p $(BUILD_DIR)/app/
	@mkdir -p $(BUILD_DIR)/asm/
	@$(foreach t, $(APPS), cp $(t) $(BUILD_DIR)/app/;)

build: clean pre binary
	@$(foreach t, $(ELFS), cp $(t).bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(ELFS), cp $(t).elf $(BUILD_DIR)/elf/;)

clean:
	@cargo clean
	@rm -rf $(BUILD_DIR)

all: build

.PHONY: elf binary build clean all