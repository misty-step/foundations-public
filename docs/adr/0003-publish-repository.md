# ADR-0003: Publish the repository under the MIT License

Status: Proposed (draft PR; accepted when the maintainers merge it and change repository visibility).

## Context

[ADR-0001](0001-canonical-foundations.md) placed Foundations in a private repository because the deployed site was the reference. The site is already public; its Markdown sources are public at the same address. Publishing the repository adds the program, tests and decision records, and free hosted CI.

## Decision

Publish `misty-step/foundations` publicly under the MIT License ("Copyright (c) 2026 Misty Step"), matching Misty Step's other public repositories. Before the change: no secrets or internal hostnames in the tree or history, CI on GitHub-hosted runners only, secret scanning and push protection enabled, and fork pull requests require maintainer approval and receive no secrets. Outside pull requests are not merged.

## Consequences

- Git history is public once visibility changes. Publish from a fresh, squashed history rather than the existing one, which carries personal author addresses and internal host references.
- Internal operating procedures stay out of this repository.
