import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: help-features', async ({ page, request }) => { await currentAppSmoke(page, request, 'help-features'); });
