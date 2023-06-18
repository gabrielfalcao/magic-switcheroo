.PHONY: all clean cls release debug fix fmt check build test examples run-$(MAGICSWITCHEROO_NAME)

INSTALL_PATH		:= $(HOME)/usr/bin/

MAGICSWITCHEROO_NAME		:=ms
MAGICSWITCHEROO_DEBUG_BIN	:=target/debug/$(MAGICSWITCHEROO_NAME)
MAGICSWITCHEROO_RELEASE_BIN	:=target/release/$(MAGICSWITCHEROO_NAME)
MAGICSWITCHEROO_BIN		:=$(MAGICSWITCHEROO_RELEASE_BIN)
MAGICSWITCHEROO_RUN		:=$(MAGICSWITCHEROO_BIN)
MAGICSWITCHEROO_RUN		:=cargo run --bin $(MAGICSWITCHEROO_NAME) --
export CFLAGS			:="$(shell pkg-config --cflags libmagic)"
export CPPFLAGS			:="$(shell pkg-config --cflags libmagic)"
export LDFLAGS			:="$(shell pkg-config --libs libmagic)"

all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(MAGICSWITCHEROO_RELEASE_BIN): $(INSTALL_PATH)
	cargo build --release

$(MAGICSWITCHEROO_DEBUG_BIN): $(INSTALL_PATH)
	cargo build

run: $(MAGICSWITCHEROO_DEBUG_BIN) $(MAGICSWITCHEROO_RELEASE_BIN)
	@mkdir -p target
	@cp pix.jpg target/image.jpg
	$(MAGICSWITCHEROO_RUN) decode target/image.jpg
	@cp pix.jpg target/image.jpg
	$(MAGICSWITCHEROO_RUN) jsonify target/image.jpg --magic=SSENTERPRISE
	@cp pix.jpg target/image.jpg
	$(MAGICSWITCHEROO_RUN) switch target/image.jpg --magic=SSENTERPRISE
	@cp pix.jpg target/image.jpg
	$(MAGICSWITCHEROO_RUN) brush target/image.jpg --magic=SSENTERPRISE

release: check fix | $(MAGICSWITCHEROO_RELEASE_BIN)
	install $(MAGICSWITCHEROO_RELEASE_BIN) $(INSTALL_PATH)

debug: check fix | $(MAGICSWITCHEROO_DEBUG_BIN)
	install $(MAGICSWITCHEROO_DEBUG_BIN) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cls:
	-@reset || tput reset

fix:
	cargo fix --allow-dirty --allow-staged

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

build test: check
	cargo $@
