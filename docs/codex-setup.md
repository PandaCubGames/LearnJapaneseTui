# Codex development setup

Use the existing checkout in each isolated cloud task. Do not create a worktree unless the user requests one.

Install the development tools listed in [CONTRIBUTING.md](../CONTRIBUTING.md#quick-start) and enable the hooks before committing. The host supplies Codex and the GitHub CLI. Don't configure credentials or regenerate OpenSpec skills as part of setup.

## Models and delegation

Codex's `spec-writer` uses `gpt-6-astra` with high reasoning effort. Its `spec-implementer` uses `gpt-6.1-terra` with medium reasoning effort. These are Codex settings; Claude Code retains its native Opus/Sonnet configuration. Role behavior should stay aligned across tools, while model names remain tool-specific.

For substantial planning requests, delegate to `spec-writer`. After the user approves implementation, delegate the approved tasks to `spec-implementer`. Handle small, well-defined edits directly. Pass the change path, scope, and acceptance criteria when delegating. If the client cannot load a custom role or access its model, report the limitation rather than silently substituting a model.

## Access checks (Codex only)

Before work that requires GitHub, test the specific capability with the environment's existing authentication:

- Git reads: `git ls-remote origin HEAD`.
- PR API reads: `gh pr view <number> --repo PandaCubGames/LearnJapaneseTui --json number,title`.
- A successful read does not prove push or PR creation permission. Never perform a write merely to test access.

Allow `github.com` for Git and `api.github.com` for API calls in the cloud network policy. Preserve inherited proxy and TLS settings. Diagnose connectivity, authentication and authorization separately. Git authentication may be injected by a proxy without a visible token. Do not replace working authentication because `GH_TOKEN` is unset or `gh auth status` is inconclusive.

Only request a missing credential after the actual operation fails and network configuration has been checked. Ask the user to configure credentials securely in environment settings; never request token values in chat, print them, or store them in this repository.

## Signed commits (Codex only)

Before creating commits or tags, check whether the existing environment provides the repository-required signing configuration and signing capability. Check configuration presence without printing key material. A configured key path alone does not prove signing works. Use the supplied signer for an authorized commit and verify its signature; do not create a test commit just to probe access.

Do not assume a local workstation's SSH key exists in cloud sessions. If signing is unavailable, finish edits and validation, leave the changes uncommitted, and report that signing must be configured securely or the owner must commit them from a configured machine. Never disable signing, generate a replacement identity, or copy a private key into the checkout.

Repository approval and owner-only merge rules in AGENTS.md continue to apply.
