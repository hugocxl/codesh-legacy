//! The built-in extensions bundled by `build.rs`, installed into the profile at boot.

use std::io::Write;
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/builtin_extensions.rs"));

/// Installs or refreshes the built-in extensions; a failure leaves the
/// previous copies and only costs those extensions, so boot goes on.
pub(crate) fn install(profile: &Path) {
    if let Err(error) = muxy_app_core::extensions::builtin::seed(
        profile,
        BUILTIN_EXTENSIONS,
        BUILTIN_EXTENSIONS_HASH,
    ) {
        crate::diagnostics::event("builtin-extensions", format_args!("error={error}"));
        let _ = writeln!(
            std::io::stderr(),
            "Built-in extensions were not installed: {error}"
        );
    }
}
