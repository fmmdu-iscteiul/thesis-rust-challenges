# Challenge 2 - Snemeis Support Export Tool (CWE-703)

## Ticket Title
Snemeis support export crashes when requester email is missing

## Priority
P2 - High (reliability issue in support operations)

## Summary
The support export tool assumes each ticket has a required requester email.
When a record is missing that field, the export path crashes instead of failing safely.
It also treats malformed requester email values as acceptable export data.

## Task
Update the export logic to handle missing or malformed required ticket fields safely without panicking.
Keep valid ticket export behavior unchanged.

## Acceptance Criteria
- Valid ticket record exports successfully.
- Missing required requester email is handled safely as an error.
- Empty requester email is handled safely as an error.
- Malformed requester email is handled safely as an error.
- No panic-based control flow remains in the export path.
- Behavior is explicit and stable using safe Rust error handling.

## Files in this challenge

- `src/lib.rs` — the ticket-export logic for this challenge. **This is the file you edit.**
- `src/main.rs` — auxiliary demo runner so you can watch the scenario. `cargo run` exports a valid ticket plus records with a missing and a malformed requester email, and reports whether the export path is still VULNERABLE. Use it to see whether the code is still vulnerable.
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

> **Important Note**: Since Cargo.toml is located inside the project folder, you will need to run `cd /home/user/snm02_improper_handling` in VSC's terminal before running `sifu check`. Same thing for `cargo`commands.

## Terminal

To open a new Terminal window, you can use the shortcut CTRL + SHIFT + C
