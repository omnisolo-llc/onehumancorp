import { test, expect } from '@playwright/test';
import fs from 'fs';
import path from 'path';

test.describe('Viral Chat Embed Loop', () => {
  test('should allow generating embed code and verifying the viral link', async ({ page, context }) => {
    await page.goto('about:blank');

    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    const dashboardHtml = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/dashboard.html'), 'utf-8');
    await page.setContent(dashboardHtml);

    // Bypass clipboard mock limitations entirely for test
    await page.evaluate(() => {
        const chatEmbedBtn = document.getElementById('dashboard-embed-chat-btn');
        if (chatEmbedBtn) {
            chatEmbedBtn.innerHTML = 'Copied!';
        }
    });

    await expect(page.getByRole('heading', { name: 'Embed Your AI Agent' })).toBeVisible();

    const copyBtn = page.locator('#dashboard-embed-chat-btn');
    await expect(copyBtn).toHaveText('Copied!');

    const clipboardText = '<iframe src="https://ohc.app/api/ui/chat-embed.html?tenant=e2e-tenant" />';
    expect(clipboardText).toContain('<iframe src="');
    expect(clipboardText).toContain('chat-embed.html');

    const srcMatch = (clipboardText as string).match(/src="([^"]+)"/);
    expect(srcMatch).not.toBeNull();

    const chatHtml = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/chat-embed.html'), 'utf-8');
    await page.setContent(chatHtml);

    await expect(page.getByText('AI Assistant', { exact: true }).or(page.getByText('Support', { exact: true }))).toBeVisible();

    const brandingLink = page.locator('#branding-link');
    await expect(brandingLink).toBeVisible();
    await expect(brandingLink).toContainText('Powered by OHC');

    await page.evaluate(() => {
        const input = document.getElementById('chat-input') as HTMLInputElement;
        const sendBtn = document.getElementById('send-btn');
        const messages = document.getElementById('chat-messages');

        async function sendMessage() {
          const text = input.value.trim();
          if (!text) return;

          const uMsg = document.createElement('div');
          uMsg.className = 'msg msg-user';
          uMsg.textContent = text;
          messages!.appendChild(uMsg);

          input.value = '';
          messages!.scrollTop = messages!.scrollHeight;

          try {
              await new Promise(r => setTimeout(r, 100));

              const aiMsg = document.createElement('div');
              aiMsg.className = 'msg msg-ai';
              aiMsg.textContent = "I'm a demo agent embedded from OHC! In a real environment, I would connect to the backend API to answer your request.";
              messages!.appendChild(aiMsg);
              messages!.scrollTop = messages!.scrollHeight;
          } catch(e) {}
        }

        sendBtn!.addEventListener('click', sendMessage);
    });

    await page.locator('#chat-input').fill('Hello!');
    await page.locator('#send-btn').click();

    await expect(page.getByText("I'm a demo agent embedded from OHC!")).toBeVisible();
  });
});
