import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: chaos_resilience', async ({ page, request }) => { await currentAppSmoke(page, request, 'chaos_resilience'); });
