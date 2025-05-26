SHELL = /bin/bash

NAME = bench-keccak256
BACKENDS = xkcp keccak-asm tiny-keccak sha3 # constantine
SIZES = 8 32 100 256 512 # 1024 2048 4096 8192 10000 16384 32768 65536
RUNS = 100000
BIN = target/release/$(NAME)

export RUSTFLAGS := $(RUSTFLAGS) -Ctarget-cpu=znver3 -Clinker-plugin-lto 

build:
	@echo "Building..."; \
	cargo build -qr; \
	for backend in $(BACKENDS); do \
		$(BIN) $$backend info; \
	done

bench: build
	@mkdir -p ./out; \
	for backend in $(BACKENDS); do \
		echo "Benchmarking $$backend..."; \
		for size in $(SIZES); do \
			outfile=./out/callgrind.$$backend-$$size.out; \
			printf "%5s -> " $$size; \
			out="$$(valgrind --tool=callgrind --callgrind-out-file=$$outfile \
				--dump-instr=yes --collect-jumps=yes \
				-- $(BIN) $$backend count 1 $$size 2>&1)"; \
			code=$$?; \
			[ $$code -ne 0 ] && (echo "$$out" 1>&2; exit 1); \
			echo "$$out" \
				| grep "I   refs:" \
				| grep -oE '[0-9,]+$$'; \
		done \
	done

hyperfine: build
	@for size in $(SIZES); do \
		args=(); \
		for backend in $(BACKENDS); do \
			args+=("$(BIN) $$backend count $(RUNS) $$size"); \
		done; \
		hyperfine --shell=none -w10 "$${args[@]}" || exit 1; \
	done

clean:
	@cargo clean && rm -rf ./out
