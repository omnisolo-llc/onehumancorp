import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { expect, it } from 'vitest';
import { metadata as multilingual } from './multilingual-order-interceptor/layout';
import { metadata as quiz } from './quiz-generator/layout';
import { metadata as referral } from './referral-fab-builder/layout';
import { metadata as subscriptions } from './subscriptions/manage/layout';
import { metadata as proposals } from './proposals/layout';
import { metadata as whatsapp } from './whatsapp-link-generator/layout';
import { metadata as tips } from './tip-jar/layout';
import { metadata as tiers } from './viral-tier-list-generator/layout';
import { metadata as exitIntent } from './exit-intent-builder/layout';
import { metadata as testimonials } from './testimonial-widget/layout';

const routes = [
  ['multilingual-order-interceptor', 'Multilingual Order Interceptor | OmniSolo OneHumanCorp'],
  ['quiz-generator', 'Viral Quiz Generator'],
  ['referral-fab-builder', 'Referral FAB Builder | OmniSolo OneHumanCorp'],
  ['subscriptions/manage', 'Manage Subscriptions | OmniSolo OneHumanCorp'],
  ['proposals', 'Inquiries & Proposals | OHC'],
  ['whatsapp-link-generator', 'WhatsApp Link Generator | OmniSolo OneHumanCorp'],
  ['tip-jar', 'Tip Jar Widget Builder | OmniSolo OneHumanCorp'],
  ['viral-tier-list-generator', 'Viral Tier List Generator'],
  ['exit-intent-builder', 'Exit-Intent Pop-up Builder | OmniSolo OneHumanCorp'],
  ['testimonial-widget', 'Testimonial Widget Builder | OmniSolo OneHumanCorp'],
] as const;

const metadata = { 'multilingual-order-interceptor': multilingual, 'quiz-generator': quiz,
  'referral-fab-builder': referral, 'subscriptions/manage': subscriptions, proposals,
  'whatsapp-link-generator': whatsapp, 'tip-jar': tips, 'viral-tier-list-generator': tiers,
  'exit-intent-builder': exitIntent, 'testimonial-widget': testimonials };

it.each(routes)('provides server metadata for /%s without a competing client head', (route, title) => {
  expect(metadata[route].title).toBe(title);
  const page = readFileSync(join(process.cwd(), 'src/app', route, 'page.tsx'), 'utf8');
  expect(page).not.toMatch(/from\s+['"]next\/head['"]|<Head\b/);
});
