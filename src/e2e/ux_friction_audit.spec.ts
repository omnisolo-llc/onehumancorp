import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test.describe("Smoke", () => { currentAppSmoke('ux_friction_audit'); });
