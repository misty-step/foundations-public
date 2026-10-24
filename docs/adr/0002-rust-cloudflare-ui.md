# ADR-0002: Leptos islands on workers-rs

2026-10-16. Decision candidate; the Stack amendment requires recorded independent review before publication. Scope: Misty Step only.

## Decision

Use **Leptos CSR** for interactive Rust/WASM islands and **workers-rs** for HTTP/auth/data adapters. Keep reusable domain types/reducers independent of either runtime. Serve versioned WASM/assets and the existing HTML/CSS shell from the same Cloudflare origin; retain Access on private apps, including data and assets. Add only `wasm-unsafe-eval` to the applicable script policy, not `unsafe-eval`. Credentials remain server-side; browser reads use same-origin credentials, no redirects and visible unknown/retry states. Use the native storage matching the product's consistency needs, not every Cloudflare service by default.

Reuse Overmind's existing workers-rs backend rather than build another server framework, RPC layer or shared UI platform. Port the first Home island, then delete its JavaScript implementation. Static reference prose still renders without JavaScript as Foundations SPEC requires; record that strong exception, not a compulsory WASM download. Prefer SSR for independently useful public HTML; SSR/hydration on workers-rs is **not proven by this CSR experiment** and is not a prerequisite for the first island.

## Experiment and measured result

Both candidates implement Home Health's Voice-budget read: the same `GET /voice/budget`, health/remaining dollars/message, Refresh, unknown on failure, recovery and existing health link. Shared fetch/parser and CSS; Rust 1.98.1, wasm-bindgen 0.2.129, locked dependencies, `opt-level=z`, LTO, one codegen unit, no wasm-opt. Stable Leptos 0.8.19 versus Dioxus 0.7.7 (resolved internals 0.7.10), both CSR. Yew offers another virtual DOM without a demonstrated advantage here; plain web-sys would require handwritten lifecycle/reactivity as these panels grow. Neither was benchmarked.

| Observation | Leptos | Dioxus |
| --- | ---: | ---: |
| WASM bytes / gzip-9 bytes | 187,766 / 78,271 | 354,494 / 143,960 |
| Generated JS bytes / gzip-9 bytes, all emitted files | 32,242 / 6,513 | 86,286 / 15,439 |
| Fresh-browser WASM init p50 / p95, ms | 17.3 / 30.1 | 16.5 / 30.8 |
| Fresh navigation to observed live panel p50 / p95, ms | 171.5 / 300.0 | 149.2 / 183.3 |
| One label-edit release build + bindgen + browser acceptance, seconds | 7.30 | 8.11 |

Ten new Chromium processes per candidate on one Linux workstation, alternating 390/1440px and light/dark, nearest-rank percentiles. Asset routing supplies **local candidate bytes at the canonical origin**; actual API requests cross Cloudflare Access with the existing service identity outside the browser. Each candidate made 15 successful live reads; anonymous read returned 302. Four explicitly synthetic controls (503, Access redirect, HTML, invalid shape) each rendered unknown then recovered through a real live read. Keyboard refresh/link and no-overflow checks passed; inspected screenshots show readable controls/content. No production writes, model calls or inference spend.

The shared unchanged Overmind workers-rs bundle (`e0c5df42a4488b812bc8bd651b160c0f6124809d`) also ran in ten independent local workerd processes. Process-launch/compile-through-first-health p50/p95 was **191.3/459.9ms**; first request after ready **12.8/42.6ms**, warm **1.6/2.5ms**. These include host/process/transport overhead; **Cloudflare edge isolate cold-start CPU is unmeasured**. The CSR framework does not change that backend. Wrangler dry-run packaged 1,462.45KiB / 528.25KiB gzip; it did not deploy.

## Why and limits

Choose Leptos for substantially smaller client payload with comparable observed init time and a direct resource/signal model. Both sources compiled on the first attempt; Dioxus's generated interpreter required extra static-module routing in the harness. Single edit-loop samples are observations, not a productivity study. Sequential live timings contain API/network variance and do not establish a framework speed advantage. This isolates one panel, not whole-app size or native desktop fitness.

Neither candidate UI has been deployed behind Access. Edge-served UI, real expired-session recovery and final CSP/asset routing remain unverified; prove them in Overmind's first live deleting slice, not by extending this spike. The immediate seven-project rewrite sequence and reproducible sources, lockfile, raw samples, hashes, commands and captures are retained in the private task record (not published). No new roadmap or service is introduced here.

References: [workers-rs](https://developers.cloudflare.com/workers/languages/rust/), [Leptos CSR](https://docs.rs/leptos/0.8.19/leptos/), [Dioxus 0.7.7](https://docs.rs/dioxus/0.7.7/dioxus/), [Jev on OpenRouter](https://openrouter.ai/docs/guides/community/jev). The Stack paragraph owns vendor defaults; this ADR does not authorize paid calls or override a product's privacy/credential contract.
