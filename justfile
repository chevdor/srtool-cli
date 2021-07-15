VERSION := `toml get cli/Cargo.toml package.version | jq -r`

_default:
    just --list

# Before releasing, this helps bumping up the version numbers
bump level:
	cargo workspaces version {{level}} --no-individual-tags

# This will generate the usage instruction
usage:
	cargo run -q -- --help > doc/usage.adoc
	cargo run -q -- pull --help > doc/usage_pull.adoc
	cargo run -q -- build --help > doc/usage_build.adoc
	cargo run -q -- version --help > doc/usage_version.adoc
	cargo run -q -- info --help > doc/usage_info.adoc
	cargo run -q -- verify --help > doc/usage_verify.adoc

# When comes the time to release, this will set a new tag
tag:
    @echo Tagging v{{VERSION}}
    git tag -f v{{VERSION}}

# Converts our AsciiDoc documentation to Markdown
md:
    #!/usr/bin/env bash
    asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

