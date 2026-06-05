import { describe, it, expect, vi } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

const mockArticles = [
  { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started-1" },
  { title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store" },
  { title: "Getting Paid", desc: "Set up how you get paid, view deposits, and handle simple taxes.", link: "/help/payments" },
  { title: "Your AI Helpers", desc: "Learn how to hire AI helpers and give them tasks to do.", link: "/help/ai-agents" },
  { title: "Finding Customers", desc: "Send emails to customers and grow your business easily.", link: "/help/marketing" },
  { title: "Account & Billing", desc: "View your bills, manage your plan, and invite team members.", link: "/help/account-billing" }
];

global.fetch = vi.fn().mockResolvedValue({
  ok: true,
  json: async () => mockArticles,
});

describe('/api/help GET', () => {
  it('returns the help articles list with specific content', async () => {
    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);

    // In next 13+, if we mock fetch, we can check the status from the real NextResponse returned by the handler
    // Wait, the handler returns the response directly.
    expect(response.status).toBe(200);
    const data = await response.json();

    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBe(6); // We know there are 6 items

    // Check specific items to ensure content is correct
    const gettingStarted = data.find((item: any) => item.link === '/help/getting-started-1');
    expect(gettingStarted).toBeDefined();
    expect(gettingStarted.title).toBe('Getting Started');
    expect(gettingStarted.desc).toContain('easily set up your store');

    const myStore = data.find((item: any) => item.link === '/help/my-store');
    expect(myStore).toBeDefined();
    expect(myStore.title).toBe('My Store');
    expect(myStore.desc).toContain('Add products');

    // Ensure all items have the required fields
    data.forEach((item: any) => {
      expect(item).toHaveProperty('title');
      expect(typeof item.title).toBe('string');
      expect(item).toHaveProperty('desc');
      expect(typeof item.desc).toBe('string');
      expect(item).toHaveProperty('link');
      expect(typeof item.link).toBe('string');
    });
  });
});
