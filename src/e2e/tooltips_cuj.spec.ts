import { test, expect } from "@playwright/test";

test.describe("Tooltip Registry CUJ", () => {
  test("creates, updates, and deletes a tooltip", async ({ page }) => {
    // Navigate to the tooltip registry
    await page.goto("/tooltip-registry.html");

    // Wait for the page to load
    await page.waitForLoadState("networkidle");

    // Enter a new tooltip id and text
    const testTooltipId = "test-e2e-tooltip-id";
    const testTooltipText = "Test E2E Tooltip Text";

    await page.fill("#new-id", testTooltipId);
    await page.fill("#new-text", testTooltipText);

    // Click Add
    await page.click("button:has-text('Add Tooltip')");

    // Wait for the success toast and for it to disappear
    await expect(page.locator(".ohc-toast", { hasText: "Tooltip added successfully" })).toBeVisible();
    await expect(page.locator(".ohc-toast")).not.toBeVisible({ timeout: 5000 });

    // Verify it's in the list
    await expect(page.locator(`tr:has-text("${testTooltipId}")`)).toBeVisible();
    await expect(page.locator(`#input-${testTooltipId}`)).toHaveValue(testTooltipText);

    // Update the tooltip
    const updatedTooltipText = "Updated Tooltip Text";
    await page.fill(`#input-${testTooltipId}`, updatedTooltipText);

    // Click Save for that row
    const row = page.locator(`tr:has-text("${testTooltipId}")`);
    await row.locator("button:has-text('Save')").click();

    // Wait for the success toast and for it to disappear
    await expect(page.locator(".ohc-toast", { hasText: "Tooltip updated successfully" })).toBeVisible();
    await expect(page.locator(".ohc-toast")).not.toBeVisible({ timeout: 5000 });

    // Delete the tooltip
    // Handle the browser confirm dialog
    page.on('dialog', dialog => dialog.accept());
    await row.locator("button:has-text('Delete')").click();

    // Wait for the success toast
    await expect(page.locator(".ohc-toast", { hasText: "Tooltip deleted successfully" })).toBeVisible();

    // Verify it's removed from the list
    await expect(page.locator(`tr:has-text("${testTooltipId}")`)).not.toBeVisible();
  });
});
