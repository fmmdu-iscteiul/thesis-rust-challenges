# Sandbox Escape via Out-of-Bounds Memory Access

## Challenge

A guest is supposed to access only its own sandboxed memory region.

Right now, the code does not enforce that boundary correctly.

Your task is to fix the vulnerable memory access logic so that:
- valid accesses still work
- invalid accesses are rejected
- all tests pass

## Weakness classes

- CWE-125: Out-of-bounds Read
- CWE-787: Out-of-bounds Write

## Background

This challenge is inspired by **CVE-2026-34987**.

It is a simplified educational exercise based on the same core idea:
untrusted memory access must stay within sandbox boundaries.

## Files in this challenge

- `src/lib.rs` — the guest memory-access logic for this challenge. **This is the file you edit.**
- `src/sandbox.rs` — supporting `Sandbox` type and memory layout (protected host prefix, guest region, protected host suffix). Setup code, not the thing to change.
- `src/main.rs` — auxiliary demo runner so you can watch the scenario. `cargo run` performs safe accesses and cross-boundary accesses and prints whether the sandbox leaked or overwrote host memory. Use it to see whether the code is still vulnerable.
- `tests/validation_tests.rs` — the boundary tests that decide whether your fix is correct. Run them yourself with `cargo test`.
- `Cargo.toml` / `Cargo.lock` — crate manifest and locked dependencies.

Completion artifacts (such as proposed solution and vulnerability explanation) stay hidden until you finish the challenge.

## Platform commands

- `sifu goal` — show the challenge objective.
- `sifu check` — run the automated verification (clippy/audit, a compile gate, then the security + regression tests). On success it reveals the flag.
- `sifu status` — show your current progress: which check categories pass/fail, tests passed/failed, hints used, and the recommended next step.
- `sifu hint` — reveal the next hint (3 available).
- `sifu send <flag>` — submit the flag from a passing `sifu check`. Completing the challenge releases the completion artifacts.

> **Important Note**: Since Cargo.toml is located inside the project folder, you will need to run `cd /home/user/sandbox_breach` in VSC's terminal before running `sifu check`. Same thing for `cargo`commands.

## What to run

Run the demo:

```bash
cargo run
```

Run the tests:

```bash
cargo test
```

The project starts in a vulnerable state, so some tests will fail initially.
In the starter version, the boundary-blocking tests are expected to fail until
the bug is fixed. Because the vulnerable code performs raw-pointer access from a
guest-controlled derived address, some demo runs may expose host-adjacent data
while others may crash outright.

## Terminal

To open a new Terminal window, you can use the shortcut CTRL + SHIFT + C