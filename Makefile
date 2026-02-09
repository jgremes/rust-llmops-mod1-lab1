#  Project components
NAME = hello-calc
LIB_NAME = calc_lib

# Artifacts
BIN_ARTIFACT = target/debug/$(NAME)
LIB_ARTIFACT = target/debug/lib$(LIB_NAME).rlib

# Source files
LIB_SRC = src/lib.rs
BIN_SRC = src/main.rs

# Test data
TEST_DATA = tests/fixtures/*.txt

#  Build the library only if lib.rs has changed
$(LIB_ARTIFACT): $(LIB_SRC)
	@echo "Building library if needed..."
	cargo build --lib

#  Build the binary only if main.rs or library have changed
$(BIN_ARTIFACT): $(BIN_SRC) $(LIB_ARTIFACT)
	@echo "Building binary  if needed..."
	cargo build --bin $(NAME)

# Build
build: $(BIN_ARTIFACT)

# Commands
# Format the code according to Rust standards
format:
	cargo fmt --quiet

# Run Clippy for static analysis
lint:
	cargo clippy --quiet	

#  Run tests only if library/data changed with passed marker
target/.test_passed: $(LIB_ARTIFACT) $(TEST_DATA)
	@echo "Running unit tests with parameterized data..."
	cargo test --lib --quiet
	@mkdir -p target
	@touch target/.test_passed

# Test command
test: target/.test_passed	

# Run the final program
run: $(BIN_ARTIFACT)
	@echo "Executing $(NAME)..."
	./$(BIN_ARTIFACT)

# Clean build artifacts
clean:
	cargo clean

# Default rule: format, lint,  build and test the project
all: format lint build test

.PHONY: all build test run format lint clean
