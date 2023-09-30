NAME=bench-keccak256
BACKENDS = sha3 tiny-keccak keccak-asm
SIZES = 1 2 4 8 16 20 32 64 100 128 256 512 1024 2048 4096 8192 10000 16384 32768 65536
# SIZES = 32
RUNS = 1000000
BIN = target/release/$(NAME)

build:
	@echo "Building..."; \
	cargo build -qr

# --cache-sim=yes --branch-sim=yes
bench: build
	@mkdir -p ./out; \
	for backend in $(BACKENDS); do \
		echo "Benchmarking $$backend..."; \
		for size in $(SIZES); do \
			outfile=./out/callgrind.$$backend-$$size.out; \
			printf "%5s -> " $$size; \
			valgrind --tool=callgrind --callgrind-out-file=$$outfile \
				--dump-instr=yes --collect-jumps=yes \
				$(BIN) $$backend size $$size 2>&1 \
				| grep "I   refs:" \
				| grep -oE '[0-9,]+$$'; \
		done \
	done

hyperfine: build
	@args=(); \
	for backend in $(BACKENDS); do \
		args+=("$(BIN) $$backend count $(RUNS)"); \
	done; \
	hyperfine -w10 -r20 "$${args[@]}";

clean:
	@rm -rf ./out
