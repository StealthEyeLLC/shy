# PASS GUARD

## Current Pass
PASS 2A — Public Surface Lock

## Authority
This file exists to prevent pass mixing.
If an action is not explicitly allowed in the current pass, it is forbidden.

---

## Allowed
- Public structs
- Public enums
- Public traits
- Public function signatures
- Re-exports from crate root (`lib.rs`)
- Tests that import ONLY crate roots
- Function bodies containing ONLY:
  - `todo!()`
  - `unimplemented!()`

---

## Forbidden
- Real logic or behavior
- Fixing warnings caused by unused parameters
- Internal module imports in tests
- Cross-crate behavior assumptions
- Renaming public items once depended upon
- Moving files or modules
- Adding or changing dependencies
- Structural changes of any kind

---

## Exit Conditions
- `cargo check` passes
- `cargo test` compiles (runtime failures allowed)

---

## Freeze Rules
- The moment another crate depends on a public item, it is frozen
- Frozen items may not be renamed, moved, or have signatures changed
- Changes require a version bump and re-entry into the pass system

---

## Notes
- APIs describe contract
- Tests describe intent
- Behavior comes later
- Warnings are acceptable in PASS 2A
- If unsure, STOP and re-evaluate the pass
