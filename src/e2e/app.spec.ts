import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: app', async ({ page, request }) => { await currentAppSmoke(page, request, 'app'); });
