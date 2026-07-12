# Universal UI Shell Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make every rendered Next.js page use exactly one responsive `AppShell`, restore Tailwind utilities, and eliminate the audited overflow, hydration, and surface inconsistencies.

**Architecture:** Keep `AppShell` as the only shell implementation and introduce a pure route registry that assigns each page to either its existing page-owned shell or the root `ProductShellGuard`. Restore the repository's Tailwind 3 pipeline, enforce the shell contract in real Chromium at desktop and mobile widths, and correct shared CSS and PowerSync initialization at their sources.

**Tech Stack:** Next.js 14, React 18, TypeScript, Tailwind CSS 3, PostCSS, Vitest, Testing Library, Playwright, pnpm/npm lockfiles, Bazel.

---

## File Map

- Create `src/ui/next/scripts/assert-tailwind-pipeline.mjs`: deterministic dependency/configuration regression check.
- Modify `src/ui/next/package.json`: remove the Tailwind 4 PostCSS path and expose the configuration check.
- Modify `package-lock.json`, `pnpm-lock.yaml`, `src/ui/next/package-lock.json`, `src/ui/next/pnpm-lock.yaml`: keep all supported dependency graphs consistent.
- Modify `src/ui/next/pnpm-workspace.yaml`: retain the audited lifecycle allowlist while regenerating the standalone lockfile.
- Create `src/ui/next/src/app/components/shellRoutes.ts`: pure route metadata and ownership resolution.
- Create `src/ui/next/src/app/components/shellRoutes.test.ts`: exhaustive route-resolution unit coverage.
- Modify `src/ui/next/src/app/components/ProductShellGuard.tsx`: consume the registry and wrap every non-page-owned UI route.
- Modify `src/ui/next/src/app/components/ProductShellGuard.test.tsx`: require formerly standalone pages to receive a shell without double wrapping page-owned routes.
- Modify `src/ui/next/src/app/globals.css`: enforce the shared radius, containment, and mobile layout contract.
- Modify `src/ui/next/src/app/agents/page.tsx`: contain the Expert Center within its viewport and local scrollers.
- Modify `src/ui/next/src/app/inbox/page.tsx`: reuse one stable shell loading state.
- Modify `src/ui/next/src/lib/powersync/PowerSyncProvider.tsx`: defer browser capability selection until after hydration.
- Create `src/ui/next/src/lib/powersync/PowerSyncProvider.ssr.test.tsx`: cover the hydration-stable server render.
- Modify `src/ui/next/src/lib/powersync/PowerSyncProvider.test.tsx`: retain browser capability coverage.
- Modify `src/ui/next/src/e2e/app-shell-style.spec.ts`: test the universal route and viewport matrix, computed utilities, radii, and overflow.
- Modify `src/ui/next/playwright.config.ts`: allow an opt-in system Chromium executable without changing CI defaults.
- Create `src/ui/next/scripts/visual-audit.mjs`: save route/viewport diagnostics and screenshots for manual inspection.
- Modify `docs/reports/production_agent_optimization_report.md`: record the UI audit, fixes, and exact verification evidence.

### Task 1: Restore One Tailwind/PostCSS Pipeline

**Files:**
- Create: `src/ui/next/scripts/assert-tailwind-pipeline.mjs`
- Modify: `src/ui/next/package.json`
- Modify: `package-lock.json`
- Modify: `pnpm-lock.yaml`
- Modify: `src/ui/next/package-lock.json`
- Modify: `src/ui/next/pnpm-lock.yaml`
- Modify: `src/ui/next/pnpm-workspace.yaml`

- [ ] **Step 1: Write the failing pipeline regression check**

Create `src/ui/next/scripts/assert-tailwind-pipeline.mjs`:

```js
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const packageJson = JSON.parse(await readFile(new URL('../package.json', import.meta.url), 'utf8'));
const postcssConfig = await readFile(new URL('../postcss.config.mjs', import.meta.url), 'utf8');

assert.match(packageJson.devDependencies.tailwindcss, /^\^?3\./, 'Tailwind 3 must remain the configured compiler');
assert.equal(packageJson.devDependencies['@tailwindcss/postcss'], undefined, 'Tailwind 4 PostCSS must not be installed');
assert.equal(packageJson.postcss, undefined, 'package.json must not compete with postcss.config.mjs');
assert.match(postcssConfig, /tailwindcss:\s*\{\}/, 'postcss.config.mjs must load the Tailwind 3 plugin');
process.stdout.write('Tailwind/PostCSS pipeline is coherent.\n');
```

