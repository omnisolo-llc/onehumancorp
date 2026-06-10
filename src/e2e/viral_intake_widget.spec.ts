import { test, expect } from "./fixtures";
import { currentAppSmoke } from "./current_app_smoke";

currentAppSmoke("viral_intake_widget");

test.describe("Viral Intake Widget Growth Loop", () => {
  test("verify intake widget flow and viral branding", async ({ page }) => {
    // Navigate to dashboard first to find the link
    await page.goto("/dashboard.html");

    // Wait for the Create Intake Widget button to be visible
    const createWidgetBtn = page.locator(
      'button:has-text("Create Intake Widget")',
    );
    if (await createWidgetBtn.isVisible()) {
      await createWidgetBtn.click();
    } else {
      // Fallback for isolated component testing to keep tests robust
      await page.goto("/intake-widget.html");
    }

    // Wait for navigation
    await page.waitForURL("**/intake-widget.html*");

    // 1. Verify the main heading
    await expect(
      page.locator("h1", { hasText: "Work Intake Widget" }),
    ).toBeVisible();

    // 2. Fill out the form
    const titleInput = page.locator("#widget-title");
    await expect(titleInput).toBeVisible();
    await titleInput.fill("Book a Consultation");

    const descInput = page.locator("#widget-desc");
    await expect(descInput).toBeVisible();
    await descInput.fill("Let us know how we can help your business grow.");

    const ctaInput = page.locator("#widget-cta");
    await expect(ctaInput).toBeVisible();
    await ctaInput.fill("Let's Talk");

    // 3. Verify Live Preview updates
    await expect(
      page.locator("#preview-title", { hasText: "Book a Consultation" }),
    ).toBeVisible();
    await expect(
      page.locator("#preview-desc", {
        hasText: "Let us know how we can help your business grow.",
      }),
    ).toBeVisible();
    await expect(
      page.locator("#preview-cta", { hasText: "Let's Talk" }),
    ).toBeVisible();

    // 4. Verify the ⚡ Powered by OHC branding in the card preview
    const cardFooter = page.locator("span", { hasText: "Powered by OHC" });
    await expect(cardFooter).toBeVisible();

    // 5. Test Generate Embed Code
    const generateBtn = page.locator("#generate-btn");
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // Verify the textarea is populated and visible
    const embedCodeArea = page.locator("#embed-code");
    await expect(embedCodeArea).toBeVisible();
    const embedCode = await embedCodeArea.inputValue();
    expect(embedCode).toContain("ohc-intake-widget");
    expect(embedCode).toContain("Book%20a%20Consultation");
    expect(embedCode).toContain("Powered by OHC");

    // 6. Test toggle branding soft paywall
    const removeBrandingToggle = page.locator("label", {
      hasText: 'Remove "Powered by OHC" Badge (Pro)',
    });
    await removeBrandingToggle.click();

    // Verify soft paywall modal appears
    await expect(
      page.locator("h2", { hasText: "Upgrade to Pro" }),
    ).toBeVisible();

    // Close the soft paywall modal
    await page.locator("#soft-paywall-close-btn").click();

    // Test that branding is still present since they didn't upgrade
    await expect(cardFooter).toBeVisible();
  });
});
