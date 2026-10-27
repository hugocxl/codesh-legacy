# Building on macOS 26/27

Use Rust from `rust-toolchain.toml`, Zig 0.15.2, and Xcode. On macOS 26/27,
Zig 0.15.2 can fail to link Ghostty against the installed SDK with errors such
as `undefined symbol: _abort`.

The macOS 27 SDK's `usr/lib/libSystem.tbd` lists only `arm64e` variants for ARM
(no plain `arm64`; it may also list x86_64 targets). Zig 0.15.2's native ARM
linker cannot use those exports. The workaround is a minimal fake SDK for Zig:

- Symlink `System`, `Library` and `SDKSettings.*` from the real SDK.
- Copy Zig's `lib/libc/darwin/libSystem.tbd` into `usr/lib/libSystem.tbd`.
- Populate `usr/include` with symlinks to Zig's bundled Darwin headers under
  `lib/libc/include/any-macos-any` and any architecture-specific directories
  such as `aarch64-macos-any` or `x86_64-macos-any` present in that distribution.

A `zig` wrapper prepends a private directory containing a fake `xcrun` whose
`--show-sdk-path` returns this SDK. Expose **only the zig wrapper** through a
separate directory containing just its symlink. Never put the fake `xcrun` on
the calling shell's PATH or set `SDKROOT` to the fake SDK: Cargo, Clang and
bindgen need the real SDK and its full headers. `SDKROOT` stays real, including
inside the Zig process; the fake SDK is selected through Zig's `xcrun` lookup.

## Setup

Run this in a fresh shell with the real Zig 0.15.2 on PATH. The temporary files
last for this build session; re-run after SDK or Zig upgrades. The Python code
reads Zig 0.15.2's `zig env` output to find the executable and library directory,
so a version-manager shim also works.

```bash
export SDKROOT="$(/usr/bin/xcrun --sdk macosx --show-sdk-path)"
CODESH_ZIG_WORKAROUND="$(mktemp -d /tmp/codesh-zig.XXXXXX)"
python3 - "$CODESH_ZIG_WORKAROUND" <<'PY'
import os, platform, re, shlex, shutil, subprocess, sys
from pathlib import Path

root = Path(sys.argv[1])
env = subprocess.check_output(["zig", "env"], text=True)
zig = Path(re.search(r'\.zig_exe = "([^"]+)"', env)[1])
lib = Path(re.search(r'\.lib_dir = "([^"]+)"', env)[1])
real = Path(os.environ["SDKROOT"])
sdk, private, bin_dir = root / "sdk", root / "private", root / "bin"
for directory in (sdk / "usr/lib", sdk / "usr/include", private, bin_dir):
    directory.mkdir(parents=True, exist_ok=True)
for source in [real / "System", real / "Library", *real.glob("SDKSettings.*")]:
    (sdk / source.name).symlink_to(source)
shutil.copyfile(lib / "libc/darwin/libSystem.tbd", sdk / "usr/lib/libSystem.tbd")
arch = {"arm64": "aarch64", "x86_64": "x86_64"}[platform.machine()]
for name in (f"{arch}-macos-any", "any-macos-any"):
    headers = lib / "libc/include" / name
    for source in headers.rglob("*"):
        target = sdk / "usr/include" / source.relative_to(headers)
        if source.is_dir():
            target.mkdir(parents=True, exist_ok=True)
        elif not target.exists():
            target.symlink_to(source)
quote = shlex.quote
xcrun = private / "xcrun"
xcrun.write_text('#!/bin/sh\nfor arg in "$@"; do\n'
    '  if [ "$arg" = --show-sdk-path ]; then\n'
    f'    printf "%s\\n" {quote(str(sdk))}; exit 0\n'
    '  fi\ndone\nexec /usr/bin/xcrun "$@"\n')
wrapper = root / "zig-wrapper"
wrapper.write_text(f'#!/bin/sh\nexport PATH={quote(str(private))}:"$PATH"\n'
    f'exec {quote(str(zig))} "$@"\n')
for script in (xcrun, wrapper):
    script.chmod(0o755)
(bin_dir / "zig").symlink_to(wrapper)
PY
export PATH="$CODESH_ZIG_WORKAROUND/bin:$PATH"
```

`xcrun --show-sdk-path` in your shell must still return the real SDK, and
`command -v zig` should return the temporary `bin/zig`. Keep this shell open
for the build. The wrapper also works with the release script's CPU-baseline
wrapper: the release script captures it in `MUXY_ZIG` before modifying PATH.

## Metal and build

GPUI compiles Metal shaders. Install Xcode's Metal component if it is missing:

```bash
xcodebuild -downloadComponent MetalToolchain
```

Then build from the repository root:

```bash
cargo build --workspace
# Or build and sign without installing (requires an Apple signing identity):
python3 scripts/build-install.py --build-only
```

See [Contributing](../../CONTRIBUTING.md) for run commands and required checks.
