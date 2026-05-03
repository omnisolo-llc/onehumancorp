#!/bin/bash
# Serve WASM frontend on port 5000.
# Immediately serves a placeholder, then builds WASM in background.
# Node.js server reads files from disk on each request, so refreshing
# the browser after the build finishes will show the app.

DIST_DIR="/tmp/ohc-dist"
WORKSPACE="$(cd "$(dirname "$0")/.." && pwd)"
NODE=$(command -v node 2>/dev/null || echo "node")

mkdir -p "$DIST_DIR"

cat > "$DIST_DIR/index.html" << 'HTML'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <title>OHC - Building...</title>
  <style>
    body { background:#000; color:#eee; font-family:monospace;
           display:flex; align-items:center; justify-content:center;
           height:100vh; margin:0; flex-direction:column; gap:12px; }
    .spinner { border:3px solid #333; border-top:3px solid #aef;
               border-radius:50%; width:32px; height:32px;
               animation:spin 1s linear infinite; }
    @keyframes spin { to { transform:rotate(360deg); } }
  </style>
  <meta http-equiv="refresh" content="8">
</head>
<body>
  <div class="spinner"></div>
  <p>Building WASM frontend &mdash; this takes a few minutes on first run.</p>
  <p style="color:#777;font-size:0.85em">Page auto-refreshes every 8 seconds.</p>
</body>
</html>
HTML

echo "[serve-wasm] Starting HTTP server on port 5000..."
"$NODE" "$WORKSPACE/scripts/static-server.js" "$DIST_DIR" &
SERVER_PID=$!

echo "[serve-wasm] Server PID=$SERVER_PID. Starting WASM build in background..."
"$WORKSPACE/scripts/build-wasm.sh" 2>&1 &
BUILD_PID=$!

wait $BUILD_PID
BUILD_EXIT=$?
if [ $BUILD_EXIT -eq 0 ]; then
    echo "[serve-wasm] WASM build succeeded. Refresh browser to see the app."
else
    echo "[serve-wasm] WASM build FAILED (exit $BUILD_EXIT). Check logs above."
    cat > "$DIST_DIR/index.html" << 'HTML'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <title>OHC - Build Failed</title>
  <style>
    body { background:#000; color:#f88; font-family:monospace;
           display:flex; align-items:center; justify-content:center;
           height:100vh; margin:0; flex-direction:column; gap:12px; }
  </style>
</head>
<body>
  <p>&#10007; WASM build failed &mdash; check workflow logs for details.</p>
</body>
</html>
HTML
fi

wait $SERVER_PID
