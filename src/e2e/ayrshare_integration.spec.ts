import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: ayrshare_integration', async ({ page, request }) => { await currentAppSmoke(page, request, 'ayrshare_integration'); });
