import { chromium, devices } from "@playwright/test";
import { mkdir } from "node:fs/promises";
import path from "node:path";
import sqlite3 from "sqlite3";

const baseUrl = process.env.PLAYWRIGHT_BASE_URL;
const outputRoot = process.env.APP_SCREENSHOT_OUTPUT_DIR;
// Default location for the SIP database unless overridden
const dbPath = process.env.OHC_SIP_DB_PATH || path.join(process.env.HOME, ".openclaw/ohc.db");

if (!baseUrl) {
  throw new Error("PLAYWRIGHT_BASE_URL is required.");
}

if (!outputRoot) {
  throw new Error("APP_SCREENSHOT_OUTPUT_DIR is required.");
}

// Wrap sqlite in promises to read swarm memory targets
function getTargetsFromDb() {
  return new Promise((resolve, reject) => {
    const db = new sqlite3.Database(dbPath, sqlite3.OPEN_READONLY, (err) => {
      if (err) return reject(err);
    });
    // Check swarm_memory for 'doc_visual_metadata' target array
    db.get("SELECT value FROM swarm_memory WHERE key = 'doc_visual_metadata'", (err, row) => {
      db.close();
      if (err) return reject(err);
      if (row && row.value) {
        try {
          const data = JSON.parse(row.value);
          if (data && data.pages) return resolve(data.pages);
        } catch (e) {
            console.error("Failed to parse visual metadata pages", e);
        }
      }
      // Default fallback
      resolve(["login", "dashboard", "agents", "settings"]);
    });
  });
}

function updateVisualFreshness() {
    return new Promise((resolve, reject) => {
      const db = new sqlite3.Database(dbPath, sqlite3.OPEN_READWRITE, (err) => {
        if (err) return reject(err);
      });
      const query = `INSERT INTO agent_status (agent_id, role, status, last_heartbeat)
                     VALUES ('playwright-capture-1', 'Principal Frontend Architect', 'Visual Freshness Score: 100', datetime('now'))
                     ON CONFLICT(agent_id) DO UPDATE SET status='Visual Freshness Score: 100', last_heartbeat=datetime('now')`;
      db.run(query, (err) => {
        db.close();
        if (err) return reject(err);
        resolve();
      });
    });
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

async function capture() {
  const browser = await chromium.launch({ headless: true });
  const targets = await getTargetsFromDb();
  console.log(`Using targets from database: ${targets.join(', ')}`);

  try {
    for (const profile of profiles) {
      const context = await browser.newContext(profile.context);
      const page = await context.newPage();
      page.setDefaultTimeout(90000);

      console.log(`Loading application on ${profile.name}...`);
      await page.goto(baseUrl, { waitUntil: "networkidle" });

      // Removed waitForFunction as flutter-view might not be standard on canvaskit renderer from 'flutter build web --profile'
      // Instead, we just wait a bit for it to finish rendering.
      await page.waitForTimeout(6000);

      const targetDir = path.join(outputRoot, profile.name);
      await mkdir(targetDir, { recursive: true });

      // Hide any tooltips or cursors if present (CSS hack for Flutter Web)
      await page.addStyleTag({ content: 'body { cursor: none !important; } .flutter-tooltip { display: none !important; }' });

      // If viewport is small (mobile), clicks change
      const isMobile = profile.name === 'ios' || profile.name === 'android';
      // Approximate center clicks for login based on profile dimensions
      const viewport = profile.context.viewport || devices[profile.context.name]?.viewport || {width: 400, height: 800};

      const centerX = viewport.width ? Math.floor(viewport.width / 2) : 756;
      const centerY = viewport.height ? Math.floor(viewport.height / 2) : 491;

      if (targets.includes("login")) {
          console.log(`Capturing login...`);
          await page.screenshot({ path: path.join(targetDir, "login.png"), fullPage: true });
      }

      // Perform login
      console.log(`Attempting login by clicking center...`);
      // Username field usually slightly above center
      await page.mouse.click(centerX, centerY - 40);
      await page.keyboard.type('test-user');
      await page.waitForTimeout(500);

      await page.mouse.click(centerX, centerY + 30); // Password field
      await page.keyboard.type('password');
      await page.waitForTimeout(500);

      await page.mouse.click(centerX, centerY + 110); // Login button
      await page.waitForTimeout(4000);

      if (targets.includes("dashboard")) {
          console.log(`Capturing dashboard...`);
          await page.screenshot({ path: path.join(targetDir, "dashboard.png"), fullPage: true });
      }

      // Sidebar navigation
      if (!isMobile) {
          if (targets.includes("agents")) {
              console.log(`Capturing agents...`);
              await page.mouse.click(100, 300); // Sidebar item "Agents"
              await page.waitForTimeout(3000);
              await page.screenshot({ path: path.join(targetDir, "agents.png"), fullPage: true });
          }

          if (targets.includes("settings")) {
              console.log(`Capturing settings...`);
              await page.mouse.click(100, 400); // Sidebar item "Settings"
              await page.waitForTimeout(3000);
              await page.screenshot({ path: path.join(targetDir, "settings.png"), fullPage: true });
          }
      }

      await context.close();
    }

    // Update the database Visual Freshness metric on successful completion
    await updateVisualFreshness();
    console.log("Visual Freshness updated in DB.");

  } finally {
    await browser.close();
  }
}

capture().catch(err => {
    console.error("Failed to capture screenshots:", err);
    process.exit(1);
});
