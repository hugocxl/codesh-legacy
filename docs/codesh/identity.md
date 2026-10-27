# Codesh desktop identity

The desktop app installs as **Codesh Beta** (`com.hugocorta.codesh-beta`)
or **Codesh** (`com.hugocorta.codesh`). Local builds use the beta app identity.
Profiles are separate from Muxy: `Codesh Dev`, `Codesh Beta`, and `Codesh` in
macOS Application Support, or `codesh-dev`, `codesh-beta`, and `codesh` under
Linux's state directory. `MUXY_DIR` still overrides the profile location.
macOS runtime leases live under `Codesh Runtime`.

Desktop English fallback text substitutes Codesh for Muxy at runtime, leaving
the source literals and English catalog intact. References to `Muxy 1.x`,
`Muxy 2`, and the mobile `Muxy app` are preserved. Existing icon assets remain.

Updates and remote helper installation use `hugocxl/codesh` GitHub releases.
Mac disk images are named `Codesh-{version}-{arch}.dmg`. The beta feed remains
`beta-2.x/update.json`; stable uses the latest release's `update.json`.
Linux CLI/server and desktop artifacts keep their existing names. Releases no
longer publish Sparkle appcasts or the mobile SDK, notify Discord, or update
Muxy's Homebrew tap.

Internal crate names, `muxy` / `muxy-server` / `muxy-app` executables, environment
variables, socket names, protocol compatibility, the `MuxyVersion` plist key,
and `muxy://pair` links are unchanged to simplify upstream merges. Theme keys,
User-Agent strings, legacy import paths, and mobile pairing names also remain.

Signing supports Developer ID Application and Apple Development certificates.
Automatic updates still verify the installed app's team and Codesh bundle ID;
ad-hoc signatures cannot use automatic updates. Notarization requires Developer
ID and notary credentials. CI only notarizes and checks notarization when all
three Apple notary secrets are present. Local `build-install.py` also supports a
notary keychain profile; without credentials it installs the signed app without
stapler validation. `--build-only` signs without notarizing or installing.
