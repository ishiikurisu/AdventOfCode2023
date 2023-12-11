.PHONY: default
default: run-rust

.PHONY: run-rust
run-rust:
	rustc main.rs
	./main < sample.txt
	./main < input.txt
	rm main

