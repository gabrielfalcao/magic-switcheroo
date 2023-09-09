.PHONY: all clean cls release debug fix fmt check build test

INSTALL_PATH		:= $(HOME)/usr/libexec/

MAGICSWITCHEROO_NAME		:=ms
MAGICSWITCHEROO_DEBUG_BIN	:=target/debug/$(MAGICSWITCHEROO_NAME)
MAGICSWITCHEROO_RELEASE_BIN	:=target/release/$(MAGICSWITCHEROO_NAME)
MAGICSWITCHEROO_BIN		:=$(MAGICSWITCHEROO_RELEASE_BIN)
MS				:=$(MAGICSWITCHEROO_BIN)
MS				:=cargo run --bin $(MAGICSWITCHEROO_NAME) --
export CFLAGS			:="$(shell pkg-config --cflags libmagic)"
export CPPFLAGS			:="$(shell pkg-config --cflags libmagic)"
export LDFLAGS			:="$(shell pkg-config --libs libmagic)"
export K9_UPDATE_SNAPSHOTS	:=1
all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(MAGICSWITCHEROO_RELEASE_BIN): $(INSTALL_PATH)
	cargo build --release

$(MAGICSWITCHEROO_DEBUG_BIN): $(INSTALL_PATH)
	cargo build

run: $(MAGICSWITCHEROO_DEBUG_BIN) $(MAGICSWITCHEROO_RELEASE_BIN)
	@mkdir -p target
	@cp pix.jpg target/mspix.jpg
	$(MS) e target/mspix.jpg --magic=SSENTERPRISE
	$(MS) r target/mspix.jpg --magic=SSENTERPRISE
	diff pix.jpg target/mspix.jpg
	@cp switcheroo.jpg target/msswitcheroo.jpg
	$(MS) e target/msswitcheroo.jpg --magic=AINTSOMETHIN
	$(MS) r target/msswitcheroo.jpg --magic=AINTSOMETHIN
	diff switcheroo.jpg target/msswitcheroo.jpg
	@cp switcheroo.jpg target/msswitcheroo.jpg
	$(MS) e target/msswitcheroo.jpg --magic=MAGIC
	$(MS) r target/msswitcheroo.jpg --magic=MAGIC
	diff switcheroo.jpg target/msswitcheroo.jpg
	$(MS) e target/msswitcheroo.jpg --magic=OVERFLOWOVERLOWOVERFLOWOVERFLOWOVERLOWOVERFLOWOVERFLOWOVERLOWOVERFLOWOVERFLOWOVERLOWOVERFLOW
	$(MS) r target/msswitcheroo.jpg --magic=OVERFLOWOVERLOWOVERFLOWOVERFLOWOVERLOWOVERFLOWOVERFLOWOVERLOWOVERFLOWOVERFLOWOVERLOWOVERFLOW
	diff switcheroo.jpg target/msswitcheroo.jpg
	$(MS) gp target/msswitcheroo.jpg 0x5e
	$(MS) gs target/msswitcheroo.jpg 0x25
	$(MS) ds target/msswitcheroo.jpg 1
	$(MS) de target/msswitcheroo.jpg 1
	diff switcheroo.jpg target/msswitcheroo.jpg

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
