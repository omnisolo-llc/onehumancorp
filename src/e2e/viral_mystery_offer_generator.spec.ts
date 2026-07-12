import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Viral Mystery Offer Generator', () => {
  test('dashboard links to Viral Mystery Offer Generator, which generates a viral link and soft paywall functions', async ({ page, context }) => {
    await adminPage(page, async () => {
      await page.goto('/dashboard.html');
      await page.getByRole('link', { name: 'Viral Mystery Offer Generator 🎁' }).click();
      await expect(page.getByRole('heading', { name: 'Viral Mystery Offer Generator 🎁' })).toBeVisible();

      const titleInput = page.locator('#offer-title');
      await titleInput.fill('Scratch for a Winter Deal!');
      const descInput = page.locator('#offer-desc');
      await descInput.fill('Enter your email to win big this winter!');
      const discountInput = page.locator('#offer-discount');
      await discountInput.fill('40% OFF');

      await expect(page.locator('#preview-title')).toHaveText('Scratch for a Winter Deal!');
      await expect(page.locator('#preview-desc')).toHaveText('Enter your email to win big this winter!');
      await expect(page.locator('#preview-branding')).toHaveText('⚡ Powered by OHC');

      const generateBtn = page.locator('#generate-btn');
      await generateBtn.click();

      const urlOutput = page.locator('#generated-url');
      const embedUrl = await urlOutput.inputValue();
      expect(embedUrl).toContain('Scratch%20for%20a%20Winter%20Deal!');
      expect(embedUrl).toContain('Enter%20your%20email%20to%20win%20big%20this%20winter!');
      expect(embedUrl).toContain('40%25%20OFF');
      expect(embedUrl).toContain('hideBranding=false');

      await page.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });
      const toggleInput = page.locator('#branding-toggle-label');
      await toggleInput.click();
      const modal = page.locator('#paywall-modal');
      await expect(modal).toHaveClass(/active/);
      await expect(page.locator('#paywall-modal').getByText('Upgrade to Pro')).toBeVisible();

      await page.locator('#close-paywall').click();
      await expect(modal).not.toHaveClass(/active/);

      await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });
      await toggleInput.click();
      await expect(modal).not.toHaveClass(/active/);
      await expect(page.locator('#preview-branding')).not.toBeVisible();

      await generateBtn.click();
      const newEmbedUrl = await urlOutput.inputValue();
      expect(newEmbedUrl).toContain('hideBranding=true');

      const publicPage = await context.newPage();
      await publicPage.goto(newEmbedUrl);

      await expect(publicPage.locator('#display-title')).toHaveText('Scratch for a Winter Deal!');
      await expect(publicPage.locator('#display-desc')).toHaveText('Enter your email to win big this winter!');
      await expect(publicPage.locator('#display-discount')).toHaveText('40% OFF');

      const scratchCover = publicPage.locator('#scratch-cover');
      await expect(scratchCover).toBeVisible();
      await expect(scratchCover).not.toHaveClass(/revealed/);

      const footerLink = publicPage.locator('#footer-branding');
      await expect(footerLink).not.toBeVisible();

      const emailInput = publicPage.locator('#email-input');
      await emailInput.fill('consumer@example.com');
      const revealBtn = publicPage.locator('#reveal-btn');
      await expect(revealBtn).toHaveText('Reveal My Offer');
      await revealBtn.click();

      const shareSection = publicPage.locator('#share-section');
      await expect(shareSection).toBeVisible({ timeout: 5000 });
      await expect(shareSection.locator('h3')).toHaveText('You got it! 🎉');
      await expect(scratchCover).toHaveClass(/revealed/);

      await publicPage.close();
    });
  });
});
