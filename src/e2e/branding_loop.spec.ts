import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: branding_loop', async ({ page, request }) => { await currentAppSmoke(page, request, 'branding_loop'); });
