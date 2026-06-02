import { test, expect } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('Cross-Device Onboarding', () => {
  test('should sync onboarding state across different browser contexts', async ({ request }) => {
    const tenantId = `tenant-${uuidv4()}`;
    const userId = `user-${uuidv4()}`;

    const draftState = {
      wizardState: {
        step: 2,
        chatStep: 1,
        businessName: 'Java Beans Coffee',
        whatYouSell: 'Coffee and pastries',
        location: 'Seattle, WA',
        businessType: 'Cafe',
        categories: ['food', 'beverage'],
        websiteTemplate: 'Modern',
        firstProductName: '',
        firstProductPrice: '',
        aiAgents: [],
        aiAutoRespond: true
      }
    };

    const baseUrl = test.info().project.use.baseURL || 'http://localhost:8080';

    // Device 1 saves draft
    const saveRes = await request.post(`${baseUrl}/api/v1/onboarding/draft`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': userId
      },
      data: draftState
    });

    // Note: The UI fetches from Next.js server (/api/onboarding/draft) which forwards to the Rust server (/api/onboarding/state or /api/onboarding/draft).
    // The Rust backend routes are set up without the /v1 prefix in `api::onboarding::router`, so they might be at /api/onboarding/draft.
    // Wait, `router` is nested under `/api/onboarding`.
    // Let's use the Rust API directly via the base URL since the Playwright setup launches the Rust server.
    const directSaveRes = await request.post(`${baseUrl}/api/onboarding/state`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': userId
      },
      data: draftState
    });

    expect(directSaveRes.ok()).toBeTruthy();

    // Device 2 retrieves draft
    const loadRes = await request.get(`${baseUrl}/api/onboarding/state`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': userId
      }
    });

    expect(loadRes.ok()).toBeTruthy();

    const data = await loadRes.json();
    expect(data.businessName).toBe('Java Beans Coffee');
    expect(data.location).toBe('Seattle, WA');
  });
});