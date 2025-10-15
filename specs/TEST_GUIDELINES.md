# 🧪 TEST GUIDELINES – termiPet TDD/BDD Framework

Defines how all tests are written, named, and organised for consistency.

---

## 🧩 Test Structure

### File Placement
- **Unit tests:** in the same module under `#[cfg(test)]`.
- **Integration tests:** in `/tests/` using realistic CLI invocations.

### Naming
```
test_<command>_<expected_behaviour>()
```
Example:  
`test_feed_increases_hunger_and_happiness()`

---

## 📖 BDD Style

All acceptance criteria must include at least two **Given/When/Then** scenarios per slice.

### Example Format
```markdown
### Scenario – Feeding a hungry pet
**Given** hunger = 60, happiness = 70  
**When** user runs `termipet feed`  
**Then** hunger = 80  
**And** happiness = 80  
**And** message includes "munches happily"
```

### Edge Case Scenario
```markdown
### Scenario – Feeding a full pet
**Given** hunger = 95  
**When** `termipet feed`  
**Then** hunger = 100  
**And** output includes "already full"
```

---

## 🧱 Test Categories

| Category | Purpose | Example |
|-----------|----------|----------|
| **Unit** | Core logic functions | save_pet(), calculate_mood() |
| **Integration** | Multi-module flow | adopt → status → feed |
| **CLI** | Command behaviour | `assert_cmd!` verifying printed output |
| **Persistence** | JSON load/save reliability | corrupt file handling |
| **Edge/Boundaries** | 0/100 caps, missing files, invalid inputs | hunger > 100 → cap to 100 |

---

## ⚙️ Test Data Fixtures

Use small, reusable helpers:
```rust
fn sample_pet() -> Pet {
    Pet::new("Kylo", "dog")
}
```
and temporary directories via `tempfile` for safe persistence tests.

---

## 🧾 Expectations per Slice

Each slice spec must include:
1. **3+ Scenarios** (2 happy paths, 1 edge/error)
2. **Named tests** derived from those scenarios
3. **No real file writes** in unit tests (mock or tempfile)
4. **All tests green** before moving to next slice

---

## ✅ Example (from Feed Command)

```
### Scenario – Feeding increases hunger
Given a pet with hunger = 60
When user runs feed()
Then hunger = 80
And happiness = 80
And printed output includes "munches happily"
```

---

## 🧪 Testing Workflow with Claude Code

1. Start each slice in **Plan Mode**.
2. Identify test names from BDD scenarios.
3. Write failing tests first.
4. Implement minimal code to pass.
5. Run `cargo test` and confirm green.
6. Update `docs/BUILD_LOG.md` with test summary.

---

## 🧠 Tips for Reliable Tests

- Use `assert_eq!` for numeric stats and `assert!(output.contains(...))` for text.
- Use `tempfile::NamedTempFile` for persistence tests.
- Avoid randomness in tests — inject seeded RNG or deterministic mocks.
- Keep tests <20 lines; group by behaviour.
- Prefer descriptive messages over generic ones (no "it works").
