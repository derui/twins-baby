---
name: bevy-test
description: Guidance for writing tests in the bevy_app module with minimal Bevy app setup.
when_to_use: Use when adding or updating tests under bevy_app. Prefer the smallest possible App setup, and only use AppTester when the behavior depends on a nearly production-level app wiring.
---

Use this skill when writing tests in the `bevy_app` module.

## Rules

- Keep the test app as simple as possible.
- Start from a minimal `App` and add only the plugins, resources, events, components, and systems required by the test.
- Use `AppTester` only when the system under test requires a nearly production app setup.
- Prefer focused tests over full app bootstrapping when a smaller setup can verify the behavior.

## Heuristic

Use a plain Bevy `App` for isolated systems, resources, events, and component behavior.

Use `AppTester` when the test depends on production-like plugin wiring, schedules, startup/update flow, or other integration behavior that is hard to model safely with a tiny app.
