import { test, expect } from '@playwright/test';

test.describe('WhatsApp Cloud API Flow CUJ', () => {
  test.beforeEach(async ({ page }) => {
    test.setTimeout(300000);
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await expect(page.getByRole('heading', { name: /Dashboard/i }).first()).toBeVisible({ timeout: 30000 });
  });

  test('Owner connects WhatsApp Cloud API', async ({ page }) => {
    await page.goto('/integrations');
    const whatsappCard = page.locator('h3', { hasText: 'WhatsApp Cloud API' }).locator('..');

    // Connect or Manage button
    const actionBtn = whatsappCard.getByRole('button');
    const btnText = await actionBtn.textContent();

    if (btnText?.includes('Connect')) {
      await actionBtn.click();
      await expect(page.getByRole('heading', { name: /Connect WhatsApp Cloud API/i })).toBeVisible();

      const continueBtn = page.getByRole('button', { name: /Continue with Meta/i });
      await expect(continueBtn).toBeVisible();

      // Because the real flow relies on `window.FB` which doesn't exist in Playwright without a mock,
      // we'll hit the connection endpoint directly as the UI would.
      const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || 'http://localhost:18789';

      // Send the request with headers required by the backend
      const response = await page.request.post(`${apiBase}/api/v1/settings/integrations/whatsapp_cloud_api`, {
        data: {
          api_token: 'test_meta_token',
          phone_number_id: 'test_phone_id',
          display_phone_number: 'tenant-whatsapp-id'
        },
        headers: {
            "x-tenant-id": "e2e-tenant",
            "Content-Type": "application/json"
        }
      });
      expect(response.ok()).toBeTruthy();

      await page.reload();
      const updatedBtn = whatsappCard.getByRole('button');
    } else {
      expect(btnText).toContain('Manage');
    }
  });

  test('Owner receives a WhatsApp Cloud API text message and it appears in inbox', async ({ page, request }) => {
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || 'http://localhost:18789';

    // Send a mock Meta webhook
    const payload = {
        "entry": [
            {
                "changes": [
                    {
                        "value": {
                            "metadata": {
                                "display_phone_number": "tenant-whatsapp-id"
                            },
                            "messages": [
                                {
                                    "from": "0987654321",
                                    "text": {
                                        "body": "Hello! I would like to order a meta cake over WhatsApp."
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    };

    const response = await request.post(`${apiBase}/api/v1/webhooks/meta`, {
      headers: {
        'Content-Type': 'application/json',
      },
      data: payload,
    });
    expect(response.ok()).toBeTruthy();

    await page.goto('/inbox');
    await expect(page.getByText(/Hello! I would like to order a meta cake over WhatsApp/i).first()).toBeVisible({ timeout: 15000 });
  });

  test('Owner receives a WhatsApp Cloud API message with image media', async ({ page, request }) => {
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || 'http://localhost:18789';

    const payload = {
        "entry": [
            {
                "changes": [
                    {
                        "value": {
                            "metadata": {
                                "display_phone_number": "tenant-whatsapp-id"
                            },
                            "messages": [
                                {
                                    "from": "0987654321",
                                    "image": {
                                        "id": "img123",
                                        "caption": "Look at this cake"
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    };

    const response = await request.post(`${apiBase}/api/v1/webhooks/meta`, {
      headers: {
        'Content-Type': 'application/json',
      },
      data: payload,
    });
    expect(response.ok()).toBeTruthy();

    await page.goto('/inbox');
    // Check if the markdown image syntax is parsed or displayed
    await expect(page.getByText(/Look at this cake/i).first()).toBeVisible({ timeout: 15000 });
  });

  test('Owner receives a WhatsApp Cloud API message with audio media', async ({ page, request }) => {
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || 'http://localhost:18789';

    const payload = {
        "entry": [
            {
                "changes": [
                    {
                        "value": {
                            "metadata": {
                                "display_phone_number": "tenant-whatsapp-id"
                            },
                            "messages": [
                                {
                                    "from": "0987654321",
                                    "audio": {
                                        "id": "audio123"
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    };

    const response = await request.post(`${apiBase}/api/v1/webhooks/meta`, {
      headers: {
        'Content-Type': 'application/json',
      },
      data: payload,
    });
    expect(response.ok()).toBeTruthy();

    await page.goto('/inbox');
    await expect(page.getByText(/\[Audio\]\(audio123\)/i).first()).toBeVisible({ timeout: 15000 });
  });

});