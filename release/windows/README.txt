OmniSolo Windows portable

Quick start

1. Extract the zip into a writable directory.
2. Run run-omnisolo.cmd.
3. Open http://127.0.0.1:18789/ in a browser.

The launcher stores local standalone data in the .ohc directory next to these
files. It creates .ohc\sqlite.key on first run and reuses it on later runs.
Keep that file with .ohc\ohc-standalone.db; losing the key can make the local
database unreadable.

Executables

omnisolo-server.exe is the normal portable entrypoint. It starts the web/API server
and initializes the built-in agent runtime for local standalone use.

omnisolo-builtin-agent.exe is an advanced external agent process. You do not need to
run it for normal portable use. Run it separately only when you want an external
gRPC agent process, and set OMNISOLO_AGENT_ADDRESS to the address it should listen on.

Useful defaults

run-omnisolo.cmd sets these defaults when they are not already defined:

OMNISOLO_STANDALONE=true
STANDALONE_MODE=true
DATABASE_URL=sqlite://.ohc/ohc-standalone.db
OMNISOLO_PORT=18789
OMNISOLO_GRPC_PORT=8081
OMNISOLO_AGENT_ADDRESS=127.0.0.1:50051
