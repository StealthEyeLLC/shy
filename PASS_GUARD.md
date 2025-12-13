# PASS_GUARD — SHY Build Integrity Contract

PASS_GUARD defines the non-negotiable rules governing SHY development.
This file is authoritative. Code, commits, and releases must comply.

--------------------------------------------------
CORE PURPOSE
--------------------------------------------------

PASS_GUARD exists to ensure:

- Deterministic builds
- Reproducible history
- Zero silent regressions
- Explicit intent at every change
- No accidental complexity

If a rule is violated, the build is invalid by definition.

--------------------------------------------------
ABSOLUTE RULES
--------------------------------------------------

1. NO PARTIAL CODE
   - All code changes must be complete, buildable files.
   - No snippets, placeholders, or TODO stubs in committed code.
   - If a file is touched, it must be logically whole.

2. BUILD MUST PASS
   - `cargo test` must pass with zero failures.
   - Warnings are allowed only if intentional and understood.
   - Panics, unimplemented paths, or uncovered enums are forbidden.

3. EXPLICIT STATE HANDLING
   - All enums must be exhaustively matched.
   - No wildcard (`_`) matches for state machines.
   - Every state transition must be deliberate.

4. NO SILENT BEHAVIOR
   - Refusals must propagate explicitly.
   - Errors must surface to the caller.
   - No swallowed errors, no implicit defaults.

--------------------------------------------------
FREEZE DISCIPLINE
--------------------------------------------------

A FREEZE means:

- No architectural changes
- No semantic changes
- Only stabilization, documentation, or tagging

During a freeze:
- Version is locked
- Cargo.lock is authoritative
- Only audit-grade changes are permitted

Breaking freeze requires an explicit version bump.

--------------------------------------------------
VERSIONING RULES
--------------------------------------------------

- Versions are semantic and monotonic
- Tags are immutable once created
- If behavior changes → version changes
- If rules change → PASS_GUARD updates first

Tags represent truth, not intent.

--------------------------------------------------
GIT RULES
--------------------------------------------------

- `target/` must never be committed
- `.gitignore` must include all build artifacts
- Commits must reflect a single logical change
- Commit messages must describe *what changed and why*

Red text from git usually means:
- Files not added yet
- Ignored paths
- Or generated artifacts (expected)

This is not an error unless source files are involved.

--------------------------------------------------
ENFORCEMENT
--------------------------------------------------

If PASS_GUARD conflicts with convenience:
PASS_GUARD wins.

If PASS_GUARD conflicts with speed:
PASS_GUARD wins.

If PASS_GUARD conflicts with habit:
PASS_GUARD wins.

--------------------------------------------------
FINAL AUTHORITY
--------------------------------------------------

PASS_GUARD is frozen unless explicitly versioned.
All contributors are bound by this contract.

SHY integrity depends on it.