- [ ] **Step 2: Run the check and verify it fails for the Tailwind 4 plugin**

Run:

```bash
cd src/ui/next
node scripts/assert-tailwind-pipeline.mjs
```

Expected: FAIL with `Tailwind 4 PostCSS must not be installed`.

- [ ] **Step 3: Remove the competing plugin and package-level PostCSS configuration**

Update the relevant portion of `src/ui/next/package.json` to remove `@tailwindcss/postcss` and the top-level `postcss` object, and add:

```json
{
  "scripts": {
    "test:tailwind-config": "node scripts/assert-tailwind-pipeline.mjs"
  },
  "devDependencies": {
    "autoprefixer": "^10.5.0",
    "postcss": "^8.5.15",
    "tailwindcss": "^3.4.19"
  }
}
```

Keep the existing dependency security overrides and the eight-package lifecycle allowlist in `pnpm-workspace.yaml` unchanged.

- [ ] **Step 4: Regenerate all supported JavaScript lockfiles without lifecycle scripts**

Run:

```bash
pnpm install --lockfile-only --ignore-scripts
npm install --package-lock-only --ignore-scripts
cd src/ui/next
pnpm install --lockfile-only --ignore-scripts
npm install --package-lock-only --ignore-scripts
```

Expected: all four commands exit 0 and no lockfile retains `@tailwindcss/postcss` for the UI importer.

- [ ] **Step 5: Verify the pipeline check and production dependency audits**

Run:

```bash
cd src/ui/next
npm run test:tailwind-config
pnpm audit --prod
npm audit --omit=dev
```

Then run from the repository root:

```bash
pnpm audit --prod
npm audit --omit=dev
```

Expected: the pipeline check prints `Tailwind/PostCSS pipeline is coherent.` and all four audits report zero production vulnerabilities.

- [ ] **Step 6: Commit the pipeline repair**

```bash
git add package-lock.json pnpm-lock.yaml src/ui/next/package.json src/ui/next/package-lock.json src/ui/next/pnpm-lock.yaml src/ui/next/pnpm-workspace.yaml src/ui/next/scripts/assert-tailwind-pipeline.mjs
git commit -m "build: restore coherent Tailwind pipeline"
```

### Task 2: Define Universal Shell Ownership

**Files:**
- Create: `src/ui/next/src/app/components/shellRoutes.ts`
- Create: `src/ui/next/src/app/components/shellRoutes.test.ts`
- Modify: `src/ui/next/src/app/components/ProductShellGuard.tsx`
- Modify: `src/ui/next/src/app/components/ProductShellGuard.test.tsx`

- [ ] **Step 1: Write failing route ownership tests**

Create `src/ui/next/src/app/components/shellRoutes.test.ts`:

```ts
import { describe, expect, test } from 'vitest';
import { resolveShellRoute } from './shellRoutes';

describe('resolveShellRoute', () => {
  test.each(['/login', '/onboarding', '/booking-widget', '/storefront-widget', '/website-builder'])(
    'assigns formerly standalone route %s to the guard shell',
    (pathname) => expect(resolveShellRoute(pathname).owner).toBe('guard'),
  );

  test.each(['/assistant', '/dashboard', '/inbox', '/orders', '/settings'])(
    'does not double wrap page-owned route %s',
    (pathname) => expect(resolveShellRoute(pathname).owner).toBe('page'),
  );

  test('inherits ownership for nested page-owned routes', () => {
    expect(resolveShellRoute('/dashboard/campaigns').owner).toBe('page');
    expect(resolveShellRoute('/proposals/example').owner).toBe('page');
  });

  test('wraps unknown UI routes with derived metadata', () => {
    expect(resolveShellRoute('/new-feature')).toEqual({
      owner: 'guard',
      title: 'New Feature',
      subtitle: 'Use this workspace from the dashboard navigation.',
    });
  });
});
```

Replace the former standalone assertion in `ProductShellGuard.test.tsx` with:

```tsx
test('wraps public and widget routes in the universal shell', () => {
  navigationMock.pathname = '/work-intake-widget';
  render(<ProductShellGuard><div>Widget content</div></ProductShellGuard>);
  expect(screen.getByTestId('app-shell')).toBeDefined();
  expect(screen.getByRole('heading', { name: 'Work Intake Widget' })).toBeDefined();
});
```

