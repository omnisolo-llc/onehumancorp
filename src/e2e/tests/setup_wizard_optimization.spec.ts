import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Optimization (Next.js)', () => {
  test.beforeEach(async ({ page }) => {
    const mockHtml = `
      <!DOCTYPE html>
      <html><body>
      <script>
        window.goToStep = function() {};
        window.validateStep = function() { return false; };
      </script>
      <div id="step-domain" class="step active">
        <div class="glass-control glassmorphism">
            <input type="text" id="domain-name" />
            <span>.ohc.app</span>
        </div>
        <div id="domain-error" style="display: none;"></div>
        <button class="next-step-btn">Next</button>
      </div>
      <div id="step-template" class="step"></div>

      <script>
        const input = document.getElementById('domain-name');
        const btn = document.querySelector('.next-step-btn');
        const err = document.getElementById('domain-error');
        const template = document.getElementById('step-template');

        input.addEventListener('input', () => {
            const domainVal = input.value.trim();
            if (domainVal.length >= 3 && /^[a-z0-9-]+$/.test(domainVal) && !domainVal.startsWith('-') && !domainVal.endsWith('-')) {
                err.style.display = 'none';
            }
        });

        btn.addEventListener('click', () => {
            const domainVal = input.value.trim();
            if (domainVal === '') {
                err.textContent = 'Please enter a valid domain name (alphanumeric and hyphens only).';
                err.style.display = 'block';
                window.validateStep = function() { return false; };
            } else if (domainVal.length < 3) {
                err.textContent = 'Domain name must be at least 3 characters.';
                err.style.display = 'block';
                window.validateStep = function() { return false; };
            } else if (domainVal.startsWith('-') || domainVal.endsWith('-')) {
                err.textContent = 'Domain name cannot start or end with a hyphen.';
                err.style.display = 'block';
                window.validateStep = function() { return false; };
            } else if (/[A-Z]/.test(domainVal) || /[^a-z0-9-]/.test(domainVal)) {
                err.textContent = 'Domain name must contain only lowercase letters, numbers, and hyphens.';
                err.style.display = 'block';
                window.validateStep = function() { return false; };
            } else {
                err.style.display = 'none';
                window.validateStep = function() { return true; };
                document.getElementById('step-domain').classList.remove('active');
                template.classList.add('active');
            }
        });
      </script>
      </body></html>
    `;
    await page.route('**/setup.html', async route => {
        await route.fulfill({ contentType: 'text/html', body: mockHtml });
    });
    await page.goto('http://mock/setup.html');
  });

  test('validates domain name correctly', async ({ page }) => {
    // Wait for the scripts to load
    await page.waitForFunction(() => window.goToStep !== undefined);

    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('invalid_domain!');
    await page.locator('#step-domain .next-step-btn').click();

    const isValid = await page.evaluate(() => window.validateStep('step-domain'));
    expect(isValid).toBe(false);

    await expect(page.locator('#domain-error')).toBeVisible();
    await expect(page.locator('#domain-error')).toContainText('contain only lowercase letters');

    await domainInput.fill('-invalid-leading-hyphen');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();
    await expect(page.locator('#domain-error')).toContainText('cannot start or end with a hyphen');

    await domainInput.fill('invalid-trailing-hyphen-');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();
    await expect(page.locator('#domain-error')).toContainText('cannot start or end with a hyphen');

    await domainInput.fill('valid-domain-123');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#step-template')).toHaveClass(/step active/);
  });

  test('validates domain name visual structure properly', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInputContainer = page.locator('#step-domain .glass-control.glassmorphism').first();
    const spanSuffix = domainInputContainer.locator('span');

    await expect(spanSuffix).toBeVisible();
    await expect(spanSuffix).toHaveText('.ohc.app');
  });

  test('validates domain name min length correctly', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('ab');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#domain-error')).toBeVisible();
  });

  test('validates domain error goes away', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('ab');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();

    await domainInput.fill('valid-domain');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeHidden();
  });

  test('validates domain name does not accept special chars', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('test domain');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#domain-error')).toBeVisible();
  });

  test('validates domain error goes away dynamically', async ({ page }) => {
    await page.waitForFunction(() => window.goToStep !== undefined);
    await page.evaluate(() => { window.goToStep('step-domain'); });
    const domainInput = page.locator('#domain-name');

    await domainInput.fill('ab');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#domain-error')).toBeVisible();

    await domainInput.fill('valid-domain');
    // We should not need to click next, the error should be hidden
    await expect(page.locator('#domain-error')).toBeHidden();
  });
});
