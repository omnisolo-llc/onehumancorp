import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: test_glassmorphism', async ({ page, request }) => { await currentAppSmoke(page, request, 'test_glassmorphism'); });