- [ ] **Step 2: Run focused tests and verify the missing registry/old exclusion failures**

Run:

```bash
cd src/ui/next
pnpm exec vitest run src/app/components/shellRoutes.test.ts src/app/components/ProductShellGuard.test.tsx
```

Expected: FAIL because `shellRoutes.ts` does not exist and the existing guard excludes widgets.

- [ ] **Step 3: Implement the pure route registry**

Create `shellRoutes.ts` with this public contract:

```ts
export type ShellRoute = {
  owner: 'guard' | 'page';
  title: string;
  subtitle?: string;
};

const pageOwnedPrefixes = [
  '/action-center', '/agent-activity', '/ai-usage-paywall', '/analytics', '/assistant',
  '/business-analytics', '/cost-dashboard', '/dashboard', '/diagnostics', '/edge-storefront-setup',
  '/embed-builder', '/feed', '/finance', '/inbox', '/integrations', '/inventory', '/kairos',
  '/kitchen', '/lead-magnet-generator', '/operations', '/orders', '/pipeline', '/products',
  '/proposals', '/quotes', '/scaling', '/services', '/settings', '/staff', '/triage',
  '/viral-product-widget',
] as const;

const metadata: Record<string, Omit<ShellRoute, 'owner'>> = {
  '/agents': { title: 'AI Departments', subtitle: 'Manage expert teams, workflows, and assistant capabilities.' },
  '/calendar': { title: 'Calendar', subtitle: 'Manage schedule, bookings, and upcoming work.' },
  '/login': { title: 'Login', subtitle: 'Access your business workspace.' },
  '/onboarding': { title: 'Setup', subtitle: 'Configure your business workspace.' },
};

const matches = (pathname: string, prefix: string) => pathname === prefix || pathname.startsWith(`${prefix}/`);
const titleFromPath = (pathname: string) => pathname.split('/').filter(Boolean)[0]
  ?.split('-').map((part) => part.charAt(0).toUpperCase() + part.slice(1)).join(' ') || 'Dashboard';

export function resolveShellRoute(pathname: string | null): ShellRoute {
  const safePath = pathname || '/dashboard';
  const prefix = Object.keys(metadata).sort((a, b) => b.length - a.length).find((item) => matches(safePath, item));
  const routeMetadata = prefix ? metadata[prefix] : {
    title: titleFromPath(safePath),
    subtitle: 'Use this workspace from the dashboard navigation.',
  };
  return {
    owner: pageOwnedPrefixes.some((item) => matches(safePath, item)) ? 'page' : 'guard',
    ...routeMetadata,
  };
}
```

Include all existing title overrides from `ProductShellGuard.tsx` in `metadata`; do not discard current labels.

- [ ] **Step 4: Make ProductShellGuard consume the registry**

Replace its local route sets and `routeConfig` with:

```tsx
const route = resolveShellRoute(usePathname());
if (route.owner === 'page') return <>{children}</>;
return <AppShell title={route.title} subtitle={route.subtitle}>{children}</AppShell>;
```

- [ ] **Step 5: Run focused and full component tests**

Run:

```bash
cd src/ui/next
pnpm exec vitest run src/app/components/shellRoutes.test.ts src/app/components/ProductShellGuard.test.tsx
pnpm exec vitest run src/app/components/ProductShellGuard.test.tsx src/app/components/shellRoutes.test.ts
```

Expected: all tests pass and formerly standalone routes are guard-owned.

- [ ] **Step 6: Commit shell ownership**

```bash
git add src/ui/next/src/app/components/shellRoutes.ts src/ui/next/src/app/components/shellRoutes.test.ts src/ui/next/src/app/components/ProductShellGuard.tsx src/ui/next/src/app/components/ProductShellGuard.test.tsx
git commit -m "ui: apply universal shell ownership"
```

### Task 3: Establish the Rendered Shell Regression Matrix

**Files:**
- Modify: `src/ui/next/src/e2e/app-shell-style.spec.ts`
- Modify: `src/ui/next/playwright.config.ts`
- Create: `src/ui/next/scripts/visual-audit.mjs`

- [ ] **Step 1: Extend the Playwright test to fail on missing shells, inactive utilities, and mobile overflow**

