import { chromium, type FullConfig } from '@playwright/test';
import { Client } from 'pg';
import * as fs from 'fs';
import * as path from 'path';

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for e2e global setup.');
  }

  const dbUrl = process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc';
  if (dbUrl.startsWith('postgres')) {
    try {
      const client = new Client({ connectionString: dbUrl });
      await client.connect();
      const sql = fs.readFileSync(path.join(__dirname, 'e2e-seed.sql'), 'utf-8');
      await client.query(sql);
      await client.end();
      console.log('Successfully seeded PostgreSQL database.');
    } catch (err) {
      console.error('Failed to seed PostgreSQL:', err);
    }
  }

  // wait for app to be ready
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(new URL('/', baseURL));
      if (response.ok) return;
    } catch {
      // App is still booting.
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}
