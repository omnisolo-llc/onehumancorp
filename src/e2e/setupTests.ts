import { Page } from '@playwright/test';

export async function setupMockContext(page: Page) {
    // Setup logic for E2E tests
}

export async function getTenantId(page: Page) {
    return '12345678-1234-1234-1234-123456789012';
}
