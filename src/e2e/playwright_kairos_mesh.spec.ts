import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test.describe("Smoke", () => { currentAppSmoke('playwright_kairos_mesh'); });