Define the representative routes and viewports in `app-shell-style.spec.ts`:

```ts
const productRoutes = [
  '/dashboard', '/assistant', '/orders', '/inventory', '/inbox', '/agents', '/settings',
  '/business-analytics', '/integrations', '/calendar', '/diagnostics', '/agent-marketplace',
  '/visual-workflow', '/website-builder', '/booking-widget', '/storefront-widget', '/onboarding', '/login',
];

const viewports = [
  { name: 'desktop', width: 1440, height: 1000 },
  { name: 'mobile', width: 390, height: 844 },
];
```

For each pair, assert:

```ts
await expect(page.locator('.app-sidebar')).toHaveCount(1);
await expect(page.locator('.app-topbar')).toHaveCount(1);
await expect(page.locator('.app-main')).toHaveCount(1);
expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(1);
```

Add one computed utility assertion on `/agents`:

```ts
const utilityStyles = await page.locator('.rounded-2xl').first().evaluate((element) => {
  const style = getComputedStyle(element);
  return { radius: style.borderTopLeftRadius, padding: style.paddingTop };
});
expect(utilityStyles.radius).not.toBe('0px');
expect(utilityStyles.padding).not.toBe('0px');
```

- [ ] **Step 2: Run the rendered test and verify the known failures**

With the development server running, run:

```bash
cd src/ui/next
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium node node_modules/@playwright/test/cli.js test src/e2e/app-shell-style.spec.ts --project=chromium --reporter=line --workers=1
```

Expected before the remaining fixes: FAIL on `/agents` mobile overflow and on surfaces above 8.5 pixels. If Task 1 has not yet restarted the server, restart it before interpreting computed styles.

- [ ] **Step 3: Preserve the opt-in browser executable and reusable audit collector**

Keep this opt-in configuration in `playwright.config.ts`:

```ts
launchOptions: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH
  ? { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH }
  : undefined,
```

Complete `scripts/visual-audit.mjs` so it uses the same 18 routes and two viewports, records status, title, shell counts, console errors, document width, visible overflowing elements, and a full-page screenshot, then writes `report.json` under `VISUAL_AUDIT_OUTPUT_DIR` (default `/tmp/ohc-visual-audit`). Exit nonzero when navigation fails, an HTTP response is 400 or greater, shell count is not one, or document overflow exceeds one pixel.

- [ ] **Step 4: Verify the audit tool reports the same failing routes as Playwright**

Run:

```bash
cd src/ui/next
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium node scripts/visual-audit.mjs
jq '[.[] | select(.horizontalOverflow or .navigationError)] | map({route, viewport, documentWidth, viewportWidth})' /tmp/ohc-visual-audit/report.json
```

Expected before Task 4: `/agents` at mobile width is reported with a document width greater than 390 pixels.

- [ ] **Step 5: Commit the rendered regression harness**

```bash
git add src/ui/next/src/e2e/app-shell-style.spec.ts src/ui/next/playwright.config.ts src/ui/next/scripts/visual-audit.mjs
git commit -m "test: enforce universal rendered shell"
```

### Task 4: Enforce Responsive Containment and Surface Tokens

**Files:**
- Modify: `src/ui/next/src/app/globals.css`
- Modify: `src/ui/next/src/app/agents/page.tsx`
- Test: `src/ui/next/src/e2e/app-shell-style.spec.ts`

- [ ] **Step 1: Confirm the focused rendered test is red**

Run the `/agents` and surface cases from Task 3 with:

```bash
cd src/ui/next
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium node node_modules/@playwright/test/cli.js test src/e2e/app-shell-style.spec.ts --project=chromium --reporter=line --workers=1 --grep "agents|normalized surfaces"
```

Expected: FAIL with mobile `scrollWidth` above 390 and/or a radius above 8.5.

- [ ] **Step 2: Normalize shared shell surfaces and containment at the source**

In `globals.css`, change `.app-card`, `.app-panel`, `.glassmorphism`, and `.glass-card` shell radii to 8 pixels, including their dark variants. Add:

```css
.app-shell,
.app-main,
.app-page,
.app-page > * {
  min-width: 0;
  max-width: 100%;
}

.app-card,
.app-panel,
.app-shell .glassmorphism,
.app-shell .glass-card {
  border-radius: 8px !important;
}

@media (max-width: 980px) {
  .app-topbar {
    height: auto;
  }

  .app-topbar-right {
    min-width: 0;
  }

  .app-page {
    padding: 14px;
    overflow-x: clip;
  }
}
```

