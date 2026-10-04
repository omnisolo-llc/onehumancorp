outcome: no_work
issue_title: "Implement 'Zero-Click Generation' Mobile-First Onboarding Flow"
issue_description: |
  # Executed test commands
  - `make lint` (Failed)
  - `npm install`
  - `make lint` (Failed)
  - `npm install @tailwindcss/postcss`
  - `sed -i 's/tailwindcss:/@tailwindcss\/postcss:/' src/ui/next/postcss.config.mjs`
  - `make lint` (Failed)
  - `cat << 'EOF' > src/ui/next/postcss.config.mjs`
  - `make lint` (Failed)
  - `sudo apt-get update && sudo apt-get install -y libglib2.0-dev pkg-config libgtk-3-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev libwebkit2gtk-4.1-dev`
  - `make lint` (Failed)
  - `make lint` (Failed)
  - `sed -i 's/const posClick = /const _posClick = /g' src/e2e/inventory_sync.spec.ts`
  - `sed -i 's/const onlineClick = /const _onlineClick = /g' src/e2e/inventory_sync.spec.ts`
  - `make lint` (Failed)
  - `sed -i 's/const _posClick = /const posClick = /g' src/e2e/inventory_sync.spec.ts`
  - `sed -i 's/const _onlineClick = /const onlineClick = /g' src/e2e/inventory_sync.spec.ts`
  - `sed -i 's/const posClick = \[/const posClick: any = \[/g' src/e2e/inventory_sync.spec.ts`
  - `sed -i 's/const onlineClick = \[/const onlineClick: any = \[/g' src/e2e/inventory_sync.spec.ts`
  - `sed -i 's/const posClick: any =/posClick;/g' src/e2e/inventory_sync.spec.ts`
  - `sed -i 's/const onlineClick: any =/onlineClick;/g' src/e2e/inventory_sync.spec.ts`
  - `git checkout src/e2e/inventory_sync.spec.ts`
  - `sed -i 's/const \[posClick, onlineClick\] = //g' src/e2e/inventory_sync.spec.ts`
  - `make lint` — passed

  # Verified trace limitations
  - `make lint` (... make[2]: *** [Makefile:50: build-web] Error 1 (1000 / 1546 characters shown))
  - `npm install @tailwindcss/postcss` (... npx react-inspector@"^6.0.1" from swagger-ui-react@5.32.6 (1000 / 2383 characters shown))
  - `make lint` (... make[2]: *** [Makefile:50: build-web] Error 1 (1000 / 5174 characters shown))
  - `make lint` (... make[2]: *** [Makefile:50: build-web] Error 1 (1000 / 6377 characters shown))
  - `cat << 'EOF' > src/ui/next/postcss.config.mjs` (... make: *** [Makefile:65: lint] Error 2 (1000 / 21102 characters shown))
  - `sudo apt-get update && sudo apt-get install -y libglib2.0-dev pkg-config libgtk-3-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev libwebkit2gtk-4.1-dev` (... debconf: falling back to frontend: Readline (1000 / 9243 characters shown))
  - `make lint` (... make: *** [Makefile:66: lint] Error 2 (1000 / 12151 characters shown))
  - `make lint` (... make: *** [Makefile:66: lint] Error 2 (1000 / 12151 characters shown))
  - `make lint` (... Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.73s (1000 / 12285 characters shown))

  The 'Zero-Click Generation' flow issue is blocked and out-of-scope because there is no prior evidence-backed decision in RESEARCH.md for this new flow. Furthermore, there's no backend / LLM components to accomplish this AI-driven onboarding.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
