VERSION := `toml get Cargo.toml workspace.package.version | jq -r`

_default:
	just --choose --chooser "fzf +s -x --tac --cycle"

# Show the list of tasks
help:
	just --list

# Before releasing, this helps bumping up the version numbers
bump level:
	cargo workspaces version {{level}} --no-individual-tags

# Clippy
_clippy:
	cargo +nightly clippy --all-features --all-targets

# Rust fmt
_fmt:
	cargo +nightly fmt --all -- --check

# Fmt + clippy
check: _fmt _clippy

# This will generate the usage instruction
usage:
	cargo run -q -- --help > doc/usage.adoc
	cargo run -q -- pull --help > doc/usage_pull.adoc
	cargo run -q -- build --help > doc/usage_build.adoc
	cargo run -q -- version --help > doc/usage_version.adoc
	cargo run -q -- info --help > doc/usage_info.adoc

# When comes the time to release, this will set a new tag
tag:
    @echo Tagging v{{VERSION}}
    git tag -f v{{VERSION}}
    git tag | sort -Vr | head

# Create and push a tag matching the current version
tag_push: tag
	git push origin v{{VERSION}}

# Converts our AsciiDoc documentation to Markdown
md:
    #!/usr/bin/env bash
    asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

# Generate the doc: usage + md
doc: usage md

# Run the tests using nextest
test:
	cargo nextest run --no-fail-fast
