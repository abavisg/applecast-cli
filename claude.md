# 🤖 Claude Code Configuration

Defines how Claude Code should plan, implement, and test each slice.

---

## 🎯 Purpose

To ensure Claude Code works incrementally, safely, and consistently across all slices using TDD and BDD practices, while keeping output tone and behaviour consistent with shared specs.

---

## 🧩 Workflow (per slice)

1. Start each slice in **Plan Mode**.
   - Read all relevant specs.
   - Summarise what will be built.
   - List files, structs, and tests.
   - Wait for confirmation before coding.

2. On approval, enter **Build Mode**.
   - Write failing tests first (TDD).
   - Implement minimal code to pass.
   - Refactor if needed.

3. When all tests pass:
   - Update `docs/BUILD_LOG.md` with slice summary and commit message.
   - Append any new CLI usage examples to `README.md` (user-facing only).

---

## 🧭 Context Setup

Before starting any slice, Claude must always load the following files:

1. `/claude.md` → global workflow and engineering rules  
2. `/specs/SLICES/<active_slice>.md` → current feature definition  
3. `/specs/BEHAVIOURS.md` → shared behaviour and mood framework  
4. `/specs/TEST_GUIDELINES.md` → test structure, naming, and BDD style  

**Purpose:**  
To ensure every slice uses consistent tone, mood transitions, stat rules, and test structure.

**Session Start Command (for Giorgos to paste in Claude Code):**

```
Read claude.md, specs/SLICES/<active_slice>.md,
specs/BEHAVIOURS.md, and specs/TEST_GUIDELINES.md.
Summarise the slice goal, mood dependencies, and test requirements.
List planned files, structs, and test names.
Do not write code yet — stay in Plan Mode until confirmation.
```

---

### 🧹 Context Reset Policy

After each slice is completed and all tests are passing:

1. Update `docs/BUILD_LOG.md` with a summary and commit message.  
2. Commit your changes (`git add . && git commit -m "feat: complete Slice 0X"`).  
3. **Reset Claude Code context** to free memory and focus on the next slice.  
4. Reload project context by running:
   ```
   Read claude.md, specs/SLICES/<next_slice>.md,
   specs/BEHAVIOURS.md, and specs/TEST_GUIDELINES.md.
   Summarise the slice goal and plan in detail.
   Do not write code yet — stay in Plan Mode until confirmation.
   ```

This ensures each slice starts clean, reduces confusion, and keeps Claude Code operating efficiently.

---

## 🧪 Test-Driven Development Rules

- Always write failing tests before implementation.  
- No skipping to implementation even if trivial.  
- Follow Given/When/Then structure from `TEST_GUIDELINES.md`.  
- Tests must describe intent, not mechanics.

Example:
```
test_feed_increases_hunger_and_happiness()
```

---

## 🧠 Best Practices

| Rule | Description |
|------|--------------|
| **Ask before major changes** | Confirm file deletions, refactors, or design shifts. |
| **Follow TDD strictly** | Never implement without a failing test first. |
| **Document success** | When slice is complete, update `docs/BUILD_LOG.md`. |
| **Stay incremental** | Never jump ahead — one slice at a time. |
| **Keep tests isolated** | Use temp files or mocks, no shared state. |
| **Format & lint** | Run `cargo fmt` and `cargo clippy` before marking done. |

---

## 🧾 Deliverables per Slice

Each slice should produce:

- **Code** – minimal, tested feature implementation  
- **Tests** – all green, written first  
- **README Update** – only user-facing usage or example changes  
- **Build Log Entry** – date, summary, commit message, slice reference  

Example build log entry:

```
✅ Slice 01 – Persistence Layer complete
- Added pet save/load logic (serde)
- Created tests for file creation, load, error recovery
- Commit: feat: add persistence layer and tests
```

---

## ✅ Definition of Done

A slice is considered complete when:

1. All tests are passing.  
2. Code reviewed (self or peer).  
3. `docs/BUILD_LOG.md` updated.  
4. README user instructions updated if relevant.  
5. Next slice spec ready to begin.

---

## 🚫 Do Not

- Skip Plan Mode.
- Modify more than one slice at once.
- Implement CLI or REPL before its slice.
- Add new crates without asking for confirmation.

---

## 🧭 Author Notes

Created by Giorgos Ampavis  

