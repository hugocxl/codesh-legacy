# Standalone dictation

Use **Toggle Voice Recording** from the View menu or command palette, or press
`Cmd+Shift+I` on macOS, to dictate into a live terminal. Finish waits for the final
transcript; Cancel discards the recording. Switching terminal or server
connection cancels the target, so text cannot reach a different terminal.

Language, offline model and Send after recording are in **Settings → General →
Voice**, saved under `[voice]` in settings.toml. Existing voice preferences move
out of the former `[composer]` table when the profile loads. Composer itself is
removed. Transcription remains on-device. See the platform requirements in
[Constraints](../tech/constraints.md).
