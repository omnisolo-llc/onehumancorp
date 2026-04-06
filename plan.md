Ah, I see! `broadcastFuncs` is defined *inside* the `if tm.db.IsSQLite() { ... } else { ... }` block, but it's used *outside* that block after `tx.Commit(ctx)`!
I need to declare `var broadcastFuncs []func()` outside the if/else block!

Let's fix this in `srcs/server/orchestration/tasks.go`!
