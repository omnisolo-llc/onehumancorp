import { render, screen, fireEvent } from "@testing-library/react";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";
import { expect, test, vi, beforeEach } from "vitest";

// Mock global fetch
global.fetch = vi.fn();

const mockApprovals = [
  {
    id: "1",
    tenant_id: "default",
    department: "Operations",
    description: "3 new orders to fulfill.",
    status: "PENDING",
    action_risk: "LOW",
    payload: { feature_type: "fulfillment_batch" }
  },
  {
    id: "2",
    tenant_id: "default",
    department: "Business Advisory",
    description: "It's been 30 days since your last promo.",
    status: "PENDING",
    action_risk: "HIGH",
    payload: { feature_type: "promo_advisory", draft_content: "Enjoy 20% off!" }
  },
  {
    id: "3",
    tenant_id: "default",
    department: "Marketing",
    description: "Review social post.",
    status: "PENDING",
    action_risk: "HIGH",
    payload: {
      feature_type: "social_post",
      image_url: "http://example.com/img.png",
      caption: "Check out our new cake!"
    }
  },
  {
    id: "4",
    tenant_id: "default",
    department: "Sales",
    description: "Draft quote for client.",
    status: "PENDING",
    action_risk: "LOW",
    payload: {
      feature_type: "quote_draft",
      suggested_price: 150.0,
      customer_inquiry: "Need a quote"
    }
  }
];

beforeEach(() => {
  vi.clearAllMocks();
  (global.fetch as any).mockImplementation((url: string) => {
    if (url.includes("/api/agents/approvals/ledger")) {
        return Promise.resolve({
            ok: true,
            json: () => Promise.resolve({ entries: [] })
        });
    }
    if (url.includes("/api/agents/approvals")) {
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ pending_approvals: mockApprovals })
      });
    }
    return Promise.reject(new Error("Unknown URL"));
  });
});

test("renders agent proposals cards", async () => {
  render(<UnifiedAgentFeed />);

  const card1 = await screen.findByText(/3 new orders to fulfill/i);
  const card2 = screen.getByText(/It's been 30 days since your last promo/i);

  expect(card1).toBeDefined();
  expect(card2).toBeDefined();
});

test("Operations card has massive Fulfill Now button", async () => {
  render(<UnifiedAgentFeed />);
  const button = await screen.findByTestId("approve-fulfillment");
  expect(button.textContent).toContain("Fulfill Now");
  expect(button.className).toContain("min-h-[56px]");
});

test("Advisory card expands when 'Yes, draft it' is clicked", async () => {
  render(<UnifiedAgentFeed />);
  const draftButton = await screen.findByTestId("approve-draft");

  // Content should be hidden initially
  expect(screen.queryByText(/Enjoy 20% off!/i)).toBeNull();

  fireEvent.click(draftButton);

  // Now it should be visible
  expect(screen.getByText(/Enjoy 20% off!/i)).toBeDefined();

  // Button should change to Approve & Send
  expect(screen.getByTestId("approve-send-promo")).toBeDefined();
});

test("Marketing card expands when 'Review Post' is clicked", async () => {
  render(<UnifiedAgentFeed />);
  const reviewButton = await screen.findByTestId("approve-social-post");

  expect(screen.queryByText(/Check out our new cake!/i)).toBeNull();

  fireEvent.click(reviewButton);

  expect(screen.getByText(/Check out our new cake!/i)).toBeDefined();
  expect(screen.getByAltText(/Social post preview/i)).toBeDefined();
});

test("Sales card shows quote preview", async () => {
  render(<UnifiedAgentFeed />);
  const quotePreview = await screen.findByText(/Proposed Quote: \$150/i);
  expect(quotePreview).toBeDefined();
});
