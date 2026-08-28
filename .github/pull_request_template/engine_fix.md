---
title: "engine: avoid polling completed future"
labels: bug, runtime
assignees: []
---

This PR prevents polling a completed async game future which caused the runtime panic "`async fn` resumed after completion". The GameEngine.task field is changed to an Option and is cleared when the task finishes.

Changes:
- src/engine.rs: make task an Option and set to None on completion to avoid re-polling

Closes: N/A
