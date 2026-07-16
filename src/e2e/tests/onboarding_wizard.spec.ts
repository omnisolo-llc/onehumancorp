import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Logic', () => {

  test.beforeEach(async ({ page }) => {
    const mockHtml = `
      <!DOCTYPE html>
      <html><body>
      <div id="step-context" class="step active">
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-categories">Next</button>
      </div>
      <div id="step-categories" class="step">
        <select id="business-categories"><option value="Other">Other</option></select>
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-name">Next</button>
      </div>
      <div id="step-name" class="step">
        <input id="business-name" type="text" />
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-assistant">Next</button>
      </div>
      <div id="step-assistant" class="step">
        <select id="assistant-tone"><option value="Professional">Professional</option></select>
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-admin">Next</button>
      </div>
      <div id="step-admin" class="step">
        <input id="admin-name" type="text" />
        <input id="admin-email" type="text" />
        <input id="admin-password" type="password" />
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-offer">Next</button>
      </div>
      <div id="step-offer" class="step">
        <input id="first-offer" type="text" class="glass-control glassmorphism" />
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-location">Next</button>
      </div>
      <div id="step-location" class="step">
        <input id="location-input" type="text" class="glass-control glassmorphism" />
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-target-audience">Next</button>
      </div>
      <div id="step-target-audience" class="step">
        <input id="target-audience" type="text" class="glass-control glassmorphism" />
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-domain">Next</button>
      </div>
      <div id="step-domain" class="step">
        <input id="domain-name" type="text" />
        <button class="prev-step-btn" data-testid="prev-step-btn">Back</button>
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-template">Next</button>
      </div>
      <div id="step-template" class="step"></div>
      <script>
        const steps = document.querySelectorAll('.step');
        let currentIdx = 0;
        document.querySelectorAll('.next-step-btn').forEach((btn) => {
            btn.addEventListener('click', () => {
                steps[currentIdx].classList.remove('active');
                currentIdx++;
                if(steps[currentIdx]) steps[currentIdx].classList.add('active');
            });
        });
      </script>
      </body></html>
    `;
    await page.route('**/setup.html', async route => {
        await route.fulfill({ contentType: 'text/html', body: mockHtml });
    });
    await page.goto('http://mock/setup.html');
  });

  test('navigates correctly', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.locator('#business-name').fill('Test');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(3).click();
    await page.locator('#admin-name').fill('Test');
    await page.locator('#admin-email').fill('t@t.com');
    await page.locator('#admin-password').fill('pass');
    await page.getByTestId('next-step-btn').nth(4).click();
    await page.locator('#first-offer').fill('Offer');
    await page.getByTestId('next-step-btn').nth(5).click();
    await page.locator('#location-input').fill('Loc');
    await page.getByTestId('next-step-btn').nth(6).click();
    await page.locator('#target-audience').fill('Aud');
    await page.getByTestId('next-step-btn').nth(7).click();

    await expect(page.locator('#step-domain')).toHaveClass(/active/);
  });
});
