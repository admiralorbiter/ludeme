# UX Flows

These are the primary user journeys. Each should be verified to work before the phase it belongs to closes.

## Play-to-note flow (Phase 1 exit criterion)

1. User arrives at a demo page from home, search, or mechanic page.
2. Demo launches in the unified play shell.
3. User plays. State machine overlay shows current states (if implemented).
4. User presses bookmark shortcut or clicks the capture panel.
5. Shell pauses the session. Canvas screenshot is taken. `MomentEmit` event fires.
6. A bookmark form slides in: player label, mechanic tag suggestions from `auto_tags`, link-to option.
7. User confirms. Bookmark is saved. Session resumes.
8. From the bookmark, user can open a note form to promote the moment to a structured Observation.
9. From the note, user can open an experiment brief with param delta pre-populated.

## Mechanic-to-compare flow (Phase 2 exit criterion)

1. User arrives at a Mechanic page (from search, trail, or linked demo).
2. Page shows definition, state graph, example demos across eras, lineage edges.
3. User clicks "Compare" on two demos listed on the mechanic page.
4. Compare Lab opens with both demos selected.
5. User chooses static screenshot comparison or dual-instance live play.
6. Comparison prompts appear. User adds dimension notes.
7. User saves a Comparison entity. It links back to the mechanic page.
8. User can promote comparison notes to Observations.

## Research-to-play flow

1. User arrives at a Work page from timeline, search, or collection.
2. Page shows historical context, notable constraints, related mechanics.
3. User clicks a mechanic chip. Mechanic page opens.
4. User clicks a linked demo on the mechanic page.
5. Demo launches in the play shell with the mechanic's context panel open.
6. Breadcrumb shows: Work > Mechanic > Demo.

## Experiment flow

1. User is playing a demo. They open the parameter tuner and drag a slider.
2. They feel a meaningful difference and want to formalize it.
3. They click "Save as Experiment" in the tuner panel.
4. An Experiment form pre-populates: parent demo, param delta (automatically from the tuner log), branch ID.
5. User adds a hypothesis and expected effect.
6. Experiment is saved as a draft. The modified param set becomes a named branch.
7. The branch is playable from the demo page's branch switcher.
8. After further play, user returns to the experiment and adds observed result, then marks keep/discard/revisit.
