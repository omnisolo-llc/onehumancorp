import { ROUTES, SELECTORS, TEST_DATA } from './constants';
import { expect, Page } from '@playwright/test';

export async function completeWizardFromStep3(page: Page) {
    await page.fill('input[placeholder="What is your business called?"]', 'Checklist Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');
    await page.click('button:has-text("Next")');
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');
    await page.click('text="✨ Modern"');
    await page.click('button:has-text("Next")');
    await page.click('text="Get a free sub-domain"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="Your Full Name"]', 'Jane Doe');
    await page.fill('input[placeholder="your@email.com"]', 'jane@example.com');
    await page.fill('input[placeholder="Create a strong password"]', 'securepass123');
    await page.click('button:has-text("Review & Launch")');
    await expect(page.locator('text="Almost there"')).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Launch!")');
    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 10000 });
}

export async function loginUser(page: Page) {
    await page.goto(ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').first().fill(TEST_DATA.EMAIL);
    await page.locator('input[type="password"]').first().fill(TEST_DATA.PASSWORD);
    await page.locator(SELECTORS.LOGIN_BTN).first().click();
}

export async function setupCompanyToTemplate(page: Page) {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click(SELECTORS.NEXT_ARROW);
    await page.click('text=📦 Physical products');
    await page.click(SELECTORS.NEXT_ARROW);
    await page.click('text=🌐 Online only');
    await page.click(SELECTORS.NEXT_ARROW);
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', TEST_DATA.PASSWORD);
    await page.click(SELECTORS.NEXT_ARROW);
}

export async function setupCompanyToTemplateFast(page: Page) {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click(SELECTORS.NEXT_ARROW);
    await page.click('text=📦 Physical products');
    await page.click(SELECTORS.NEXT_ARROW);
    await page.click('text=🌐 Online only');
    await page.click(SELECTORS.NEXT_ARROW);
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', TEST_DATA.PASSWORD);
}

export async function setupCompanyToStep3(page: Page) {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click(SELECTORS.NEXT_ARROW);
}

export async function setupCompanyToTemplateHero(page: Page) {
    await page.click('text=🚀 Start My Business');
    await expect(page.locator('text=What kind of business are you building?')).toBeVisible();
    await page.click('text=🛒 Online Store');
    await page.click(SELECTORS.NEXT_ARROW);

    await expect(page.locator('text=Give your business a name')).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company Hero');
    await page.click(SELECTORS.NEXT_ARROW);

    await expect(page.locator('text=What do you sell?')).toBeVisible();
    await page.click('text=📦 Physical products');
    await page.click(SELECTORS.NEXT_ARROW);

    await expect(page.locator('text=How do you want to receive payments?')).toBeVisible();
    await page.click('text=🌐 Online only');
    await page.click(SELECTORS.NEXT_ARROW);

    await expect(page.locator('text=Create your account')).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', TEST_DATA.PASSWORD);
    await page.click(SELECTORS.NEXT_ARROW);

    await expect(page.locator('text=Choose a Template')).toBeVisible();
    await page.click('text=✨ Modern');
    await page.click(SELECTORS.NEXT_ARROW);
}

export async function finishWizardWithTestCake(page: Page) {
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click(SELECTORS.NEXT_ARROW);
    await page.click('text=🌐 Free OHC Domain');
    await page.click(SELECTORS.NEXT_ARROW);
    await expect(page.locator('text="Publish my business →"')).toBeVisible();
    await page.click('text="Publish my business →"');
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });
}
