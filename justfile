set dotenv-load := true

init:
    cargo install action-validator dircat just
    brew install lefthook

check: fmt
    cargo +nightly clippy -- -W clippy::pedantic

check_fmt:
    cargo +nightly fmt -- --check

yaml_fmt:
    yamlfmt lefthook.yml
    yamlfmt -dstar .github/**/*.{yaml,yml}

md_fmt:
    markdown-fmt -m 80 CONTRIBUTING.md
    markdown-fmt -m 80 README.md

fmt: yaml_fmt md_fmt
    cargo +nightly fmt

test:
    cargo test

audit:
    cargo audit -D warnings

doc:
    cargo doc --open

release: check
    cargo +stable build --release
