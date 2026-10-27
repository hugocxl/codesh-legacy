# Codesh settings

Keyboard shortcuts form one inset-separated card, rendered through the existing
virtual list. Shortcut labels use sentence case and New Terminal matches the
single-tab menus. macOS bindings use key glyphs; Reset appears only when the
effective binding differs from its default. Recording, cancellation and conflict
validation use the existing keymap path.

Appearance uses the shared slider for sidebar vibrancy level (0–100), saving on
release or Settings closure. Status bar visibility, collapsed sidebar style and
tips are excluded from the visible catalog and search because Codesh ignores
them. Their persisted appearance fields remain readable.

Composer and Quick Terminal have been removed from the desktop app, settings,
menus and shortcuts. Standalone dictation also sends directly to a terminal,
so it and its native backends remain. Its preferences now live in General →
Voice and the independent [voice] settings table. Existing voice preferences
are migrated from the old [composer] table when [voice] is absent. Legacy top-level sections in settings.toml and
terminal.toml are ignored; active settings still undergo value validation.
Retired shortcut overrides are ignored, and new settings serialization omits the
removed sections. Legacy imports report removed options as unsupported. Existing
profile draft files are not deleted. Mobile and protocol data are unchanged.

The desktop retains native accessibility checks for terminal vibrancy, and 1.x
shortcut modifier decoding for other imported shortcuts.
