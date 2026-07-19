import fs from 'node:fs';
import path from 'node:path';
import { expect, test } from './fixtures';

const repoRoot = process.env.SOURCE_REPO_ROOT || path.resolve(__dirname, '../..');
const productionRoots = ['src/ui/next/src/app']
  .map((root) => path.join(repoRoot, root));

const ignoredPathFragments = [
  `${path.sep}e2e${path.sep}`,
  `${path.sep}test${path.sep}`,
  `${path.sep}tests${path.sep}`,
  `${path.sep}coverage${path.sep}`,
  `${path.sep}target${path.sep}`,
  `${path.sep}.next${path.sep}`,
  '.test.',
  '_test.rs',
  '.spec.',
];

const seededProduct = {
  id: 'e2e-product-cake',
  title: 'Vegan Celebration Cake',
  description: 'Plant-based celebration cake for local pickup.',
  item_type: 'physical',
  price_cents: 3999,
  inventory_count: 12,
  image_url: '/dashboard_with_charts.png',
} as const;

function walkFiles(dir: string): string[] {
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  return entries.flatMap((entry) => {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) return walkFiles(fullPath);
    return entry.isFile() ? [fullPath] : [];
  });
}

function isProductionSource(file: string) {
  if (!/\.(ts|tsx|rs)$/.test(file)) return false;
  if (ignoredPathFragments.some((fragment) => file.includes(fragment))) return false;
  return true;
}

test.describe('real data contract', () => {
  test('renders the stable PostgreSQL-seeded product returned by the real catalog API', async ({ page }) => {
    const catalogResponsePromise = page.waitForResponse((response) => {
      const url = new URL(response.url());
      return url.pathname === '/api/v1/catalog/products' && response.request().method() === 'GET';
    });

    await page.goto('/products');

    const catalogResponse = await catalogResponsePromise;
    expect(catalogResponse.status()).toBe(200);
    const products = await catalogResponse.json();
    expect(Array.isArray(products)).toBeTruthy();
    expect(products).toContainEqual(expect.objectContaining(seededProduct));

    const seededProductRow = page.locator('.app-list-item').filter({ hasText: seededProduct.title });
    await expect(seededProductRow).toBeVisible();
    await expect(seededProductRow).toContainText('$39.99');

    const seededProductImage = seededProductRow.getByRole('img', { name: seededProduct.title });
    await expect(seededProductImage).toHaveAttribute('src', seededProduct.image_url);
    await expect.poll(async () => seededProductImage.evaluate((image: HTMLImageElement) => ({
      complete: image.complete,
      naturalWidth: image.naturalWidth,
      naturalHeight: image.naturalHeight,
    }))).toEqual({
      complete: true,
      naturalWidth: 1280,
      naturalHeight: 720,
    });
  });

  test('Rust server does not own browser application pages', async () => {
    expect(fs.existsSync(path.join(repoRoot, 'src/server/lib.rs')), 'Production source files are not available in this Bazel Playwright runfiles tree.').toBeTruthy();
    const serverLib = fs.readFileSync(path.join(repoRoot, 'src/server/lib.rs'), 'utf8');
    const forbiddenPatterns = [
      /async\s+fn\s+ui_handler\b/,
      /<!DOCTYPE html>/i,
      /axum::response::Html/,
      /\.fallback\(\s*ui_handler\s*\)/,
      /\.route\("\/(?:business-setup|website-builder|brand-studio|login|agents|team|meetings|dashboard|inbox|inventory|orders|products\/new|share-cards|win-back|seasonal-promo|help|api-docs|changelog|kairos|services\/new)"/,
    ];

    const violations = forbiddenPatterns
      .filter((pattern) => pattern.test(serverLib))
      .map((pattern) => pattern.toString());

    expect(violations).toEqual([]);
  });

  test('mutating Next API routes delegate to real services instead of hardcoded success', async () => {
    const violations: string[] = [];
    const routeFiles = walkFiles(path.join(repoRoot, 'src/ui/next/src/app/api'))
      .filter((file) => /route\.tsx?$/.test(file))
      .filter(isProductionSource);

    for (const file of routeFiles) {
      const relative = path.relative(repoRoot, file);
      const source = fs.readFileSync(file, 'utf8');
      if (!/export\s+async\s+function\s+POST\b/.test(source)) continue;

      const delegatesToService = [
        /\bfetch\(/,
        /\b(?:proxyCurrentBackendPath|proxyBackend(?:Get|Post|Put|Request))\b/,
        /\bPool\b|\bpg\b|\bsqlx\b/i,
        /process\.env\.[A-Z0-9_]*(URL|DSN|ENDPOINT|HOST)/,
        /BACKEND_URL|OHC_BACKEND_URL|OHC_API_URL/,
      ].some((pattern) => pattern.test(source));
      const failsClosed = /status:\s*(501|503)/.test(source);

      if (!delegatesToService && !failsClosed) {
        violations.push(`${relative}: POST handler does not call a backend, database, or fail closed`);
      }
    }

    expect(violations).toEqual([]);
  });

  test('production UI controls do not use alert-only handlers', async () => {
    const violations: string[] = [];
    const controlAlertPatterns = [
      /<(button|a)\b[^>]*(onClick|onclick)=["'{][^\n>]*\balert\s*\(/,
    ];
    const files = new Set<string>();

    for (const root of productionRoots) {
      for (const file of walkFiles(root).filter(isProductionSource)) {
        files.add(file);
      }
    }
    files.add(path.join(repoRoot, 'src/server/lib.rs'));

    for (const file of files) {
      const relative = path.relative(repoRoot, file);
      const source = fs.readFileSync(file, 'utf8');
      const lines = source.split('\n');

      lines.forEach((line, index) => {
        if (controlAlertPatterns.some((pattern) => pattern.test(line))) {
          violations.push(`${relative}:${index + 1}: ${line.trim()}`);
        }
      });
    }

    expect(violations).toEqual([]);
  });
});
