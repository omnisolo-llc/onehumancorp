import { test, expect } from './fixtures';

test.describe('Viral Giveaway Generator', () => {
  test('should generate giveaway embed code', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/ui/viral-giveaway-generator.html');
    await expect(page.getByRole('heading', { name: 'Viral Giveaway Generator 🎁' })).toBeVisible();

    await page.locator('#giveaway-title').fill('Win a Free Cake for a Year!');
    await page.locator('#giveaway-description').fill('Enter to win. Share with friends for extra entries!');
    await page.locator('#extra-entries').fill('5');

    await page.getByRole('button', { name: 'Generate Widget' }).click();

    const embedCode = page.locator('#embed-code');
    await expect(embedCode).toBeVisible();

    const embedValue = await embedCode.innerText();
    expect(embedValue).toContain('<iframe src="');
    expect(embedValue).toContain('/api/v1/growth/giveaway/embed');
    expect(embedValue).toContain('title=Win%20a%20Free%20Cake%20for%20a%20Year!');
    expect(embedValue).toContain('entries=5');
    expect(embedValue).toContain('⚡ Powered by OHC');

    const copyBtn = page.locator('#copy-btn');
    await expect(copyBtn).toHaveText('Copy to Clipboard');
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');
  });
});
