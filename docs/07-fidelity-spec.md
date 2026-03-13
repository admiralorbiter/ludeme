# Fidelity Specification

Every published demo carries a fidelity badge. The badge is not a quality judgment — it is a transparency signal about what the demo is claiming to demonstrate.

## Fidelity levels

### Faithful

The demo makes a good-faith attempt to match the original mechanic at the level of its rules, timing values, input mapping, and fail conditions. Visual and audio presentation may be simplified, but the mechanic behavior should be indistinguishable to a player familiar with the original.

**Disqualifiers:** Changing jump arc gravity without documenting it. Using a different scoring formula. Simplifying enemy state machines in a way that changes how pressure feels. Any parameter change that would affect mastery.

**Example:** A Pong demo that matches the original ball speed range, paddle acceleration, and score-to-win value, presented with simplified graphics. Faithful.

### Interpreted

The demo is based on the original mechanic but makes at least one documented departure — a parameter chosen for clarity, a control scheme updated for modern input, a simplification that trades historical accuracy for teaching value. Departures must be noted in the demo's metadata.

**Required:** Each departure listed in the `notable_interpretations` field with a reason.

**Example:** A platform demo that uses the original jump arc physics but maps controls to WASD instead of a joystick, and increases coyote time from 2 to 6 frames for modern feel. Interpreted, with those changes documented.

### Experimental

The mechanic is the inspiration, not the source. The demo tests a hypothesis, explores a variant, or asks "what if this parameter changed." No claim of historical accuracy. May be a fork of a faithful or interpreted demo with parameter deltas, or a wholly original construction around a mechanic concept.

**Required:** A hypothesis or intent statement in the demo's metadata. Linked to the parent demo or mechanic if applicable.

**Example:** A Pong demo where friction is applied to the ball over time, testing whether a decay mechanic adds strategic depth. Experimental.

## Fidelity gate in the publish checklist

Before a demo moves from draft to review, the Studio must confirm:

- Fidelity level is set
- If Faithful: no undocumented parameter changes from known original values
- If Interpreted: at least one entry in `notable_interpretations`
- If Experimental: hypothesis or intent statement is present and linked parent exists (demo or mechanic)
