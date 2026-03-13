# Canonical Taxonomy v1

These files live in `taxonomy/` in the repository root and are seeded into the database at startup. Treat them as append-only until 50+ entries force a restructure.

## Mechanic families (`taxonomy/mechanic-families.toml`)

```toml
[[families]]
slug = "movement"
label = "Movement"
description = "How entities traverse space. Includes walking, running, jumping, flying, swimming, and screen-relative movement."

[[families]]
slug = "collision-response"
label = "Collision and Deflection"
description = "What happens when entities meet. Bounce physics, knockback, block, deflect, pass-through, and destruction."

[[families]]
slug = "scoring-pressure"
label = "Scoring and Pressure"
description = "How the game escalates tension and rewards. Points, timers, lives, combos, and thresholds."

[[families]]
slug = "state-transitions"
label = "State Transitions"
description = "Discrete mode changes in a character or system. Grounded/airborne, charge states, vulnerability windows, phase changes."

[[families]]
slug = "economy"
label = "Economy and Resources"
description = "Acquisition, spending, scarcity, and flow of any resource. Ammo, currency, health, fuel, time."

[[families]]
slug = "timing-windows"
label = "Timing and Rhythm"
description = "Player actions that are only valid or optimal within a time window. Parry, just-frame inputs, rhythmic beats."

[[families]]
slug = "spatial-rules"
label = "Spatial Rules"
description = "Rules about where entities can be. Screen wrap, bounded zones, grid constraints, proximity triggers."

[[families]]
slug = "ai-behavior"
label = "AI and NPC Behavior"
description = "How non-player entities make decisions. Patrol, chase, flee, predict, swarm."

[[families]]
slug = "progression"
label = "Progression and Unlock"
description = "How the game changes as the player advances. Difficulty curves, new mechanics introduced, gates and unlocks."

[[families]]
slug = "information"
label = "Information and Visibility"
description = "What the player knows, when they know it, and what is hidden. Fog of war, tells, reveals, deception."
```

## Relation types (`taxonomy/relation-types.toml`)

```toml
[[relations]]
slug = "variant-of"
label = "Variant of"
description = "A recognizable version of the source with at least one deliberate change."
directed = true

[[relations]]
slug = "derived-from"
label = "Derived from"
description = "Takes the source as a direct starting point, with meaningful transformation."
directed = true

[[relations]]
slug = "influenced-by"
label = "Influenced by"
description = "Shares concepts or feel without direct derivation. Looser lineage claim."
directed = true

[[relations]]
slug = "simplifies"
label = "Simplifies"
description = "Reduces the source mechanic to a smaller rule set, often for accessibility or scope."
directed = true

[[relations]]
slug = "amplifies"
label = "Amplifies"
description = "Intensifies or extends the source mechanic, often increasing complexity or expressiveness."
directed = true

[[relations]]
slug = "replaces"
label = "Replaces"
description = "The source mechanic was superseded by this one in later works by the same team or lineage."
directed = true

[[relations]]
slug = "discarded-after"
label = "Discarded after"
description = "The mechanic appears and then largely disappears from the canon. Not replaced, just abandoned."
directed = true

[[relations]]
slug = "resurfaces-in"
label = "Resurfaces in"
description = "A mechanic that was discarded reappears in a later work, often with fresh framing."
directed = true

[[relations]]
slug = "contrasted-with"
label = "Contrasted with"
description = "Useful for comparison but not in the same lineage. Highlights design differences."
directed = false

[[relations]]
slug = "co-occurs-with"
label = "Co-occurs with"
description = "These mechanics commonly appear together in the same works."
directed = false

[[relations]]
slug = "prerequisite-for"
label = "Prerequisite for"
description = "Understanding or mastering the source is required to appreciate the target."
directed = true

[[relations]]
slug = "commonly-paired-with"
label = "Commonly paired with"
description = "Not a lineage claim, but a pattern of co-design."
directed = false
```

## Confidence levels

Used on Observations, RelationshipEdges, and Source entries.

| Level | Meaning |
|---|---|
| `speculative` | An educated guess. Should not be cited as settled. |
| `tentative` | Reasonable inference from limited evidence. Subject to revision. |
| `supported` | Backed by at least one credible source or reproducible observation. |
| `established` | Multiple independent sources agree. Safe to use as a foundation claim. |
