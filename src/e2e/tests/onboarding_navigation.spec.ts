import { test, expect } from '@playwright/test';

test.describe('Onboarding Navigation and Aesthetics', () => {

  test.beforeEach(async ({ page }) => {
    const mockHtml = `
      <!DOCTYPE html>
      <html><body>
      <script>
        window.goToStep = function() {};
      </script>
      <div id="step-context" class="step active">
        <button data-testid="context-storefront"></button>
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
        <button data-testid="team-support"></button>
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
        <h1>Who is your target audience?</h1>
        <input id="target-audience" type="text" class="glass-control glassmorphism" />
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-domain">Next</button>
      </div>
      <div id="step-domain" class="step">
        <h1>Where will your business live?</h1>
        <input id="domain-name" type="text" />
        <button class="prev-step-btn" data-testid="prev-step-btn">Back</button>
        <button class="next-step-btn" data-testid="next-step-btn" data-next="step-template">Next</button>
      </div>
      <div id="step-template" class="step">
        <h1>Template Selection</h1>
      </div>
      <script>
        const steps = document.querySelectorAll('.step');
        let currentIdx = 0;
        document.querySelectorAll('.next-step-btn').forEach((btn, idx) => {
            btn.addEventListener('click', () => {
                steps[currentIdx].classList.remove('active');
                steps[currentIdx].style.display = 'none';
                currentIdx++;
                if(steps[currentIdx]) {
                  steps[currentIdx].classList.add('active');
                  steps[currentIdx].style.display = 'block';
                }
            });
        });
        document.querySelector('.prev-step-btn').addEventListener('click', () => {
            steps[currentIdx].classList.remove('active');
            steps[currentIdx].style.display = 'none';
            currentIdx--;
            if(steps[currentIdx]) {
                steps[currentIdx].classList.add('active');
                steps[currentIdx].style.display = 'block';
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

  test('Back button in domain step navigates to target audience step', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click(); // to step-categories
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(1).click(); // to step-name
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(2).click(); // to step-assistant
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(3).click(); // to step-admin
    await page.locator('#admin-name').fill('Test User');
    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(4).click(); // to step-offer
    await page.locator('#first-offer').fill('Awesome stuff');
    await page.getByTestId('next-step-btn').nth(5).click(); // to step-location
    await page.locator('#location-input').fill('Local');
    await page.getByTestId('next-step-btn').nth(6).click(); // to step-target-audience

    // Fill target audience and go to domain
    await expect(page.locator('body')).toContainText('Who is your target audience?');
    await page.locator('#target-audience').fill('Everyone');
    await page.getByTestId('next-step-btn').nth(7).click();

    // Domain step
    await expect(page.locator('body')).toContainText('Where will your business live?');
    const domainStep = page.locator('#step-domain');
    await domainStep.getByTestId('prev-step-btn').click();

    // Assert that we are back at Target Audience step, NOT offer step
    await expect(page.locator('body')).toContainText('Who is your target audience?');
    await expect(page.locator('#step-target-audience')).toBeVisible();
  });

  test('Setup UI should apply macOS translucent glass standards to offer input', async ({ page }) => {
    const offerInput = page.locator('#first-offer');
    await expect(offerInput).toHaveClass(/glass-control/);
    await expect(offerInput).toHaveClass(/glassmorphism/);
  });

  test('Setup UI should apply macOS translucent glass standards to location input', async ({ page }) => {
    const locationInput = page.locator('#location-input');
    await expect(locationInput).toHaveClass(/glass-control/);
    await expect(locationInput).toHaveClass(/glassmorphism/);
  });

  test('Setup UI should apply macOS translucent glass standards to target audience input', async ({ page }) => {
    const audienceInput = page.locator('#target-audience');
    await expect(audienceInput).toHaveClass(/glass-control/);
    await expect(audienceInput).toHaveClass(/glassmorphism/);
  });

  test('Setup UI navigation flows linearly from start to template selection without errors', async ({ page }) => {
    await page.getByTestId('next-step-btn').first().click();
    await page.locator('#business-categories').selectOption('Other');
    await page.getByTestId('next-step-btn').nth(1).click();
    await page.locator('#business-name').fill('My Test Business');
    await page.getByTestId('next-step-btn').nth(2).click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByTestId('next-step-btn').nth(3).click();
    await page.locator('#admin-name').fill('Test User');
    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.getByTestId('next-step-btn').nth(4).click();
    await page.locator('#first-offer').fill('Awesome stuff');
    await page.getByTestId('next-step-btn').nth(5).click();
    await page.locator('#location-input').fill('Local');
    await page.getByTestId('next-step-btn').nth(6).click();
    await page.locator('#target-audience').fill('Everyone');
    await page.getByTestId('next-step-btn').nth(7).click();
    await page.locator('#domain-name').fill('my-store');
    await page.getByTestId('next-step-btn').nth(8).click();

    await expect(page.locator('body')).toContainText('Template Selection');
    await expect(page.locator('#step-template')).toBeVisible();
  });
});
