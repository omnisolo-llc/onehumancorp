import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: business_setup_2', async ({ page, request }) => { await currentAppSmoke(page, request, 'business_setup_2'); });
