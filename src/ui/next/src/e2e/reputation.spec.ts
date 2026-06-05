import { test, expect } from '@playwright/test';

test.describe('Reputation & Review Engine', () => {
  const TENANT_ID = 'e2e-reputation-tenant';

  test.beforeEach(async ({ page }) => {
    await page.addInitScript((tenant) => {
      localStorage.setItem('tenant', tenant);
      localStorage.setItem('tenant_id', tenant);
      localStorage.setItem('token', 'e2e-mock-token');
    }, TENANT_ID);
  });

  test('CUJ: Automatically handle incoming review and approve AI draft', async ({ page, request }) => {
    const reviewId = `rev-${Date.now()}`;
    const webhookRes = await request.post('/api/v1/local_seo/webhook', {
      data: {
        tenant_id: TENANT_ID,
        review_id: reviewId,
        reviewer_name: 'Carlos Handyman',
        star_rating: 4,
        comment: 'Great work, highly recommended!',
        platform: 'Google'
      }
    });

    expect(webhookRes.ok()).toBeTruthy();

    await page.route('/api/v1/local_seo/reviews/pending*', async (route) => {
      await route.fulfill({
        status: 200,
        json: [
          {
            review_id: reviewId,
            reviewer_name: 'Carlos Handyman',
            star_rating: 4,
            comment: 'Great work, highly recommended!',
            ai_draft_reply: 'Hi Carlos, thank you so much for the 4-star review! We are glad you had a great experience.',
            reply_status: 'PENDING'
          }
        ]
      });
    });

    let approveCalled = false;
    await page.route(`/api/v1/local_seo/reviews/${reviewId}/approve`, async (route) => {
      approveCalled = true;
      await route.fulfill({
        status: 200,
        json: { status: 'success', review_id: reviewId }
      });
    });

    await page.goto('/dashboard');
    await expect(page.getByText('Reputation Inbox')).toBeVisible();

    await page.click('text=Reputation Inbox');
    await expect(page).toHaveURL(/\/reviews/);

    await expect(page.getByText('Carlos Handyman')).toBeVisible();
    await expect(page.getByText('Great work, highly recommended!')).toBeVisible();
    await expect(page.getByText('Hi Carlos, thank you so much for the 4-star review!')).toBeVisible();

    const approveBtn = page.getByRole('button', { name: 'Approve & Post' });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    expect(approveCalled).toBeTruthy();
    await expect(page.getByText('Carlos Handyman')).not.toBeVisible();
    await expect(page.getByText('All Caught Up!')).toBeVisible();
  });
});
