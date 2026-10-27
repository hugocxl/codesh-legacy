# Agents

## Repository

Before finishing, run:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

## Main Guides

Only test real scenarios and risks: data loss, security, compatibility,
concurrency and regressions. Don't test obvious code or defaults.

## Third-party dependencies

Before adding a third-party dependency, provide its GitHub URL, star count and
maintenance metrics (such as recent releases and commit activity) for the user
to approve adding it.

## Performance

Take care with CPU and memory use. GPUI changes can easily cause high resource
usage; consider redraws, subscriptions and retained state.

## Codesh AI policy

In this fork, AI agents may commit and push on behalf of the owner when asked.
Follow the owner's instructions for each task, including any prohibition on
committing or pushing.

Upstream's humans-only policy applies to anything sent to `muxy-app/muxy`:
only humans may commit and push, open issues or pull requests, or interact with
issues and pull requests there. Humans must write those messages themselves,
without AI assistance. Agents may read upstream issues and pull requests.

## Fork rules

- `main` mirrors upstream and is never edited directly; it only receives
  fast-forward upstream updates. Fork work lands on `codesh`; branch tasks from
  `codesh` and integrate them there.
- Keep patches merge-friendly: prefer new files under `crates/*/src/codesh*`
  and `docs/codesh/`, with only few-line hooks into upstream code.
- Never change `muxy-protocol`.
- Keep `muxy` crate and binary names.

Enable rerere and sync upstream as follows. Replace `<upstream tag>` with the
chosen upstream release tag; rebasing rewrites fork history.

```bash
git config rerere.enabled true
git fetch upstream && git checkout main && git merge --ff-only upstream/main && git push origin main && git checkout codesh && git rebase <upstream tag>
```

After resolving conflicts, run the checks above, rebuild, and re-screenshot the
Codesh desktop to review the customizations. Run this procedure only when an
upstream sync is requested.
