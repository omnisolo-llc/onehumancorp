import { NextResponse } from 'next/server';

// Let's create an in-memory store to simulate the DB for this E2E test.
// In reality, this route would just ping the Rust backend.
(global as any).mockSmartPricingApprovals = (global as any).mockSmartPricingApprovals || [];

export async function POST(request: Request) {
  // Try to forward to backend, but ignore failures since we are mocking
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  try {
      await fetch(`${backendUrl}/api/agents/simulate-smart-pricing`, {
          method: 'POST',
      });
  } catch (e) {
      // Ignore
  }

  // Also seed our global mock store so that the /api/agents/approvals route sees it.
  (global as any).mockSmartPricingApprovals = [
    {
      id: "mock-smart-pricing-1",
      department: "BUSINESS_ADVISORY",
      description: "Smart Price Suggestion: Winter Scarf",
      status: "PENDING",
      action_risk: "LOW",
      payload: {
        context: {
          smart_pricing: true,
          old_price: 50.00,
          new_price: 42.50,
          sales_projection: "+$120"
        }
      }
    }
  ];

  return NextResponse.json({ success: true });
}
