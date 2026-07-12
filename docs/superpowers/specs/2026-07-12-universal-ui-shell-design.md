# Universal UI Shell Design

## Objective

Make every rendered Next.js page use the same application shell and responsive surface language. This includes authentication, onboarding, builders, widgets, and operational pages. Preserve each page's workflows and content while eliminating raw, unstyled, double-shelled, or horizontally overflowing layouts.

The visual audit found that the browser receives Tailwind 4 output while the application uses Tailwind 3 directives, configuration, and utility syntax. Most utilities are therefore absent at runtime. It also found route-specific shell exclusions and page-owned shells, a 1122-pixel `/agents` document at a 390-pixel viewport, an `/inbox` hydration mismatch, and inconsistent 16-pixel surfaces where the existing shell contract requires at most 8 pixels.

## Chosen Approach

Restore the existing Tailwind 3 pipeline, then make `AppShell` the single shell component used by every page. A route registry will decide whether a legacy page already owns its `AppShell` or whether `ProductShellGuard` supplies it. This permits one shell contract without producing nested sidebars while page-owned wrappers are migrated safely.

The rejected alternatives are:

- A CSS-only repair. This would restore utilities but retain route exclusions and inconsistent shell ownership.
- A simultaneous rewrite of all page markup. The application has more than 150 page entry points, so a full rewrite would create unnecessary regression risk without improving the shell contract.

## Styling Pipeline

The UI will use one Tailwind major version and one PostCSS configuration. Tailwind 3 is the compatibility choice because the repository already uses its directives, TypeScript configuration, dark-mode variants, arbitrary values, and utility conventions.

The conflicting Tailwind 4 PostCSS dependency and package-level PostCSS configuration will be removed. `postcss.config.mjs`, `tailwind.config.ts`, and `globals.css` will remain the authoritative pipeline. Lockfiles for supported package managers will be regenerated consistently.

A build-level regression check will verify that representative utilities such as `rounded-2xl`, `bg-white/60`, responsive variants, and padding utilities produce non-default computed styles in a real browser.

## Shell Ownership

`AppShell` remains the only implementation of the navigation, top bar, page frame, help placement, and responsive mobile navigation. `ProductShellGuard` will no longer classify any UI route as standalone.

The route registry will contain:

- Page title and subtitle metadata.
- An explicit temporary ownership mode for pages that currently render `AppShell` themselves.
- A guard-owned mode for all other pages, including login, onboarding, builders, and widgets.

Every page must resolve to exactly one owner. Unknown routes default to a guard-owned shell with a title derived from the path. API handlers and static assets are outside the page-shell contract because they do not render UI.

Legacy page-owned wrappers may be removed incrementally. Until removal, the explicit ownership mode prevents double shells. The acceptance test is the rendered result—exactly one visible sidebar, top bar, and main region—not which file currently supplies the component.

## Visual Contract

The universal shell will retain the current application language: slate navigation, light content canvas, blue primary actions, restrained teal status accents, and high-contrast typography.

Shared rules are:

- Shell panels and cards use an 8-pixel maximum corner radius; pills and circular controls are exempt.
- Pages use the shell's content width, spacing, and background rather than introducing a viewport-sized competing canvas.
- Main content and all direct layout children use `min-width: 0` where required.
- Wide tables, tab lists, and carousels scroll inside their owning region instead of expanding the document.
- Desktop actions remain in the top bar. On mobile they wrap or collapse without covering titles or content.
- The floating help control stays within the viewport and does not obscure primary controls.
- Page-specific colors and feature illustrations may remain, but page-level navigation, typography hierarchy, surfaces, and responsive behavior follow the shell contract.

## Responsive Behavior

The acceptance viewports are 1440 by 1000 pixels for desktop and 390 by 844 pixels for mobile. Additional automated checks may cover a tablet breakpoint.

At mobile width:

- The document width must not exceed the viewport by more than one pixel.
- Navigation becomes the existing compact shell navigation.
- Multi-column content stacks into one column unless the contained region provides intentional local scrolling.
- Controls remain reachable with no clipped labels or overlapping headings.
- Minimum-width feature cards must be constrained by their parent or placed in a bounded horizontal scroller.

## Runtime and Error States

Shell rendering must not depend on the backend being available. Pages may show a contained loading, empty, offline, or error state, but they must retain their shell and layout.

The `/inbox` server and client must render the same initial state to avoid hydration replacement. Expected local-development backend failures must be represented by stable UI states rather than raw error overlays. Unexpected browser exceptions and hydration warnings fail the rendered audit.

No authentication or authorization behavior changes are included in this visual project. Wrapping login and onboarding in the shell changes presentation only.

## Testing Strategy

Implementation follows test-driven development. Before each behavior change, a focused test must fail for the observed defect.

Required coverage includes:

1. A configuration regression test proving the project uses one compatible Tailwind/PostCSS pipeline.
2. Component or route tests proving every page classification resolves to exactly one shell owner.
3. A rendered Playwright matrix that checks representative routes at desktop and mobile widths for exactly one `.app-sidebar`, `.app-topbar`, and `.app-main`.
4. Computed-style checks proving Tailwind utilities are active.
5. Document-width checks that reproduce and prevent the `/agents` mobile overflow.
6. Hydration-console checks reproducing and preventing the `/inbox` mismatch.
7. Surface checks enforcing the 8-pixel shell radius contract.
8. Existing Vitest, Playwright, TypeScript, Next production-build, and Bazel UI targets.

The rendered matrix will cover at least dashboard, assistant, orders, inventory, inbox, agents, settings, analytics, integrations, calendar, diagnostics, marketplace, visual workflow, website builder, booking widget, storefront widget, onboarding, and login. Any failure records its route, viewport, console errors, document width, and screenshot.

## Delivery Sequence

1. Add failing pipeline and computed-style regressions, then repair the Tailwind/PostCSS mismatch.
2. Add failing shell-ownership tests, then remove standalone exclusions and complete route metadata.
3. Add failing responsive tests, then correct document overflow and top-bar/help collisions.
4. Add the hydration regression, then stabilize the inbox initial render.
5. Normalize shared surfaces and verify representative page content visually.
6. Run the full rendered matrix, production build, UI tests, TypeScript checks, and Bazel target.

Changes will be split into reviewable commits so the dependency repair, shell architecture, responsive corrections, and hydration correction can be inspected or reverted independently.

## Success Criteria

- Every rendered page has exactly one universal application shell.
- Tailwind utility classes have observable computed styles in production and development builds.
- No audited mobile page creates document-level horizontal overflow.
- No audited page produces a hydration warning or uncaught browser exception.
- Shared shell surfaces obey the 8-pixel radius contract.
- Desktop and mobile screenshots show consistent navigation, typography hierarchy, spacing, and help placement.
- The Next production build and relevant UI test targets pass without introducing new type errors.

## Non-Goals

- Redesigning the information architecture or business workflows.
- Replacing `AppShell` with a new design system.
- Rewriting all page components when a shared-shell or containment correction is sufficient.
- Changing backend availability, authentication policy, or API behavior.