- [ ] **Step 3: Contain the Expert Center's wide descendants**

Change the root in `agents/page.tsx` to:

```tsx
<div className="min-h-screen min-w-0 max-w-full overflow-x-hidden bg-stone-50 text-zinc-950 transition-colors duration-200 dark:bg-zinc-950 dark:text-zinc-50">
```

Add `min-w-0 max-w-full` to its `header`, main content container, and each parent of an `overflow-x-auto` tab/card row. Preserve `min-w-[280px]` on cards only when their immediate row owns `overflow-x-auto` and its parent is width-constrained.

- [ ] **Step 4: Run the focused rendered regression**

Run:

```bash
cd src/ui/next
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium node node_modules/@playwright/test/cli.js test src/e2e/app-shell-style.spec.ts --project=chromium --reporter=line --workers=1 --grep "agents|normalized surfaces"
```

Expected: PASS with one shell, document width at most 391 pixels on mobile, active utility styles, and no surface radius above 8.5 pixels.

- [ ] **Step 5: Run page component regressions**

Run:

```bash
cd src/ui/next
pnpm exec vitest run src/app/agents/page.test.tsx src/app/components/ProductShellGuard.test.tsx
```

Expected: all tests pass.

- [ ] **Step 6: Commit containment and tokens**

```bash
git add src/ui/next/src/app/globals.css src/ui/next/src/app/agents/page.tsx
git commit -m "ui: normalize responsive shell surfaces"
```

### Task 5: Make Inbox Hydration Deterministic

**Files:**
- Modify: `src/ui/next/src/lib/powersync/PowerSyncProvider.tsx`
- Create: `src/ui/next/src/lib/powersync/PowerSyncProvider.ssr.test.tsx`
- Modify: `src/ui/next/src/lib/powersync/PowerSyncProvider.test.tsx`
- Modify: `src/ui/next/src/app/inbox/page.tsx`
- Test: `src/ui/next/src/e2e/app-shell-style.spec.ts`

- [ ] **Step 1: Write the failing initial-render regression**

Create `PowerSyncProvider.ssr.test.tsx`:

```tsx
// @vitest-environment node
import { renderToString } from 'react-dom/server';
import { expect, test, vi } from 'vitest';
import { PowerSyncProvider } from './PowerSyncProvider';

vi.mock('./db', () => ({ getPowerSyncDB: vi.fn() }));

test('renders the same fallback before browser capability detection', () => {
  const html = renderToString(
    <PowerSyncProvider fallback={<div>Stable loading state</div>} unsupportedFallback={<div>API fallback</div>}>
      <div>Database content</div>
    </PowerSyncProvider>,
  );
  expect(html).toContain('Stable loading state');
  expect(html).not.toContain('API fallback');
});
```

- [ ] **Step 2: Run the focused test and verify it fails**

Run:

```bash
cd src/ui/next
pnpm exec vitest run src/lib/powersync/PowerSyncProvider.ssr.test.tsx
```

Expected: FAIL because capability selection currently happens during render and chooses `unsupportedFallback` in jsdom/server-like conditions.

- [ ] **Step 3: Defer capability selection until after mount**

Replace render-time `supported` selection with state:

```tsx
const [supported, setSupported] = useState<boolean | null>(null);

useEffect(() => {
  setSupported(browserSupportsPowerSync());
}, []);

useEffect(() => {
  if (supported !== true) return;
  // existing initialization and cleanup
}, [supported]);

if (supported === null) return fallback || <div>Loading local database...</div>;
if (!supported || error) return unsupportedFallback || fallback || <div>Local database is unavailable in this browser context.</div>;
```

In `inbox/page.tsx`, extract one `InboxLoadingState` component and use identical text and markup for the provider's initial fallback. Keep the API fallback only for the post-mount unsupported/error state.

- [ ] **Step 4: Run unit and rendered hydration regressions**

Run:

```bash
cd src/ui/next
pnpm exec vitest run src/lib/powersync/PowerSyncProvider.test.tsx src/lib/powersync/PowerSyncProvider.ssr.test.tsx
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium node node_modules/@playwright/test/cli.js test src/e2e/app-shell-style.spec.ts --project=chromium --reporter=line --workers=1 --grep inbox
```

