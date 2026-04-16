import { chromium, devices } from "@playwright/test";
import { mkdir } from "node:fs/promises";
import path from "node:path";

const baseUrl = process.env.PLAYWRIGHT_BASE_URL;
const outputRoot = process.env.APP_SCREENSHOT_OUTPUT_DIR;

if (!baseUrl) {
  throw new Error("PLAYWRIGHT_BASE_URL is required.");
}

if (!outputRoot) {
  throw new Error("APP_SCREENSHOT_OUTPUT_DIR is required.");
}

const desktopContext = (userAgent) => ({
  viewport: { width: 1512, height: 982 },
  deviceScaleFactor: 1,
  isMobile: false,
  hasTouch: false,
  colorScheme: "light",
  userAgent,
});

const profiles = [
  {
    name: "web",
    context: desktopContext(
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    ),
  },
  {
    name: "linux",
    context: desktopContext(
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    ),
  },
  {
    name: "windows",
    context: desktopContext(
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    ),
  },
  {
    name: "macos",
    context: desktopContext(
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    ),
  },
  {
    name: "android",
    context: {
      ...devices["Pixel 7"],
      colorScheme: "light",
    },
  },
  {
    name: "ios",
    context: {
      ...devices["iPhone 14"],
      colorScheme: "light",
    },
  },
];

/** Wait for Flutter to mount its first frame. */
async function waitForFlutter(page, timeoutMs = 60000) {
  await page.waitForFunction(
    () =>
      Boolean(
        document.querySelector("flutter-view") ||
        document.querySelector("flt-glass-pane") ||
        document.querySelector("canvas"),
      ),
    { timeout: timeoutMs },
  );
  await page.waitForTimeout(1500);
}

/**
 * Seed the backend with a named scenario.
 * Silently ignores errors so screenshots still run in static-only mode.
 */
async function seedBackend(page, scenario = "launch-readiness") {
  await page.evaluate(async (sc) => {
    try {
      await fetch(window.location.origin + "/api/dev/seed", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ scenario: sc }),
      });
    } catch {
      // no backend available – continue
    }
  }, scenario);
}

/**
 * Log in using seeded admin credentials.
 * Navigates to /login and submits the form via keyboard.
 */
async function loginAsAdmin(page) {
  await page.goto(baseUrl + "/login");
  await waitForFlutter(page);
  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  await page.keyboard.type("admin@localhost");
  await page.keyboard.press("Tab");
  await page.keyboard.type("adminpass123");
  await page.keyboard.press("Enter");
  await page.waitForTimeout(1500);
}

/**
 * Navigate to a route, wait for Flutter to settle, and take a screenshot.
 */
async function captureRoute(page, targetDir, name, route) {
  await page.goto(baseUrl + route);
  await waitForFlutter(page);
  await page.waitForTimeout(800);
  await mkdir(targetDir, { recursive: true });
  await page.screenshot({
    path: path.join(targetDir, `${name}.png`),
    fullPage: true,
  });
}

// ---------------------------------------------------------------------------
// All screens to capture
// ---------------------------------------------------------------------------

/**
 * Returns the ordered list of { name, route } to screenshot after login.
 * Public (unauthenticated) routes are handled separately.
 */
const authenticatedScreens = [
  { name: "dashboard", route: "/dashboard" },
  { name: "agents", route: "/agents" },
  { name: "agent-hire-wizard", route: "/agents/hire" },
  { name: "prompt-tuning-wizard", route: "/agents/default/tune" },
  { name: "meetings", route: "/meetings" },
  { name: "chat", route: "/chat" },
  { name: "channels", route: "/channels" },
  { name: "ai-providers", route: "/ai-config" },
  { name: "skills", route: "/skills" },
  { name: "logs", route: "/logs" },
  { name: "security", route: "/security" },
  { name: "settings", route: "/settings" },
  { name: "service-management", route: "/service" },
  { name: "setup-wizard", route: "/wizard" },
  { name: "diagnostics", route: "/diagnostics" },
  { name: "business-setup-wizard", route: "/business_setup" },
  { name: "handoffs", route: "/handoffs" },
  { name: "cost-dashboard", route: "/cost" },
  { name: "dynamic-scaling", route: "/scaling" },
  { name: "pipelines", route: "/pipelines" },
  { name: "integrations", route: "/integrations" },
  { name: "user-management", route: "/users" },
  { name: "fix-wizard", route: "/wizards/fix/default" },
  { name: "upgrade-wizard", route: "/wizards/upgrade" },
  { name: "billing-wizard", route: "/wizards/billing" },
  { name: "task-list", route: "/orchestration/tasks" },
  { name: "swarm-memory", route: "/swarm-memory" },
  { name: "growth-experiments", route: "/growth-experiments" },
  { name: "referrals", route: "/referrals" },
];

const publicScreens = [
  { name: "landing-page", route: "/landing" },
  { name: "login", route: "/login" },
];

// ---------------------------------------------------------------------------
// Main capture loop
// ---------------------------------------------------------------------------

// Run headless:false so that xvfb-run can provide a real X11 display with
// Mesa software GL (LIBGL_ALWAYS_SOFTWARE=1).  This allows Flutter's
// CanvasKit/skwasm renderer to use WebGL and produce real screenshots.
// When run via `bazelisk run //srcs/app:capture_screenshots` the wrapper
// script (test/capture_screenshots.sh) calls us under xvfb-run with the
// LIBGL_ALWAYS_SOFTWARE env var set.
const browser = await chromium.launch({
  headless: false,
  args: [
    '--no-sandbox',
    '--disable-dev-shm-usage',
    '--use-gl=egl',
    '--enable-webgl',
    '--enable-webgl2',
  ],
});

try {
  for (const profile of profiles) {
    const context = await browser.newContext(profile.context);
    const page = await context.newPage();

    // ── Public screens (no login required) ──────────────────────────────

    for (const { name, route } of publicScreens) {
      const targetDir = path.join(outputRoot, name);
      await captureRoute(page, targetDir, profile.name, route);
    }

    // Seed before authenticated screens
    await page.goto(baseUrl);
    await waitForFlutter(page);
    await seedBackend(page);
    await loginAsAdmin(page);

    // ── Authenticated screens ────────────────────────────────────────────

    for (const { name, route } of authenticatedScreens) {
      const targetDir = path.join(outputRoot, name);
      await captureRoute(page, targetDir, profile.name, route);
    }

    await context.close();
  }
} finally {
  await browser.close();
}