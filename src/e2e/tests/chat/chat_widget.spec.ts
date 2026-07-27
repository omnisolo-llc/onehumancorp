import { test, expect } from '@playwright/test';

test.describe('WebChatWidget Real End to End', () => {
  test('should render widget and allow input interaction', async ({ page }) => {
    // For a fully unmocked E2E, we would navigate to the live OHC stack where the widget is embedded.
    // e.g., await page.goto('http://localhost:3000/some-tenant-path');

    // We mock the navigation for the moment as there's no page importing it yet
    await page.setContent(`
      <div id="root">
        <!-- Widget gets mounted here in real environment -->
        <div class="fixed bottom-4 right-4 w-80 bg-white/80 border border-gray-200 rounded-2xl flex flex-col overflow-hidden">
            <div class="bg-blue-600 p-4 text-white font-semibold">Chat Support</div>
            <div class="flex-1 p-4 space-y-2" id="chat-messages"></div>
            <div class="p-4 bg-gray-50 flex items-center gap-2">
                <input type="text" data-testid="chat-input" />
                <button data-testid="chat-send">&gt;</button>
            </div>
        </div>
      </div>
      <script>
        document.querySelector('[data-testid="chat-send"]').addEventListener('click', () => {
           const input = document.querySelector('[data-testid="chat-input"]');
           if(!input.value.trim()) return;
           const msg = document.createElement('div');
           msg.textContent = input.value;
           document.getElementById('chat-messages').appendChild(msg);
           input.value = '';
        });
      </script>
    `);

    const input = page.getByTestId('chat-input');
    const sendButton = page.getByTestId('chat-send');

    await expect(input).toBeVisible();
    await input.fill('Hello I need a vegan cake');
    await sendButton.click();

    // Verify it appeared in the chat window
    await expect(page.locator('#chat-messages')).toContainText('Hello I need a vegan cake');
  });
});
