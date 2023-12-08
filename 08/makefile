.PHONY: default
default: run-clj

.PHONY: run-clj
run-clj:
	clj main.clj < sample.txt
	clj main.clj < input.txt > output.txt
	cat output.txt

