#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 PandaCubGames
#
# Installs the development tools this repository needs and enables the git
# hooks. Safe to run again: it only installs what is missing.
#
#   scripts/setup-dev.sh
#
# Needs already on the machine: git, curl, Node.js with npm, and uv or pipx.
# It does not set up commit signing, the commit identity or GitHub access;
# those need the owner's own keys and are described in AGENTS.md.

set -euo pipefail

cd "$(dirname "$0")/.."

MIN_RUST="1.88"

say() { printf '\n==> %s\n' "$*"; }
warn() { printf 'warning: %s\n' "$*" >&2; }
die() { printf 'error: %s\n' "$*" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

# --- Versions pinned elsewhere in the repository ---------------------------
# Read them from where they are already pinned, so this script cannot drift.

# Commitizen: the rev of its hook in .pre-commit-config.yaml (v4.19.3 -> 4.19.3).
cz_version="$(awk '/commitizen-tools\/commitizen/ { found = 1 }
                   found && /rev:/ { sub(/^v/, "", $2); print $2; exit }' \
                   .pre-commit-config.yaml)"
[ -n "$cz_version" ] || die "no Commitizen version found in .pre-commit-config.yaml"

# OpenSpec: the CLI version that generated the checked-in skills.
openspec_version="$(sed -n 's/^ *generatedBy: *"\(.*\)"/\1/p' \
                    .claude/skills/openspec-propose/SKILL.md | head -n 1)"
[ -n "$openspec_version" ] || die "no OpenSpec version found in the generated skills"

# --- Rust -------------------------------------------------------------------
say "Rust (stable, $MIN_RUST or newer, with rustfmt and clippy)"
if ! have rustup; then
    have curl || die "curl is needed to install Rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |
        sh -s -- -y --profile minimal --default-toolchain stable
fi
# An existing toolchain is left at its version; this only adds what is missing.
have rustc || rustup toolchain install stable --profile minimal
rustup component add rustfmt clippy
rust_version="$(rustc --version | awk '{ print $2 }')"
lowest="$(printf '%s\n%s\n' "$MIN_RUST" "$rust_version" | sort -V | head -n 1)"
[ "$lowest" = "$MIN_RUST" ] || die "Rust $rust_version is older than $MIN_RUST"

# The Linux release build is statically linked with musl. Only release work
# needs this target, so a failed download is a warning, not an error.
if [ "$(uname -s)" = "Linux" ]; then
    rustup target add "$(uname -m)-unknown-linux-musl" ||
        warn "could not add the musl target; static release builds won't work"
fi

# --- OpenSpec ---------------------------------------------------------------
say "OpenSpec $openspec_version"
have npm || die "Node.js with npm is needed to install OpenSpec"
if [ "$(openspec --version 2>/dev/null || true)" != "$openspec_version" ]; then
    npm install --global --no-fund --no-audit "@fission-ai/openspec@$openspec_version"
fi
openspec config set telemetry.enabled false >/dev/null

# --- Commitizen and pre-commit ----------------------------------------------
say "Commitizen $cz_version and pre-commit"
if have uv; then
    uv tool install --quiet "commitizen==$cz_version"
    uv tool install --quiet pre-commit
elif have pipx; then
    pipx install --force "commitizen==$cz_version"
    pipx install pre-commit
else
    die "uv or pipx is needed to install Commitizen and pre-commit"
fi

# --- Git hooks --------------------------------------------------------------
say "Git hooks"
if git rev-parse --git-dir >/dev/null 2>&1; then
    pre-commit install --hook-type commit-msg --hook-type pre-commit
    # Download the hook environments now, so the first commit isn't slow.
    pre-commit install-hooks || warn "could not prepare the hook environments"
else
    warn "not a git checkout; hooks not enabled"
fi

# --- Dependencies -----------------------------------------------------------
say "Rust dependencies"
cargo fetch --locked || warn "could not download the Rust dependencies"

say "Installed"
rustc --version
cargo --version
printf 'openspec %s\n' "$(openspec --version)"
printf 'commitizen %s\n' "$(cz version)"
pre-commit --version
