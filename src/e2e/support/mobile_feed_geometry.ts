import { expect, type Locator, type Page } from '@playwright/test';

/** Measure the loaded card and every rendered control, including expanded editors. */
export async function expectMobileCardGeometry(page: Page, card: Locator) {
  await expect(card).toBeVisible();
  const viewport = page.viewportSize();
  expect(viewport?.width).toBe(375);
  const width = viewport!.width;
  const cardBox = await card.boundingBox();
  expect(cardBox).not.toBeNull();
  expect(cardBox!.x).toBeGreaterThanOrEqual(0);
  expect(cardBox!.x + cardBox!.width).toBeLessThanOrEqual(width);
  expect(await card.evaluate(element => element.scrollWidth)).toBeLessThanOrEqual(
    await card.evaluate(element => element.clientWidth),
  );
  expect(await page.evaluate(() => document.body.scrollWidth)).toBeLessThanOrEqual(width);
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(width);

  const controls = card.locator('button:visible, a:visible, input:visible, textarea:visible, select:visible');
  expect(await controls.count()).toBeGreaterThan(0);
  for (const control of await controls.all()) {
    const box = await control.boundingBox();
    expect(box, await control.getAttribute('data-testid') || await control.textContent() || 'card control').not.toBeNull();
    expect(box!.width).toBeGreaterThanOrEqual(44);
    expect(box!.height).toBeGreaterThanOrEqual(44);
    expect(box!.x).toBeGreaterThanOrEqual(cardBox!.x);
    expect(box!.x + box!.width).toBeLessThanOrEqual(cardBox!.x + cardBox!.width);
  }
}
