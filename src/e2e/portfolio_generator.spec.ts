import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: portfolio_generator', async ({ page, request }) => { await currentAppSmoke(page, request, 'portfolio_generator'); });
