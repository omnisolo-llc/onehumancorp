import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: lens_audit', async ({ page, request }) => { await currentAppSmoke(page, request, 'lens_audit'); });
