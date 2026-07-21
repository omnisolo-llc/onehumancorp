import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, test, expect, vi, beforeEach } from "vitest";
import UnifiedTriageFeed from "./page";
import { TooltipProvider } from "../../components/TooltipRegistry";

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push: vi.fn(), replace: vi.fn() }),
  usePathname: () => "/triage",
  useSearchParams: () => new URLSearchParams(),
}));

describe("UnifiedTriageFeed @mobile", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        items: [
          {
            id: "1",
            tenant_id: "test",
            source: "Instagram DM",
            customer_id: "sarah_bakes",
            priority: "High",
            context: "vegan chocolate cakes? I am ready to pay.",
            action_type: "Send Quote",
            action_payload: "Hi Sarah! I have availability on Tuesday. A custom vegan chocolate cake starts at $50. Should I send over a deposit link?",
            created_at: new Date().toISOString(),
          },
        ],
      }),
    });
  });

  test("Maya captures a lead, reviews an AI-drafted reply, and approves the quote all from the 375px feed", async () => {
    render(<TooltipProvider><UnifiedTriageFeed /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.getByText("sarah_bakes")).toBeInTheDocument();
    });

    const header = screen.getByTestId("triage-card-header-1");
    fireEvent.click(header);

    expect(screen.getByText("Proposed Action: Send Quote")).toBeInTheDocument();

    const approveButton = screen.getByTestId("triage-approve-1");
    expect(approveButton).toBeInTheDocument();

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ success: true }),
    });

    fireEvent.click(approveButton);

    await waitFor(() => {
        expect(screen.queryByTestId("triage-card-1")).not.toBeInTheDocument();
        expect(screen.getByText("All caught up! You're a hero.")).toBeInTheDocument();
    });
  });
});
