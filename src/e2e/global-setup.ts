import { chromium, type FullConfig } from '@playwright/test';

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for e2e global setup.');
  }

  // wait for app to be ready
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(new URL('/', baseURL));
      if (response.ok) {
        // Reset Next.js Mock API state
        try { await fetch(new URL('/api/pos/orders', baseURL), { method: 'DELETE' }); } catch (e) {}
        try { await fetch(new URL('/api/pos/inventory', baseURL), { method: 'DELETE' }); } catch (e) {}
        return;
      }
    } catch {
      // App is still booting.
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}
