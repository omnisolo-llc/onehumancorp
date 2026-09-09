import { expect, test } from "../../../../e2e/fixtures";

test.describe("OmniSolo browser branding", () => {
  test("login is a standalone OmniSolo surface", async ({ anonymousPage: page }) => {
    await page.goto("/login?next=/dashboard");

    await expect(page).toHaveTitle(/OmniSolo/);
    await expect(page.getByRole("heading", { name: "Sign in to OmniSolo OneHumanCorp" })).toBeVisible();
    await expect(page.getByRole("navigation", { name: "Primary" })).toHaveCount(0);
  });

  test("the public login document contains no legacy first-party branding", async ({ page }) => {
    const response = await page.request.get("/login?next=/dashboard");
    expect(response.ok()).toBe(true);
    const document = await response.text();

    expect(document.replaceAll("OmniSolo OneHumanCorp", "OmniSolo")).not.toMatch(/one human corp|onehumancorp|ohc\.app|ohc\.store|api\.onehumancorp/i);
    expect(document).toContain("OmniSolo");
  });

  test("browser-owned state uses only OmniSolo storage namespaces", async ({ page }) => {
    const affectedRoutes = [
      "/verify-email",
      "/pos/terminal",
      "/pos/kds",
      "/storefront-builder",
      "/website-builder",
      "/digital-business-card",
      "/referrals",
    ];
    const violations: string[] = [];

    for (const route of affectedRoutes) {
      await page.goto(route, { waitUntil: "domcontentloaded" });
      await page.waitForTimeout(250);
      const routeViolations = await page.evaluate(() => {
        const persisted = [localStorage, sessionStorage].flatMap((storage) =>
          Array.from({ length: storage.length }, (_, index) => storage.key(index) ?? "")
            .filter((key) => /^ohc(?:_|-)/i.test(key))
        );
        return [...new Set(persisted)];
      });
      violations.push(...routeViolations.map((key) => `${route}: ${key}`));
    }

    expect(violations).toEqual([]);
  });

  test("login does not start authenticated optional-content requests", async ({ anonymousPage }) => {
    const optionalRequests: string[] = [];
    anonymousPage.on("request", (request) => {
      if (/\/api\/v1\/(help|videos|tooltips)$/.test(new URL(request.url()).pathname)) {
        optionalRequests.push(request.url());
      }
    });

    await anonymousPage.goto("/login?next=/dashboard");
    await expect(anonymousPage.getByRole("heading", { name: "Sign in to OmniSolo OneHumanCorp" })).toBeVisible();
    await anonymousPage.waitForTimeout(500);

    expect(optionalRequests).toEqual([]);
  });

  test("mobile help stays available on Help Center but avoids product collisions", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });

    await page.goto("/help");
    await expect(page.getByRole("button", { name: "Open help chat" })).toBeVisible();

    await page.goto("/website-builder");
    await expect(page.getByRole("button", { name: "Open help chat" })).toBeHidden();
  });

  test("customer referral embeds use the canonical cloud origin", async ({ page }) => {
    await page.goto("/customer-referral-program");
    await expect(page.getByRole("heading", { name: "Customer Referral Program" })).toBeVisible();

    await page.getByRole("button", { name: "Generate Widget Embed" }).click();

    const embedCode = page.locator("pre");
    await expect(embedCode).toContainText("https://cloud.omnisolo.co/api/v1/growth/customer-referral/embed");
    await expect(embedCode).toContainText("Powered by OmniSolo");
    await expect(embedCode).not.toContainText(/one human corp|onehumancorp|ohc\.app|ohc\.store|powered by ohc/i);
  });

  test("the shared footer exposes OmniSolo copy on a growth page", async ({ page }) => {
    await page.goto("/project-showcase");

    const footer = page.getByRole("link", { name: /powered by omnisolo/i });
    await expect(footer).toBeVisible();
    await footer.hover();
    await expect(page.getByText(/built with omnisolo/i)).toBeVisible();
  });

  test("the menu generator has no external font requests", async ({ page }) => {
    const externalFontRequests: string[] = [];
    await page.setViewportSize({ width: 390, height: 844 });
    page.on("request", (request) => {
      if (/fonts\.(googleapis|gstatic)\.com/i.test(request.url())) {
        externalFontRequests.push(request.url());
      }
    });

    await page.goto("/menu-generator");
    await expect(page.getByRole("heading", { name: "Menu Details" })).toBeVisible();
    await page.waitForTimeout(500);

    expect(externalFontRequests).toEqual([]);
  });

  test("the wrapped snapshot has no external font requests", async ({ page }) => {
    const externalFontRequests: string[] = [];
    await page.setViewportSize({ width: 1440, height: 1000 });
    page.on("request", (request) => {
      if (/fonts\.(googleapis|gstatic)\.com/i.test(request.url())) {
        externalFontRequests.push(request.url());
      }
    });

    await page.goto("/wrapped");
    await expect(page.getByText("Your OmniSolo Snapshot")).toBeVisible();
    await page.waitForTimeout(500);

    expect(externalFontRequests).toEqual([]);
  });

  test("the WhatsApp link generator has no external font requests", async ({ page }) => {
    const externalFontRequests: string[] = [];
    page.on("request", (request) => {
      if (/fonts\.(googleapis|gstatic)\.com/i.test(request.url())) {
        externalFontRequests.push(request.url());
      }
    });

    await page.goto("/whatsapp-link-generator");
    await expect(page.getByRole("heading", { name: "WhatsApp Link Generator 📱" })).toBeVisible();
    await page.waitForTimeout(500);

    expect(externalFontRequests).toEqual([]);
  });

  test("audited pages have no external font requests", async ({ page }) => {
    test.setTimeout(180_000);
    const affectedRoutes = [
      "/work-intake-widget",
      "/referrals",
      "/scribe-mission-track",
      "/viral-job-board-generator",
      "/gift-cards",
      "/flash-sale-generator",
      "/proposal-generator",
      "/cart-recovery",
      "/pos/kds",
      "/viral-coupon-unlock",
      "/quote-calculator",
      "/giveaway/enter",
      "/milestone-alerts",
      "/event-rsvp-builder",
    ];
    const externalFontRequests: string[] = [];
    page.on("request", (request) => {
      if (/fonts\.(googleapis|gstatic)\.com/i.test(request.url())) {
        externalFontRequests.push(request.url());
      }
    });

    for (const viewport of [
      { width: 1440, height: 1000 },
      { width: 390, height: 844 },
    ]) {
      await page.setViewportSize(viewport);
      for (const route of affectedRoutes) {
        await page.goto(route, { waitUntil: "domcontentloaded" });
        await page.waitForTimeout(250);
      }
    }

    expect(externalFontRequests).toEqual([]);
  });

  test("web feed uses authenticated HTTP polling without opening an unauthenticated socket", async ({ page }) => {
    const socketErrors: string[] = [];
    const socketUrls: string[] = [];
    const feedRequests: string[] = [];
    page.on("console", (message) => {
      if (message.type() === "error" && /WebSocket connection.*\/api\/v1\/feed\/ws/i.test(message.text())) {
        socketErrors.push(message.text());
      }
    });
    page.on("websocket", (socket) => socketUrls.push(socket.url()));
    page.on("request", (request) => {
      const url = new URL(request.url());
      if (url.pathname === "/api/v1/agent-feed" && request.method() === "GET") {
        feedRequests.push(request.url());
      }
    });

    await page.goto("/feed");
    await expect.poll(() => feedRequests.length, { timeout: 7_000 }).toBeGreaterThan(1);

    expect(socketErrors).toEqual([]);
    expect(socketUrls).toEqual([]);
  });

  test("the POS route renders its catalog and cart interaction", async ({ page }) => {
    await page.goto("/pos");
    await expect(page.getByRole("heading", { name: "POS Terminal" })).toBeVisible();
    await expect(page.getByText("Custom Cake")).toBeVisible();

    await page.getByRole("button", { name: /Custom Cake/ }).click();
    await expect(page.getByText("Cart (1)")).toBeVisible();
    await expect(page.getByText("1x Custom Cake")).toBeVisible();
  });

  test("the global commerce route renders through the shared app shell", async ({ page }) => {
    await page.goto("/settings/global-commerce");
    await expect(page.getByText("Global Commerce")).toBeVisible();
    await expect(page.getByText("Base Currency")).toBeVisible();
    await expect(page.locator("select").first()).toHaveValue(/^(USD|EUR|GBP|CAD|AUD|JPY)$/);
  });

  test("an invalid customer quote uses a quiet not-found surface", async ({ page }) => {
    const consoleErrors: string[] = [];
    const requestedUrls: string[] = [];
    page.on("console", (message) => {
      if (message.type() === "error") consoleErrors.push(message.text());
    });
    page.on("request", (request) => requestedUrls.push(request.url()));

    await page.goto("/quote/visual-audit-id");
    await expect(page.getByText("Quote not found.")).toBeVisible();

    expect(requestedUrls.some((url) => url.endsWith("/api/v1/quotes/visual-audit-id"))).toBe(false);
    expect(requestedUrls.some((url) => url.endsWith("/api/quotes/visual-audit-id"))).toBe(false);
    expect(consoleErrors.filter((message) => /fetch quote|error fetching quote/i.test(message))).toEqual([]);
  });

  test("an invalid internal quote does not issue a malformed API request", async ({ page }) => {
    const consoleErrors: string[] = [];
    const requestedUrls: string[] = [];
    page.on("console", (message) => {
      if (message.type() === "error") consoleErrors.push(message.text());
    });
    page.on("request", (request) => requestedUrls.push(request.url()));

    await page.goto("/quotes/visual-audit-id");
    await expect(page.getByText("Quote not found")).toBeVisible();

    expect(requestedUrls.some((url) => url.endsWith("/api/v1/quotes/visual-audit-id"))).toBe(false);
    expect(consoleErrors.filter((message) => /400 \(Bad Request\)|failed to fetch quote/i.test(message))).toEqual([]);
  });

  test("the mobile POS route renders with a query-string tenant", async ({ page }) => {
    await page.goto("/pos/mpos?tenant_id=e2e-tenant");
    await expect(page.getByRole("heading", { name: "mPOS" })).toBeVisible();
    await expect(page.getByText("Vegan Celebration Cake")).toBeVisible();
    await expect(page.getByText("$39.99")).toBeVisible();
    await expect(page.getByRole("button", { name: "Quick Charge" })).toBeDisabled();
  });
});
