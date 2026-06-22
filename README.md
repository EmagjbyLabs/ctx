# ctx / repoctx

`ctx / repoctx` is a local-first CLI for turning a Git repository into a prompt-ready context digest. It walks the current repository, skips common generated and binary files, renders a directory tree, and includes the contents of matching text files.

By default, the installed binary is named `ctx` and copies the generated digest to your clipboard. Use `--stdout` when you want to print the digest instead.

Because `repoctx` runs locally and reads files directly from your working tree, it is useful for private repositories and codebases that should not be uploaded to a hosted indexing service.

## Install

Install from this repository with Cargo:

```sh
cargo install --path .
```

You can also use the provided Make target:

```sh
make install
```

## Run Locally

Run the tool from anywhere inside a Git repository:

```sh
ctx
```

For development, run it without installing:

```sh
cargo run
```

## Usage Examples

Generate a digest for the current repository and copy it to the clipboard:

```sh
ctx
```

Include only Rust source files and `Cargo.toml`:

```sh
ctx --include 'src/**/*.rs' 'Cargo.toml'
```

Exclude generated files, lockfiles, or other paths that are not useful as context:

```sh
ctx --exclude 'target/**' '*.lock' 'src/generated/**'
```

Combine include and exclude filters:

```sh
ctx --include 'src/**/*.rs' --exclude 'src/generated/**'
```

Print the digest to stdout instead of copying it to the clipboard:

```sh
ctx --stdout
```

Save stdout output to a file:

```sh
ctx --stdout > repo-context.md
```

## Options

```text
Usage: ctx [OPTIONS]

Options:
      --include <PATTERN>...  Include only files matching these glob-style patterns
      --exclude <PATTERN>...  Exclude files matching these glob-style patterns
      --stdout                Print the generated digest to stdout instead of copying it to the clipboard
  -h, --help                  Print help
  -V, --version               Print version
```

`repoctx` respects Git ignore files and skips common dependency, build, hidden repository metadata, large, and binary artifacts by default.
