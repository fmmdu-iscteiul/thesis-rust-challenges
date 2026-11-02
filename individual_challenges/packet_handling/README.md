# Integer Overflow Challenge (CWE-190)

`process_client_packet` models a CVE-2018-10933-inspired auth bypass scenario where integer overflow in packet framing logic helps an attacker push a forbidden auth state transition.

## Vulnerability Story

The vulnerable handler computes a frame size using narrowing + wrapping arithmetic.
A very large payload length can be reduced to a small value after wrapping, which can pass packet-size checks.
If the attacker then sends a client-side `USERAUTH_SUCCESS` message, the vulnerable code incorrectly marks the session as authenticated.

This combines:

- `CWE-190`: Integer Overflow or Wraparound
- Auth state confusion similar in spirit to CVE-2018-10933 (client-forged success message accepted by server)

## Goal for Learners

Understand how integer overflow in packet-size handling can enable authentication state abuse in protocol code.

In the intentionally vulnerable state, auth-bypass tests fail.

## Files in this challenge

- `src/auth_challenge.rs` — the packet-handling logic for this challenge. **This is the file you edit.**
- `src/main.rs` — auxiliary demo runner so you can watch the scenario. `cargo run` sends a normal auth request and then a forged `USERAUTH_SUCCESS` packet, and prints whether the attacker ended up authenticated. Use it to see whether the code is still vulnerable.
- `src/lib.rs` — glue that re-exports the challenge items as the `challenge_project` crate so the tests can reach them. You normally don't touch it.
- `tests/validation_tests.rs` — the security and regression tests that decide whether your fix is correct. Run them yourself with `cargo test`.
- `Cargo.toml` / `Cargo.lock` — crate manifest and locked dependencies.

Completion artifacts (such as proposed solution and vulnerability explanation) stay hidden until you finish the challenge.

## Platform commands

- `sifu goal` — show the challenge objective.
- `sifu check` — run the automated verification (clippy/audit, a compile gate, then the security + regression tests). On success it reveals the flag.
- `sifu status` — show your current progress: which check categories pass/fail, tests passed/failed, hints used, and the recommended next step.
- `sifu hint` — reveal the next hint (3 available).
- `sifu send <flag>` — submit the flag from a passing `sifu check`. Completing the challenge releases the completion artifacts.

> **Important Note**: Since Cargo.toml is located inside the project folder, you will need to run `cd /home/user/rust_integer` in VSC's terminal before running `sifu check`. Same thing for `cargo`commands.

## Validation

Run tests from this crate:

```bash
cargo test -- --nocapture
```

## Terminal

To open a new Terminal window, you can use the shortcut CTRL + SHIFT + C