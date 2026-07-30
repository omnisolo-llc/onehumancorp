import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { vi } from "vitest";
import { AssistantInsightsWidget } from "./AssistantInsightsWidget";

describe("AssistantInsightsWidget", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("renders loading state initially", () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ items: [] }),
    });

    render(<AssistantInsightsWidget />);
    expect(screen.getByText("Assistant Insights")).toBeInTheDocument();
    expect(screen.getByText("Loading your insights...")).toBeInTheDocument();
  });

  it("renders null if there are no items", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ items: [] }),
    });

    const { container } = render(<AssistantInsightsWidget />);

    await waitFor(() => {
      expect(screen.queryByText("Loading your insights...")).not.toBeInTheDocument();
    });

    expect(container.firstChild).toBeNull();
  });

  it("renders up to 3 insight items and allows approval", async () => {
    const mockItems = [
      {
        id: "item-1",
        intent: "Follow up",
        customer_info: { name: "Maya" },
        suggested_actions: [{ action_type: "Draft email", message: "Draft quote for Carlos" }],
        status: "PENDING",
      },
      {
        id: "item-2",
        intent: "Cart Recovery",
        customer_info: { name: "Priya" },
        suggested_actions: [{ action_type: "Send SMS", message: "Follow up on abandoned cart for Priya" }],
        status: "PENDING",
      }
    ];

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ items: mockItems }),
    });

    render(<AssistantInsightsWidget />);

    await waitFor(() => {
      expect(screen.getByTestId("assistant-insights-widget")).toBeInTheDocument();
    });

    expect(screen.getByText("Draft email")).toBeInTheDocument();
    expect(screen.getByText("Draft quote for Carlos")).toBeInTheDocument();
    expect(screen.getByText("For: Maya")).toBeInTheDocument();

    expect(screen.getByText("Send SMS")).toBeInTheDocument();
    expect(screen.getByText("Follow up on abandoned cart for Priya")).toBeInTheDocument();
    expect(screen.getByText("For: Priya")).toBeInTheDocument();

    // Test approve action
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ success: true }),
    });

    const approveButton = screen.getByTestId("approve-insight-item-1");
    await userEvent.click(approveButton);

    expect(global.fetch).toHaveBeenCalledWith("/api/v1/ui/dashboard/daily-work/action/item-1", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ action_status: "APPROVED" }),
    });

    // Optimistic UI update
    await waitFor(() => {
      expect(screen.queryByTestId("insight-item-item-1")).not.toBeInTheDocument();
    });
    expect(screen.getByTestId("insight-item-item-2")).toBeInTheDocument();
  });
});
