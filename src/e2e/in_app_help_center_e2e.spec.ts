import { test, expect } from "@playwright/test";

test("Help Center: Access from Home Page", async ({ page }) => {
  await page.goto("/");
  await page.click("text=Login");
  await page.fill("input[name=username]", "test");
  await page.click("button[type=submit]");
  await page.click("button[aria-label='Help']");
  await expect(page.locator(".help-portal")).toBeVisible();
});

test("Help Center: Search Functionality", async ({ page }) => {
  await page.goto("/");
  await page.click("text=Login");
  await page.fill("input[name=username]", "test");
  await page.click("button[type=submit]");
  await page.click("button[aria-label='Help']");
  await page.fill("input[placeholder='Search help...']", "payment");
  await page.click("button[type=submit]");
  await expect(page.locator(".search-results")).toContainText("payment");
});

test("Help Center: Mobile View Toggle", async ({ page }) => {
  await page.goto("/");
  await page.click("text=Login");
  await page.fill("input[name=username]", "test");
  await page.click("button[type=submit]");
  await page.click("button[aria-label='Help']");
  await page.setViewportSize({ width: 375, height: 812 });
  await expect(page.locator(".help-portal-mobile")).toBeVisible();
});

test("Help Center: Read Article", async ({ page }) => {
  await page.goto("/");
  await page.click("text=Login");
  await page.fill("input[name=username]", "test");
  await page.click("button[type=submit]");
  await page.click("button[aria-label='Help']");
  await page.click("text=Getting Started");
  await expect(page.locator(".article-content")).toBeVisible();
});

test("Help Center: Contact Support", async ({ page }) => {
  await page.goto("/");
  await page.click("text=Login");
  await page.fill("input[name=username]", "test");
  await page.click("button[type=submit]");
  await page.click("button[aria-label='Help']");
  await page.click("text=Still need help?");
  await expect(page.locator(".support-form")).toBeVisible();
});
