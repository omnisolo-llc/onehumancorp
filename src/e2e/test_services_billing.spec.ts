import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: test_services_billing', async ({ page, request }) => { await currentAppSmoke(page, request, 'test_services_billing'); });
