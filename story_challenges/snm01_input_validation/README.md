# Challenge 1 - Snemeis Employee Login Helper (CWE-20)

## Ticket Title
Snemeis login helper crashes on malformed employee ID input

## Priority
P2 - High (user-facing reliability and security hygiene)

## Summary
The employee login helper currently assumes employee IDs are always valid numbers. Malformed input causes a panic and crashes request handling.

## Task
Update the employee ID parsing flow so untrusted input is handled safely without panicking. Keep valid numeric behavior unchanged, and make malformed input follow an explicit failure path instead of silently mapping to a fallback ID.

## Acceptance Criteria
- Valid numeric input is accepted and parsed.
- Alphabetic input is rejected safely with an explicit error.
- Empty input is rejected safely with an explicit error.
- Malformed mixed input (example: 12x7) is rejected safely with an explicit error.
- Optional: trimmed numeric input (example: " 1042 ") is accepted.
- No panic-prone parsing remains in the login helper path.
- The fix must not silently substitute a fallback ID such as `0` for invalid input.

## Files in this challenge

- `src/lib.rs` — the login-helper logic for this challenge. **This is the file you edit.**
- `src/main.rs` — auxiliary demo runner so you can watch the scenario. `cargo run` simulates several login attempts (valid, trimmed, alphabetic, empty, mixed) and prints which inputs are handled safely and which crash. Use it to see whether the code is still vulnerable.
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

> **Important Note**: Since Cargo.toml is located inside the project folder, you will need to run `cd /home/user/snm01_input_validation` in VSC's terminal before running `sifu check`. Same thing for `cargo`commands.

## Terminal

To open a new Terminal window, you can use the shortcut CTRL + SHIFT + C