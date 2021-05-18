VERSION := `toml get cli/Cargo.toml package.version | jq -r`

bump:
	cargo workspaces version minor --no-individual-tags

usage:
	cargo run -q -- --help > doc/usage.adoc
	cargo run -q -- build --help > doc/usage_build.adoc

tag:
    @echo Tagging v{{VERSION}}
    git tag -f v{{VERSION}}
