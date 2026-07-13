---
name: proto-raw-binary-rename
description: proto renames non-archive (raw binary) downloads to the plugin ID on disk — locate_executables must derive the exe name from get_plugin_id()
metadata:
  type: project
---

proto (moonrepo) renames a raw, non-archive prebuilt download to the **plugin ID** when moving it into the install dir (`tool.get_file_name()` in `crates/core/src/flow/install.rs`). The user-chosen ID from `proto plugin add <id> ...` decides the on-disk filename, e.g. `claude-code.exe`.

**Why:** Claude Code ships as a raw binary (no archive), so hardcoding any exe name in `locate_executables` breaks whenever the plugin ID differs. This caused the Windows "Unable to symlink binary" / `missing_alternate_binary` regression shipped in v0.2.0–v0.3.0 (PR #1 renamed the exe to `claude` on a wrong premise).

**How to apply:** In `locate_executables`, always use `get_plugin_id()` + `env.os.get_exe_name(&id)` and `ExecutableConfig::new_primary` — same pattern as moonrepo's own `moon` plugin (tools/moon in moonrepo/plugins), which documents this exact behavior in a comment. Never hardcode the exe filename for raw-binary tools.
