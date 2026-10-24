# craft-libs patch debt (C0 fork)

Measured against upstream `main` at the rev C0vid pins. New files (`crates/media/src/isobmff/mux/c0_timing.rs`, `crates/media/tests/c0_timing.rs`, this file) are additions.

| File | Lines | What | Why | Offered upstream |
|---|---:|---|---|---|
| `crates/media/src/isobmff/mux.rs` | +1 / -0 | Declare the final presentation-window extension (`mod c0_timing;`) | C0vid trims browser AAC priming that is only known after encoding; moved here from filmcraft-isobmff when upstream moved the muxer into craft-media (2026-10-25) | no |