Expected: unit tests pass and the browser console contains no hydration mismatch or uncaught exception for `/inbox` at either viewport.

- [ ] **Step 5: Commit the hydration correction**

```bash
git add src/ui/next/src/lib/powersync/PowerSyncProvider.tsx src/ui/next/src/lib/powersync/PowerSyncProvider.test.tsx src/ui/next/src/lib/powersync/PowerSyncProvider.ssr.test.tsx src/ui/next/src/app/inbox/page.tsx src/ui/next/src/e2e/app-shell-style.spec.ts
git commit -m "fix: stabilize inbox hydration"
```

### Task 6: Production Build and Full Visual Verification

**Files:**
- Modify: `docs/reports/production_agent_optimization_report.md`
- Verify: all UI files changed in Tasks 1–5

- [ ] **Step 1: Run the complete UI unit suite**

Run:

```bash
cd src/ui/next
pnpm exec vitest run
```

Expected: all UI test files pass with zero failures.

- [ ] **Step 2: Run TypeScript and the production build independently**

Run:

```bash
cd src/ui/next
pnpm exec tsc --noEmit
pnpm run build
```

Expected: both commands exit 0. If either fails, stop and apply the systematic-debugging workflow to the first error before continuing; do not suppress type or build checks.

- [ ] **Step 3: Run the complete rendered shell matrix**

Run:

```bash
cd src/ui/next
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium node node_modules/@playwright/test/cli.js test src/e2e/app-shell-style.spec.ts src/e2e/test_styled_pages.spec.ts --project=chromium --reporter=line --workers=1
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium node scripts/visual-audit.mjs
```

Expected: all Playwright cases pass; the audit reports 36 successful renders, exactly one shell per render, no navigation failures, and no document-level overflow.

- [ ] **Step 4: Inspect representative screenshots manually**

Inspect at original resolution:

```text
/tmp/ohc-visual-audit/desktop__dashboard.png
/tmp/ohc-visual-audit/mobile__dashboard.png
/tmp/ohc-visual-audit/desktop__agents.png
/tmp/ohc-visual-audit/mobile__agents.png
/tmp/ohc-visual-audit/desktop__integrations.png
/tmp/ohc-visual-audit/mobile__integrations.png
/tmp/ohc-visual-audit/desktop__website-builder.png
/tmp/ohc-visual-audit/mobile__website-builder.png
/tmp/ohc-visual-audit/desktop__login.png
/tmp/ohc-visual-audit/mobile__login.png
```

Confirm one navigation system, readable hierarchy, no clipped text, no action/help collisions, consistent 8-pixel surfaces, and locally contained wide content.

- [ ] **Step 5: Run Bazel and dependency verification**

Run from the repository root:

```bash
bazel test //src/ui/next:next_vitest --test_output=errors
pnpm audit --prod
npm audit --omit=dev
cd src/ui/next
pnpm audit --prod
npm audit --omit=dev
```

Expected: Bazel passes and all production dependency audits report zero vulnerabilities.

- [ ] **Step 6: Record exact evidence and remaining environmental limitations**

Append a UI finding/remediation section to `docs/reports/production_agent_optimization_report.md` containing:

```markdown
### UI-01 Universal UI shell and rendered consistency

- Root cause: Tailwind 4 PostCSS processed a Tailwind 3 application, omitting page utilities.
- Remediation: one Tailwind 3 pipeline, universal AppShell ownership, responsive containment, stable PowerSync hydration, and rendered regression matrix.
- Evidence: include exact Vitest, TypeScript, Next build, Playwright, audit, and Bazel counts from this run.
- Visual matrix: 18 routes at 1440x1000 and 390x844; record overflow, shell-count, hydration, and console results.
- Environment: record the opt-in `/snap/bin/chromium` override if bundled Playwright Chromium remains unsupported on this host.
```

Use the actual outputs; do not copy expected counts into the report.

- [ ] **Step 7: Commit the verified audit record**

```bash
git add docs/reports/production_agent_optimization_report.md
git commit -m "docs: record universal UI verification"
```

- [ ] **Step 8: Confirm repository state**

Run:

```bash
git status --short
git log --oneline --decorate -8
```

Expected: no generated Playwright, coverage, `.next`, or TypeScript build artifacts are tracked or left modified; only explicitly preserved unrelated user changes may remain.
