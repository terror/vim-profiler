set dotenv-load

default:
	just --list

alias f := fmt
alias r := run
alias t := test

all: build test clippy fmt-check

[group: 'misc']
build:
  cargo build

[group: 'check']
check:
 cargo check

[group: 'check']
ci: test clippy forbid
  cargo fmt --all -- --check
  cargo update --locked --package vim-profiler

[group: 'check']
clippy:
  cargo clippy --all --all-targets

[group: 'format']
fmt:
  cargo fmt

[group: 'format']
fmt-check:
  cargo fmt --all -- --check

[group: 'check']
forbid:
  ./bin/forbid

[group: 'misc']
install:
  cargo install -f vim-profiler

[group: 'dev']
install-dev-deps:
  cargo install cargo-watch

[group: 'dev']
run *args:
  cargo run {{ args }}

[group: 'test']
test:
  cargo test

[group: 'release']
update-changelog:
  echo >> CHANGELOG.md
  git log --pretty='format:- %s' >> CHANGELOG.md

[group: 'dev']
watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
