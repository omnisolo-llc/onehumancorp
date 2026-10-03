import { E2E_ADMIN_USER, expect, test } from "../../../../e2e/fixtures";
import { discoverApplicationRoutes } from "./production_route_inventory";
import { recordSmokeHttpResponse, isVerifiedVoicePolicyDiagnostic } from "../../../../e2e/support/hosted_voice_policy";
import { isVerifiedRuntimePolicyDiagnostic, runtimeUnavailableMessage } from "../../../../e2e/support/runtime_policy";
import { createLinkBioActor } from "../../../../e2e/link_bio_owner";

const baseUrl = process.env.PLAYWRIGHT_BASE_URL ?? process.env.BASE_URL ?? "http://127.0.0.1:3000";
const adminEmail = process.env.OMNISOLO_ADMIN_EMAIL ?? process.env.OHC_ADMIN_EMAIL ?? E2E_ADMIN_USER.email;
const adminPassword = process.env.OMNISOLO_ADMIN_PASSWORD ?? process.env.OHC_ADMIN_PASSWORD ?? E2E_ADMIN_USER.password;
const organizationId = process.env.OMNISOLO_ADMIN_ORGANIZATION_ID
  ?? process.env.OHC_ADMIN_ORGANIZATION_ID
  ?? E2E_ADMIN_USER.organizationId;

async function loginThroughRenderedForm(page: import("@playwright/test").Page, actor = { email: adminEmail!, password: adminPassword!, organizationId }, appBaseUrl = baseUrl) {
  await page.goto(new URL("/login", appBaseUrl).toString(), { waitUntil: "domcontentloaded" });
  await page.getByLabel("Email or username").fill(actor.email);
  await page.getByLabel("Password").fill(actor.password);
  await page.getByLabel(/Organization/).fill(actor.organizationId);
  await page.getByRole("button", { name: "Log in" }).click();
  await page.waitForURL(/\/dashboard(?:\?|$)/, { timeout: 30_000 });
}

test("health check is public and returns a live response", async ({ anonymousPage: page }) => {
  const response = await page.goto(new URL("/healthz", baseUrl).toString(), {
    waitUntil: "domcontentloaded",
  });
  expect(response?.status()).toBe(200);
  expect(page.url()).toMatch(/\/healthz(?:\?|$)/);
  await expect(page.locator("body")).toContainText("ok");
});

