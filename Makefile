prog :=learning-rust

debug ?=

$(info debug is $(debug))

ifdef debug
	release :=
	target :=debug
	extension :=debug
else
	release :=--release
	target :=release
	extension :=
endif

build:
	cargo build $(release)
	cp ext/*.py target/$(target)

all: build install

help:
	@echo "usage: make $(prog) [debug=1]"
