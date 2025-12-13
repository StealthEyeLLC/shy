# PASS_GUARD

Project: SHY  
Owner: StealthEye LLC  
Purpose: Enforce build discipline, version freezes, and audit safety  
Status: ACTIVE

---

## CURRENT STATE

STATUS: FROZEN  
CURRENT VERSION: v0.9.0  
FREEZE LEVEL: HARD  
DATE FROZEN: 2025-12-12

---

## FREEZE RULES (HARD)

While a version is frozen, the following rules are absolute:

ALLOWED:
- Bug fixes that do NOT change public behavior
- Explicitly approved safety fixes
- Documentation updates
- Test additions or corrections
- Version bump commits

FORBIDDEN:
- New features
- API changes
- Signature changes
- Behavior changes
- Silent refactors
- Renaming public items
- Moving files that affect imports
- “While I’m here” edits

Any forbidden change REQUIRES:
- A version bump
- A new PASS_GUARD entry
- A new tag

---

## VERSION POLICY

Versioning follows semantic intent, not marketing.

- PATCH (x.y.Z)
  Bug fixes only. No behavior changes.

- MINOR (x.Y.0)
  Additive features. No breaking changes.

- MAJOR (X.0.0)
  Breaking changes, redesigns, or architectural shifts.

No version may be modified after tagging.

---

## TAGGING RULES

- Every freeze MUST be tagged
- Tags are immutable
- Reusing or overwriting tags is forbidden
- Tag messages must describe the freeze intent

Example:
