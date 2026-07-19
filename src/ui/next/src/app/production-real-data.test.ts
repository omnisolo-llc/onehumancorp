import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(join(__dirname, path), 'utf8');

describe('production pages fail closed instead of fabricating business outcomes', () => {
  it.each([
    ['quiz/page.tsx', /setTimeout\([^]*setIsFinished\(true\)/],
    ['giveaway/enter/page.tsx', /setTimeout\([^]*setIsEntered\(true\)/],
    ['offering/new/page.tsx', /Beginner Guitar Lesson/],
    ['edge-storefront-setup/page.tsx', /handleGenerate[^]*setStep\('success'\)/],
    ['perplexity-harness/page.tsx', /According to source \[1\]/],
    ['hybrid-landing/page.tsx', /Download Started/],
    ['win-back/page.tsx', /setTimeout\([^]*WINBACK/],
  ])('%s does not turn a local timer or canned value into success', (file, forbidden) => {
    expect(source(file)).not.toMatch(forbidden);
  });

  it('wrapped metrics do not derive or default business results', () => {
    const wrapped = source('wrapped/page.tsx');
    expect(wrapped).not.toContain('42050');
    expect(wrapped).not.toContain('pending_orders * 40');
    expect(wrapped).not.toContain('Signature Blend');
  });

  it('the builder presents only drafts returned by the backend', () => {
    const builder = source('builder/page.tsx');
    expect(builder).not.toContain('(Variant B)');
    expect(builder).not.toContain('(Variant C)');
  });

  it('triage and smart pricing do not call record-generation endpoints', () => {
    expect(source('triage/page.tsx')).not.toContain('simulate-missed-lead-btn');
    expect(source('smart-pricing/page.tsx')).not.toContain('/simulate-smart-pricing');
  });

  it('campaign and unlock pages do not report success without backend confirmation', () => {
    expect(source('cart-recovery/page.tsx')).not.toContain('setIsSent(true)');
    expect(source('loyalty-program/page.tsx')).not.toContain('setIsSent(true)');
    expect(source('work-intake-widget/page.tsx')).not.toContain('setRemoveBranding(true)');
  });

  it('campaign pages do not call the non-dispatching campaign endpoint', () => {
    for (const file of ['cart-recovery/page.tsx', 'review-campaigns/page.tsx', 'win-back/page.tsx']) {
      expect(source(file), file).not.toContain("fetch('/api/v1/growth/campaign/send'");
    }
    expect(source('cart-recovery/page.tsx')).not.toContain('setIsAutoEnabled');
  });

  it('finance does not create a canned invoice or display a fabricated draft card', () => {
    const finance = source('finance/page.tsx');
    expect(finance).not.toContain("Nora's Design Project");
    expect(finance).not.toContain('client_id: "new-client"');
    expect(finance).not.toContain('description: "Consulting Services"');
  });

  it('integrations never submit fallback credentials when the provider SDK is unavailable', () => {
    const integrations = source('integrations/page.tsx');
    expect(integrations).not.toContain('e2e-token');
    expect(integrations).not.toContain('tenant-whatsapp-id');
    expect(integrations).not.toContain('YOUR_APP_ID');
    expect(integrations).not.toContain("if (id === 'ayrshare')");
  });

  it('checkout does not grant loyalty points or discounts after an unavailable response', () => {
    const checkout = source('checkout/page.tsx');
    expect(checkout).not.toMatch(/points_balance:\s*50|setAvailablePoints\(50\)|points_balance \|\| 50/);
  });

  it('AI widgets do not invent quota or entitlement changes after sharing', () => {
    const usage = source('dashboard/AIUsageLimitWidget.tsx');
    expect(usage).not.toContain('useState(100)');
    expect(usage).not.toMatch(/setActionsUsed\(Math\.max/);
    expect(source('dashboard/AIFeaturePaywallWidget.tsx')).not.toContain("fetch('/api/v1/agents/toggle'");
  });

  it('share and voice UI do not claim unverified capabilities', () => {
    expect(source('unlock/page.tsx')).not.toContain('setIsUnlocked(true)');
    expect(source('field-ops/jobs/page.tsx')).not.toContain('(Simulated)');
    expect(source('interactive-demo/page.tsx')).not.toContain('// Fallback for tests');
    expect(source('components/PostPurchaseShareWidget.tsx')).not.toContain('setUnlocked(true)');
  });

  it('remaining branded generators do not trust local or share-only entitlement state', () => {
    const entitlementPages = [
      'social-proof-nudge/page.tsx',
      'lead-magnet-generator/page.tsx',
      'pre-order-widget/page.tsx',
      'referral-fab-builder/page.tsx',
      'business-analytics/page.tsx',
      'digital-business-card/page.tsx',
      'viral-post-generator/page.tsx',
      'share-cards/page.tsx',
      'analytics/page.tsx',
      'giveaway/page.tsx',
      'seasonal-promo/page.tsx',
      'discount-code-generator/page.tsx',
    ];

    for (const file of entitlementPages) {
      const page = source(file);
      expect(page, file).not.toMatch(/localStorage\.(?:getItem|setItem)\(['"](?:has_pro|pro_plan|trial_active|ohc_[^'"]*_shared)['"]/);
    }
    expect(source('pre-order-widget/page.tsx')).not.toContain('setTimeout(() =>');
    expect(source('digital-business-card/page.tsx')).not.toContain('setHasSharedToUnlock(true)');
    expect(source('viral-post-generator/page.tsx')).not.toContain('setHasSharedToUnlock(true)');
  });

  it('share widgets do not apply rewards without verification', () => {
    expect(source('components/ShareAndSaveWidget.tsx')).not.toContain('onShareComplete();');
    expect(source('dashboard/SuccessMilestoneWidget.tsx')).not.toContain('Copy & Share to Unlock');
    expect(source('components/ViralTrialExtensionWidget.tsx')).not.toContain('alert(');
  });

  it('booking never substitutes an E2E guest identity', () => {
    expect(source('booking/page.tsx')).not.toContain('customerEmail || "guest"');
  });

  it('does not advertise unverified referral rewards or temporary trial durations', () => {
    expect(source('wrapped/page.tsx')).not.toContain('$50 credit');
    expect(source('agents/components/AgentUpsellPaywall.tsx')).not.toContain('10% off your Pro plan');
    const entitlementFiles = [
      'social-proof-nudge/page.tsx', 'agents/page.tsx', 'components/AiTimeSavingsWidget.tsx',
      'components/AIPaywallWidget.tsx', 'components/ViralTrialExtensionWidget.tsx', 'win-back/page.tsx',
      'interactive-demo/page.tsx', 'share-cards/page.tsx', 'trial-extension/page.tsx',
      'spin-to-win-generator/page.tsx', 'cart-recovery/page.tsx', 'viral-scratch-off-generator/page.tsx',
      'giveaway/page.tsx', 'discount-code-generator/page.tsx', 'seasonal-promo/page.tsx',
    ];
    for (const file of entitlementFiles) {
      expect(source(file), file).not.toMatch(/7[- ]day|7 Days|7 days|extra week|Trial Extended/);
    }
  });

  it('does not expose simulated staff summaries or hardcoded operational activity', () => {
    expect(source('staff/manager/page.tsx')).not.toContain('/api/v1/staff/generate-summary');
    const agents = source('agents/page.tsx');
    expect(agents).not.toContain('Weekly business review');
    expect(agents).not.toContain('Weekly stats execution');
    expect(source('dashboard/page.tsx')).not.toMatch(/label: "Growth", value: "Active"/);
  });
});
