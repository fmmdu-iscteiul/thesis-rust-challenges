# Challenge 3 - Snemeis Escalation Handler (CWE-20 + CWE-703)

## Ticket Title
Snemeis escalation service crashes on malformed escalation requests

## Priority
P1 - Critical (service crash caused by unvalidated external input)

## Summary
The backend escalation helper parses a semicolon-delimited string and routes
escalation requests to the correct team. The implementation assumes every
segment is well-formed, every required field is present, and every value is
the correct type. Malformed or incomplete input causes uncontrolled panics
instead of safe, reported errors.

## Task
Refactor `parse_escalation` and `handle_escalation` in `src/lib.rs` to validate
input explicitly at every fallible step. Both parsing and processing must return
controlled errors instead of panicking on bad data. Keep the happy path
unchanged.

## Acceptance Criteria
- Valid escalation request is processed and returns the correct routing record.
- A missing required field (`ticket`, `level`, or `team`) returns a safe error.
- A non-numeric value for `ticket` or `level` returns a safe error.
- A malformed segment (no `=` separator) returns a safe error.
- An unknown field returns a safe error instead of being silently ignored.
- Empty input returns a safe error.
- No call to `unwrap()` or equivalent panic-prone logic remains on any fallible path.
- All tests in `tests/validation_tests.rs` pass.

## Files in this challenge

- `src/lib.rs` — the escalation parsing and routing logic for this challenge. **This is the file you edit.**
- `src/main.rs` — auxiliary demo runner so you can watch the scenario. `cargo run` feeds valid, malformed, and unknown-field escalation strings through the handler and prints where the vulnerable version panics or silently ignores input. Use it to see whether the code is still vulnerable.
- `tests/validation_tests.rs` — the validation tests that decide whether your fix is correct. Run them yourself with `cargo test`.
- `context.txt` — the internal support ticket describing the scenario.
- `Cargo.toml` / `Cargo.lock` — crate manifest and locked dependencies.

Completion artifacts (such as proposed solution and vulnerability explanation) stay hidden until you finish the challenge.

## Platform commands

- `sifu goal` — show the challenge objective.
- `sifu check` — run the automated verification (clippy/audit, a compile gate, then the security + regression tests). On success it reveals the flag.
- `sifu status` — show your current progress: which check categories pass/fail, tests passed/failed, hints used, and the recommended next step.
- `sifu hint` — reveal the next hint (3 available).
- `sifu send <flag>` — submit the flag from a passing `sifu check`. Completing the challenge releases the completion artifacts.

> **Important Note**: Since Cargo.toml is located inside the project folder, you will need to run `cd /home/user/snm03_combined_vulns` in VSC's terminal before running `sifu check`. Same thing for `cargo`commands.

## Terminal

To open a new Terminal window, you can use the shortcut CTRL + SHIFT + C