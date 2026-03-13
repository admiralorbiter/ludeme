# Vision & Guardrails

## Name

**Ludeme** — from game theory: the smallest self-contained unit of play. A mechanic atom. The word names what this platform treats as first-class.

## Product thesis

Most resources about game history separate the thing you play from the thing you learn. A video archive shows footage but not feel. A wiki shows dates and facts but not the mechanic in motion. A prototype folder shows code or a clone but not lineage, context, or why the mechanic mattered. Ludeme closes that gap.

The atomic unit of discovery is the mechanic, not the game title. Demos are small and sharply scoped. Research notes are structured and linked. Everything is addressable and cross-referenced. A live moment in play can become a bookmark, observation, comparison, or experiment in two or three actions.

## Core design principles

1. **Play first.** The platform stays grounded in interaction, not drift into a detached wiki.
2. **Mechanics are first-class entities.** Games matter, but the atomic unit of discovery is the mechanic, rule, or pattern.
3. **Everything linkable.** A demo, note, moment, source, experiment, and collection should all be addressable and cross-linked.
4. **Transparent fidelity.** Label each demo as faithful, interpreted, or experimental so users know what they are learning from.
5. **Small artifacts, deep metadata.** Keep demos tiny, but make their tags, relations, notes, and comparisons rich.
6. **Nonlinear growth.** The system works whether you build by chronology, platform cluster, or mechanic family.
7. **One shell, many modes.** Discovery, play, research, and authoring are connected rooms in the same house.
8. **Remove fields before you remove relationships.** The platform gets its value from linked structure more than long-form prose.

## Non-goals for v1

- Not a ROM archive or emulator front-end.
- Not a museum-grade historical database before the play-and-research loop works.
- Not a full-game remake platform. Small demos and sharply scoped slices win over complete clones.
- Not a social network. Public sharing comes after the core system is coherent.

---

## Guardrails

These are the rules that keep Ludeme integrated as it grows. Treat violations as technical debt, not just style issues.

- No published demo without mechanic tags, a fidelity badge, and at least one route to related context.
- No published Observation without linked evidence, even if the evidence is just a timestamped MomentBookmark.
- No new feature that only benefits one mode unless it obviously strengthens the overall play-to-research loop.
- Do not build advanced graph or comparison features on top of weak metadata. Data quality comes first.
- Favor reusable shells, templates, and taxonomies over custom page logic for one famous game.
- Keep full-game ambition in check. Small demos plus rich relationships are more scalable and more teachable.
- The taxonomy lives in version control. Do not add families or relation types directly to the database.
- The shell API contract is the boundary between game code and platform code. Do not cross it.
- Every entity has a publish state. Nothing reaches a public surface without passing the release rubric.

---

*Ludeme — build the system first. Once the grammar, shell, and authoring model are stable, the content can expand in any direction without losing coherence.*