test("all application pages render through the real authenticated service", async ({ anonymousPage: page, baseURL, adminUser }) => {
  test.setTimeout(15 * 60_000);
  const actor = await createLinkBioActor(page, baseURL, adminUser);
  const baseUrl = new URL(baseURL!).origin;
  await page.context().clearCookies();
  await loginThroughRenderedForm(page, { email: actor.email, password: adminUser.password, organizationId: actor.tenantId }, baseUrl);
  // The private preview route requires an actual owned configuration. Build it
  // through the maintained editor, then verify the persisted response before crawling.
  await page.goto(new URL('/link-in-bio-generator', baseUrl).href);
  await page.getByRole('textbox', { name: 'Store / Creator Name' }).fill('Owned smoke profile');
  await page.getByRole('textbox', { name: 'Bio / Description' }).fill('Private configuration for this isolated smoke actor.');
  const saved = page.waitForResponse(response => new URL(response.url()).origin === new URL(baseUrl).origin
    && new URL(response.url()).pathname === '/api/v1/growth/link-in-bio' && response.request().method() === 'POST');
  await page.getByRole('button', { name: 'Save private configuration', exact: true }).click();
  const saveResponse = await saved;
  expect(saveResponse.status()).toBe(200);
  expect(await saveResponse.text()).toBe('');
  expect(saveResponse.request().postDataJSON().tenant_id).toBe(actor.tenantId);
  expect(saveResponse.request().headers()['x-ohc-expected-user']).toBe(actor.userId);
  expect(saveResponse.request().headers()['x-ohc-expected-tenant']).toBe(actor.tenantId);
  const savedProfile = await page.request.get(new URL(`/api/v1/growth/link-in-bio/${encodeURIComponent(actor.tenantId)}`, baseUrl).href, {
    headers: { 'x-ohc-expected-user': actor.userId, 'x-ohc-expected-tenant': actor.tenantId },
  });
  expect(savedProfile.status()).toBe(200);
  expect(await savedProfile.json()).toMatchObject({ store_name: 'Owned smoke profile', bio: 'Private configuration for this isolated smoke actor.', links: [] });
  const failures: string[] = [];
  const httpFailures: string[] = [];
  const requestFailures: string[] = [];
  const consoleErrors: { text: string; url: string }[] = [];
  const verifiedPolicyUrls = new Set<string>();
  const policyChecks: Promise<void>[] = [];
  const websocketFailures: string[] = [];
  page.on("response", (response) => {
    recordSmokeHttpResponse(response, baseUrl, { failures, httpFailures, verifiedPolicyUrls, policyChecks });
  });
  page.on("requestfailed", (request) => {
    const failure = request.failure()?.errorText ?? "request failed";
    if (
      !request.url().startsWith("data:") &&
      !failure.includes("ERR_ABORTED") &&
      !failure.includes("NS_BINDING_ABORTED") &&
      !failure.includes("aborted")
    ) {
      requestFailures.push(`${failure} ${request.url()}`);
    }
  });
  page.on("console", (message) => {
    if (message.type() === "error") {
      const text = message.text();
      if (text.includes("Failed to load resource: the server responded with a status of 404")) return;
      if (text.includes("Failed to fetch") || text.includes("ERR_ABORTED") || text.includes("aborted")) return;
      consoleErrors.push({ text, url: message.location().url });
    }
  });
  page.on("websocket", (websocket) => {
    websocket.on("socketerror", (error) => websocketFailures.push(`${error} ${websocket.url()}`));
  });

  const routeFailures: string[] = [];
  const contentFailures: string[] = [];

  for (const route of discoverApplicationRoutes(undefined, { tenant: actor.tenantId })) {
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
      if (route === '/sona') {
        await expect(page.getByRole('alert').filter({ hasText: runtimeUnavailableMessage })).toBeVisible();
        await expect(page.getByText('No patterns recorded yet.', { exact: true })).toHaveCount(0);
      }
      if (route === '/settings') {
        await expect(page.getByText('Voice settings are unavailable in this deployment. No provider action can be started.', { exact: true })).toBeVisible();
        await expect(page.getByRole('checkbox', { name: 'Enable AI Voice Receptionist', exact: true })).toBeDisabled();
        await expect(page.getByRole('button', { name: 'Get Number', exact: true })).toHaveCount(0);
      }
      const body = await page.locator("body").innerText();
      if (/Generated Offering|AI description|mock data|fake data/i.test(body)) {
        contentFailures.push(`${route} contains fabricated data`);
      }
      if (/OneHumanCorp|One Human Corp|Powered by OHC/i.test(body.replaceAll("OmniSolo OneHumanCorp", "OmniSolo"))) {
        contentFailures.push(`${route} contains legacy branding`);
      }
    } catch (error) {
      routeFailures.push(`${route}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  await Promise.all(policyChecks);
  expect([...verifiedPolicyUrls].sort(), "the exact hosted voice and absent runtime boundaries were verified from their actual responses").toEqual([new URL("/api/v1/settings/voice", baseUrl).href, new URL("/api/v1/sona", baseUrl).href].sort());
  expect(routeFailures, "application page failures during the page crawl").toEqual([]);
  expect(contentFailures, "fabricated data or legacy branding during the page crawl").toEqual([]);
  expect(httpFailures, "unexpected HTTP 4xx responses during the page crawl").toEqual([]);
  expect(failures, "server-side 5xx responses during the page crawl").toEqual([]);
  expect(requestFailures, "failed browser requests during the page crawl").toEqual([]);
  expect(consoleErrors.filter(({ text, url }) => !isVerifiedVoicePolicyDiagnostic(text, url, verifiedPolicyUrls) && !isVerifiedRuntimePolicyDiagnostic(text, url, verifiedPolicyUrls)), "browser console errors during the page crawl").toEqual([]);
  expect(websocketFailures, "failed WebSocket upgrades during the page crawl").toEqual([]);
});

test("catalog create/read round trip uses the persisted service", async ({ anonymousPage: page }) => {
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

test("logout removes protected access and retains public route access", async ({ anonymousPage: page }) => {
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
