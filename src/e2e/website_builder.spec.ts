import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: website_builder', async ({ page, request }) => { await currentAppSmoke(page, request, 'website_builder'); });
