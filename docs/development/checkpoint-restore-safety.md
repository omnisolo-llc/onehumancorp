# Local checkpoint restore safety

The local checkpoint store now restores through
`restore_checkpoint_for_thread(thread_id, checkpoint_id)`. The old one-argument
method remains source-compatible but fails with a scope-required error for the
built-in stores. Custom stores must explicitly implement the scoped method;
there is no fallback to their old unscoped restore implementation.

New Git checkpoints use a task-and-checkpoint-specific SHA256 ref under
`refs/ohc/checkpoints/`, separate from every legacy tag. Unoccupied
legacy aliases remain for read-only compatibility and are never moved over an
existing alias. Git restoration resolves the checkpoint ref to an immutable
commit and checks the stored checkpoint and task IDs before any workspace mutation.
Sanitized-tag collisions and a stale task progress file carried into another
task's commit are rejected. A legacy global tag is accepted only if its target
commit actually records that task's progress file using a literal Git pathspec.
Immutable commit-ID reads remain available for history enumeration; restore requests must name the stored checkpoint ID.

An existing Git index lock, including a linked worktree's actual index lock,
causes a retryable error. Checkpoint save and restore never remove that lock.
Ignored file/directory collisions with the destination are rejected before
stashing. Git checkout additionally uses `--no-overwrite-ignore` to protect
ignored paths created after the preflight check.

On a valid restore, dirty tracked files and untracked work are preserved with
`git stash push --include-untracked`. A stash failure aborts the restore. The
checkpoint is checked out on a fresh `agent-restore-*` branch, without resetting
an existing branch, `reset --hard`, or `clean`. Ignored files stay in place.
The saved stash commit is logged and included in subsequent checkout errors;
`git stash apply --index <stash-commit>` can recover staged and untracked work.
A clean checkout does not falsely report an older stash as a new recovery copy.

PostgreSQL restoration selects the requested `(thread_id, checkpoint_id)` in a
transaction and locks that row before pruning only that thread's newer history.
Missing or wrong-task checkpoints cannot prune another thread's records.

Agent rewind callers propagate configured-store read, decode, and restore
failures before emitting a successful rewind. Lightweight message-only rewind
remains available when no checkpoint store is configured.

## Boundaries that remain

This is a local restore safety contract, not tenant authorization. Raw/global
runtime admission remains disabled. A tenant-aware adapter still needs actor
and task authorization, isolated workspace/storage ownership, cancellation and
concurrency controls, and service-level acceptance tests before it can expose
these operations. Checkpoint creation still snapshots the configured repository;
it must be isolated. New task-scoped refs support duplicate checkpoint IDs across tasks
and IDs that would collide under legacy sanitization. This repair does not
make a shared user checkout safe as a multi-tenant workspace.

## Verification

The `scripts/checkpoint-restore-contract` suite compiles the full production
store with real private Git repositories and PostgreSQL temporary tables. The
protocol restore method is copied verbatim with minimal receiver plumbing;
that focused test is not full runtime compilation. The suite has explicit
PostgreSQL prerequisites and does not silently pass unavailable DB tests.
See its README for the command. On 2026-10-03, the 37-case focused suite and
strict Clippy passed; the actual built-in-agent test module compiled, and its
32 store plus 10 targeted caller tests passed. Full repository lint/tests,
hosted CI, UI, and tenant-runtime acceptance remain separate gates.
