# The name of the module
NAME = hello
# The module's final file
MOD_FILE = $(NAME).kmod

# The path to the kernel's sources
KERN_SRC ?= ../maestro

# The path to the configuration file
CONFIG_PATH := $(shell realpath "$(KERN_SRC)/.config")

# Configuration as arguments for the compiler
CONFIG_ARGS := # TODO

# The architecture to compile for
# TODO Use the config file, but override with environement
ARCH ?= x86

# The absolute path to the linker script
LINKER_PATH := $(shell realpath "$(KERN_SRC)/arch/$(ARCH)/linker.ld")
# The absolute path to the target file
TARGET_PATH := $(shell realpath "$(KERN_SRC)/arch/$(ARCH)/target.json")

# The flags for the Rust compiler
RUSTFLAGS = -C link-arg=-T$(LINKER_PATH) -C prefer-dynamic $(CONFIG_ARGS)

# TODO Support release
$(MOD_FILE):
	RUSTFLAGS='$(RUSTFLAGS)' cargo +nightly build --target $(TARGET_PATH)
	cp target/target/debug/lib$(NAME).so $(MOD_FILE)
