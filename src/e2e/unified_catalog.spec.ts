import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: unified_catalog', async ({ page, request }) => { await currentAppSmoke(page, request, 'unified_catalog'); });
