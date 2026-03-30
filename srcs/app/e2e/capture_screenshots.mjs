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

const routes = [
  { name: "dashboard", path: "/dashboard" },
  { name: "agents", path: "/agents" },
  { name: "meetings", path: "/meetings" },
  { name: "chat", path: "/chat" },
  { name: "handoffs", path: "/handoffs" },
  { name: "cost", path: "/cost" },
  { name: "scaling", path: "/scaling" },
  { name: "pipelines", path: "/pipelines" },
  { name: "integrations", path: "/integrations" },
  { name: "users", path: "/users" },
  { name: "channels", path: "/channels" },
  { name: "ai-config", path: "/ai-config" },
  { name: "skills", path: "/skills" },
  { name: "security", path: "/security" },
  { name: "logs", path: "/logs" },
  { name: "settings", path: "/settings" },
  { name: "service", path: "/service" },
  { name: "wizard", path: "/wizard" },
  { name: "agent-hire", path: "/agents/hire" }
];

const browser = await chromium.launch({ headless: true });

try {
  for (const profile of profiles) {
    const context = await browser.newContext(profile.context);
    const page = await context.newPage();
    const targetDir = path.join(outputRoot, profile.name);
    await mkdir(targetDir, { recursive: true });

    // Login page capture
    console.log(`[${profile.name}] Navigating to /login...`);
    await page.goto(baseUrl + '/', { waitUntil: "networkidle" });
    await page.waitForFunction(
      () =>
        Boolean(
          document.querySelector("flutter-view") ||
            document.querySelector("flt-glass-pane") ||
            document.querySelector("canvas"),
        ),
      { timeout: 60000 },
    );
    await page.waitForTimeout(1500);

    // We capture login
    await page.screenshot({
      path: path.join(targetDir, "login_default.png"),
      fullPage: true,
    });

    console.log(`[${profile.name}] Entering credentials...`);
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type("ceo@onehumancorp.com");
    await page.keyboard.press('Tab');
    await page.keyboard.type("password");
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');

    try {
        await page.waitForURL(/\/dashboard/, { timeout: 8000 });
        console.log(`[${profile.name}] Logged in successfully.`);
    } catch (e) {
        console.log(`[${profile.name}] Login redirect timeout, trying direct injection...`);
        await page.evaluate(() => {
            window.localStorage.setItem('flutter.auth_token', '"fake_token"');
        });
        await page.reload({ waitUntil: "networkidle" });
    }

    for (const route of routes) {
      console.log(`[${profile.name}] Capturing ${route.path}...`);
      await page.goto(baseUrl + '/', { waitUntil: "networkidle" });
      await page.evaluate((path) => {
          // Force flutter router to navigate
          window.history.pushState(null, '', path);
          window.dispatchEvent(new PopStateEvent('popstate'));
      }, route.path);
      // Wait for network idle after the pseudo-navigation
      await page.waitForTimeout(1000);
      await page.waitForFunction(
        () =>
          Boolean(
            document.querySelector("flutter-view") ||
              document.querySelector("flt-glass-pane") ||
              document.querySelector("canvas"),
          ),
        { timeout: 60000 },
      );
      await page.waitForTimeout(2000);

      await page.screenshot({
        path: path.join(targetDir, `${route.name}_default.png`),
        fullPage: true,
      });
    }

    await context.close();
  }
} catch (e) {
  console.error("Error during screenshot capture:", e);
  process.exit(1);
} finally {
  await browser.close();
}
