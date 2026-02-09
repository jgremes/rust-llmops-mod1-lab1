# Makefile Demo with Rust

## Instructions

1. Navigate to the project directory containing `main.rs` and `Makefile`.

2. Build the project using `make all`. This will:
   - Format the code with \`cargo fmt\`
   - Lint the code with \`cargo clippy\`  
   - Run unit tests with \`cargo test\`
   - Run the program with \`cargo run\`

3. Verify that the output looks correct. You should see the sum, difference, product and quotient of 10 and 5 printed.  

4. Make a change to \`main.rs\`, for example change the numbers being used. 

5. Run \`make all\` again. Notice it automatically reformatted, linted, and retested with the new code.

## Challenges  

1. Add a new math function to \`main.rs\` (e.g. power), include tests. Update \`Makefile\` to run tests and program.

2. Parameterize the test values in \`main.rs\` to make them configurable. Read test inputs from a file or environment variables.

3. Break the calculations out into a separate \`lib.rs\` module that \`main.rs\` depends on. Update \`Makefile\` accordingly.

4. Add logic to \`Makefile\` to only build/test dependent artifacts that changed rather than everything.

5. Set up a build process to run this \`Makefile\` to validate PRs automatically in your source control system.