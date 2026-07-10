import { TooltipProvider } from "../../../components/TooltipRegistry";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import CampaignOrchestrationPage from "./page";

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    prefetch: vi.fn(),
  }),
  usePathname: () => '',
  useSearchParams: () => new URLSearchParams(),
}));

const responses: Record<string, unknown> = {
  "/api/ui/dashboard/unified-feed": {
    metrics: {
      active_customers: 42,
      pending_orders: 2,
      total_sales: 1840,
      total_campaigns_sent: 7,
    },
    orders: [
      {
        id: "order-1001",
        customer_name: "Alice",
        customer_email: "alice@example.com",
        total_amount: 68.5,
        status: "delivered",
      },
    ],
    inbox: [
      {
        id: "msg-1",
        source: "chat",
        content: "Can you remind me what I bought last time?",
        status: "open",
      },
    ],
    supply: {
      vendors: [],
      raw_materials: [
        { id: "flour", name: "Flour", current_quantity: 2, reorder_threshold: 4 },
      ],
      bom_items: [],
    },
  },
};

function jsonResponse(data: unknown) {
  return Promise.resolve({
    ok: true,
    json: () => Promise.resolve(data),
  });
}

describe("CampaignOrchestrationPage", () => {
  it("loads tenant-scoped dashboard data and exposes campaign orchestration paths", async () => {
    localStorage.setItem("tenant_id", "tenant-123");
    const fetchMock = vi.fn((input: RequestInfo | URL, init?: RequestInit) => {
      const url = String(input);
      if (url.includes("/api/v1/growth/campaign/generate-review")) {
        return jsonResponse({ message: "Hi Alice, please review order-1001" });
      }

      const match = Object.keys(responses).find((path) => url.includes(path));
      if (match) return jsonResponse(responses[match]);

      return jsonResponse({});
    });
    global.fetch = fetchMock as any;

    render(
      <TooltipProvider>
        <CampaignOrchestrationPage />
      </TooltipProvider>,
    );

    await waitFor(() => {
      expect(screen.queryByText("Campaign Orchestration")).not.toBeNull();
    });

    expect(fetchMock).toHaveBeenCalledWith("/api/ui/dashboard/unified-feed?tenant_id=tenant-123");
    // expect(screen.getByText("42")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /Open review workflow/i }).getAttribute("href")).toBe("/review-campaigns");
    expect(screen.getByRole("link", { name: /Open receipt workflow/i }).getAttribute("href")).toMatch(/\/orders(\/order-1001)?/);

    fireEvent.click(screen.getByRole("button", { name: /Generate review draft/i }));

    await waitFor(() => {
      expect(screen.queryByText(/Hi Alice, please review order-1001/)).not.toBeNull();
    });
    expect(fetchMock).toHaveBeenCalledWith(
      "/api/v1/growth/campaign/generate-review",
      expect.objectContaining({
        method: "POST",
        body: JSON.stringify({
          customer_name: "Alice",
          customer_email: "alice@example.com",
          order_id: "order-1001",
          product_name: "recent purchase",
          tenant_id: "tenant-123",
        }),
      }),
    );
  });
});
