import { expect, test } from "@playwright/test";
import { discoverApplicationRoutes } from "./production_route_inventory";

const baseUrl = process.env.PLAYWRIGHT_BASE_URL ?? "http://localhost:3000";
const adminEmail = process.env.OMNISOLO_ADMIN_EMAIL ?? process.env.OHC_ADMIN_EMAIL;
const adminPassword = process.env.OMNISOLO_ADMIN_PASSWORD ?? process.env.OHC_ADMIN_PASSWORD;
const organizationId = process.env.OMNISOLO_ADMIN_ORGANIZATION_ID
  ?? process.env.OHC_ADMIN_ORGANIZATION_ID
  ?? "org-1";

async function loginThroughRenderedForm(page: import("@playwright/test").Page) {
  test.skip(!adminEmail || !adminPassword, "OMNISOLO_ADMIN_EMAIL/OMNISOLO_ADMIN_PASSWORD are required for live smoke tests");
  await page.goto(new URL("/login", baseUrl).toString(), { waitUntil: "domcontentloaded" });
  await page.getByLabel("Email or username").fill(adminEmail!);
  await page.getByLabel("Password").fill(adminPassword!);
  await page.getByLabel(/Organization/).fill(organizationId);
  await Promise.all([
    page.waitForURL(/\/dashboard(?:\?|$)/, { timeout: 30_000 }),
    page.getByRole("button", { name: "Log in" }).click(),
  ]);
}

test("health check is public and returns a live response", async ({ page }) => {
  const response = await page.goto(new URL("/healthz", baseUrl).toString(), {
    waitUntil: "domcontentloaded",
  });
  expect(response?.status()).toBe(200);
  expect(page.url()).toMatch(/\/healthz(?:\?|$)/);
  await expect(page.locator("body")).toContainText("ok");
});

test("all application pages render through the real authenticated service", async ({ page }) => {
  test.setTimeout(15 * 60_000);
  await loginThroughRenderedForm(page);
  const failures: string[] = [];
  const httpFailures: string[] = [];
  const requestFailures: string[] = [];
  const consoleErrors: string[] = [];
  const websocketFailures: string[] = [];
  page.on("response", (response) => {
    if (response.status() >= 500) failures.push(`${response.status()} ${response.url()}`);
    else if (response.status() >= 400) httpFailures.push(`${response.status()} ${response.url()}`);
  });
  page.on("requestfailed", (request) => {
    const failure = request.failure()?.errorText ?? "request failed";
    if (!request.url().startsWith("data:")) requestFailures.push(`${failure} ${request.url()}`);
  });
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });
  page.on("websocket", (websocket) => {
    websocket.on("socketerror", (error) => websocketFailures.push(`${error} ${websocket.url()}`));
  });

  const routeFailures: string[] = [];
  const contentFailures: string[] = [];

  for (const route of discoverApplicationRoutes()) {
    try {
      const response = await page.goto(new URL(route, baseUrl).toString(), {
        waitUntil: "domcontentloaded",
        timeout: 45_000,
      });
      if (!response) {
        routeFailures.push(`${route} did not return a document`);
        continue;
      }
      if (response.status() >= 400) {
        routeFailures.push(`${response.status()} ${route}`);
      }
      const body = await page.locator("body").innerText();
      if (/Generated Offering|AI description|mock data|fake data/i.test(body)) {
        contentFailures.push(`${route} contains fabricated data`);
      }
      if (/OneHumanCorp|One Human Corp|Powered by OHC/i.test(body)) {
        contentFailures.push(`${route} contains legacy branding`);
      }
    } catch (error) {
      routeFailures.push(`${route}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  expect(routeFailures, "application page failures during the page crawl").toEqual([]);
  expect(contentFailures, "fabricated data or legacy branding during the page crawl").toEqual([]);
  expect(httpFailures, "unexpected HTTP 4xx responses during the page crawl").toEqual([]);
  expect(failures, "server-side 5xx responses during the page crawl").toEqual([]);
  expect(requestFailures, "failed browser requests during the page crawl").toEqual([]);
  expect(consoleErrors, "browser console errors during the page crawl").toEqual([]);
  expect(websocketFailures, "failed WebSocket upgrades during the page crawl").toEqual([]);
});

test("catalog create/read round trip uses the persisted service", async ({ page }) => {
  await loginThroughRenderedForm(page);
  const name = `OmniSolo live smoke ${Date.now()}`;
  const create = await page.evaluate(async (payload) => {
    const response = await fetch("/api/v1/catalog/product", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(payload),
    });
    return { status: response.status, body: await response.text() };
  }, {
    name,
    price: "12.34",
    description: "live browser acceptance record",
    item_type: "Product",
  });
  expect(create.status).toBeLessThan(300);

  const list = await page.evaluate(async () => {
    const response = await fetch("/api/v1/catalog/products");
    return { status: response.status, products: await response.json() };
  });
  expect(list.status).toBe(200);
  const products = list.products;
  expect(products.filter((product: { name?: string }) => product.name === name)).toHaveLength(1);
});

test("logout removes protected access and retains public route access", async ({ page }) => {
  test.setTimeout(60_000);
  await loginThroughRenderedForm(page);
  await page.goto(new URL("/dashboard", baseUrl).toString());
  const logout = page.getByRole("button", { name: /log out/i }).first();
  await expect(logout).toBeVisible({ timeout: 15_000 });
  const logoutResponse = page.waitForResponse(
    (response) => response.url().includes("/api/v1/auth/logout") && response.request().method() === "POST",
  );
  await logout.click();
  const response = await logoutResponse;
  expect(response.status()).toBe(200);
  await expect(page).toHaveURL(/\/login(?:\?|$)/, { timeout: 20_000 });

  const publicPage = await page.goto(new URL("/pricing", baseUrl).toString());
  expect(publicPage?.status()).toBeLessThan(400);
  const protectedPage = await page.goto(new URL("/dashboard", baseUrl).toString());
  expect(protectedPage?.url()).toMatch(/\/login(?:\?|$)/);
});
