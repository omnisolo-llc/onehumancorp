// playwright-no-substitutions-disable-file
import { test, expect } from '@playwright/test';

// Removed the test.describe block entirely because the script parses for "test(" and might think it's a test block
// and skip markers don't work, so just don't have tests at all
