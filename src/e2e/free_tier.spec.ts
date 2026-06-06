import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: free_tier', async ({ page, request }) => { await currentAppSmoke(page, request, 'free_tier'); });
