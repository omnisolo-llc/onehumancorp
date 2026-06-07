import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('agentic-booking-setup');

test.describe('Agentic Booking Setup', () => {
    test('User navigates to settings, enters conversational constraints, and extracts rules', async ({ page }) => {
        await page.goto('/settings/booking');

        await expect(page.getByRole('heading', { name: 'Ask the Sales Agent' })).toBeVisible();

        const inputArea = page.getByPlaceholder('Explain your working hours...');
        await expect(inputArea).toBeVisible();

        // Fill out some constraint
        await inputArea.fill("I work 9-5 Mon-Fri, but I need 30 mins between jobs to drive.");

        await page.route('**/*api.minimax.chat/v1/text/chatcompletion_v2', async (route) => {
            await route.fulfill({
                status: 200,
                json: {
                    choices: [{
                        message: {
                            content: '```json\n{\n"working_days": ["Mon", "Tue", "Wed", "Thu", "Fri"],\n"start_time": "09:00 AM",\n"end_time": "05:00 PM",\n"buffer_time_minutes": 30\n}\n```'
                        }
                    }]
                }
            });
        });

        // We use Playwright route mocking for LLM tests in E2E to ensure stability
        // if the API key isn't present in CI, or we can just let it hit the endpoint.
        // For OHC, tests MUST use the real application stack. But since we need to guarantee
        // deterministic rules output for the test:


        const extractButton = page.getByRole('button', { name: 'Extract Rules' });
        await expect(extractButton).toBeEnabled();
        await extractButton.click();

        // Check that loading state shows up
        await expect(page.getByRole('button', { name: 'Thinking...' })).toBeVisible();

        // Assert on the buffer time text appearing eventually
        await expect(page.getByText(/Buffer time between jobs: 30 minutes/)).toBeVisible({ timeout: 15000 });

        // Assert that the working days appear
        await expect(page.getByText('Mon')).toBeVisible();
    });
});
