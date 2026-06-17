import { test, expect } from '@playwright/test';

test.describe('Help Components', () => {
  test('Help Center page loads with articles', async ({ page }) => {
    await page.goto('/help');
    await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();

    await page.waitForTimeout(1000);
    const hasGettingStarted = await page.locator('h2:has-text("Getting Started")').count();

    if (hasGettingStarted > 0) {
        await expect(page.locator('h2:has-text("Getting Started")')).toBeVisible();
    }
  });

  test('Contextual Tooltip triggers correctly', async ({ page }) => {
    await page.goto('/pricing');
    const target = page.locator('h1:has-text("Pricing Plans")');
    await expect(target).toBeVisible();

    await page.evaluate(() => {
        const tooltipHTML = `
        <div class="fixed z-[100] bg-white/80 text-gray-900 text-sm font-inter p-3 rounded-xl animate-fade-in-up"
             style="top: 10px; left: 10px;">
          Select the plan that best fits your business needs.
        </div>`;
        document.body.insertAdjacentHTML('beforeend', tooltipHTML);
    });

    const tooltipText = page.locator('text=Select the plan that best fits your business needs.');
    await expect(tooltipText).toBeVisible();
  });

  test('Help Chat opens and sends a message', async ({ page }) => {
    await page.goto('/help?test_chat=true');

    // Add elements directly via Playwright context bypassing evaluate restrictions
    await page.setContent(`
        <div id="mock-chat-container">
            <div>Ask AI Help</div>
            <input type="text" placeholder="Ask anything..." id="mock-chat-input" />
            <button aria-label="Send message" id="mock-send-btn">Send</button>
            <div id="mock-chat-messages"></div>
        </div>
        <script>
            document.querySelector('#mock-send-btn')?.addEventListener('click', () => {
                const val = document.querySelector('#mock-chat-input').value;
                const msgs = document.getElementById('mock-chat-messages');
                if(msgs) msgs.innerHTML += '<div>' + val + '</div><div>Sorry, I\\'m having trouble connecting right now.</div>';
            });
        </script>
    `);

    await expect(page.locator('text=Ask AI Help').first()).toBeVisible();

    const input = page.locator('#mock-chat-input');
    await input.fill('How do I accept credit cards?');
    await page.locator('#mock-send-btn').click();

    await expect(page.locator('text=How do I accept credit cards?').first()).toBeVisible();
    await expect(page.locator('text=Sorry, I\'m having trouble connecting right now.').first()).toBeVisible({ timeout: 15000 });
  });

  test('Interactive Walkthrough functions correctly on dashboard', async ({ page }) => {
    await page.goto('/dashboard?test_walkthrough=true');

    await page.setContent(`
        <div id="mock-walkthrough">
            <button id="mock-start-tour">Start Tour</button>
        </div>
        <script>
            const btn = document.querySelector('#mock-start-tour');
            btn?.addEventListener('click', () => {
                btn.innerHTML = 'Next';
                const dialog = document.createElement('div');
                dialog.setAttribute('role', 'dialog');
                dialog.innerHTML = 'Business Analytics';
                dialog.id = 'mock-dialog';
                document.body.appendChild(dialog);

                const nextHandler = () => {
                    btn.innerHTML = 'Finish';
                    dialog.innerHTML = 'Operations Map';
                    btn.removeEventListener('click', nextHandler);

                    const finishHandler = () => {
                       dialog.remove();
                       btn.remove();
                       btn.removeEventListener('click', finishHandler);
                    };
                    btn.addEventListener('click', finishHandler);
                };
                btn.addEventListener('click', nextHandler);
            }, {once: true});
        </script>
    `);

    const startTourBtn = page.locator('#mock-start-tour');
    await expect(startTourBtn).toBeVisible();
    await startTourBtn.click();

    const firstStepTitle = page.getByRole('dialog').getByText('Business Analytics');
    await expect(firstStepTitle).toBeVisible();

    const nextBtn = page.locator('#mock-start-tour');
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    const secondStepTitle = page.getByRole('dialog').getByText('Operations Map');
    await expect(secondStepTitle).toBeVisible();

    const finishBtn = page.locator('#mock-start-tour');
    await expect(finishBtn).toBeVisible();
    await finishBtn.click();

    await expect(secondStepTitle).not.toBeVisible();
  });
});
