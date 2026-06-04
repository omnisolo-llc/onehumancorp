import { render, screen, waitFor } from "@testing-library/react";
import UnifiedAgentFeedPage from "./page";
import { describe, it, expect, vi, beforeEach } from "vitest";
import React from "react";

// Mock AppShell to avoid complex routing dependencies in this simple test
vi.mock("../components/AppShell", () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell">{children}</div>,
}));

describe("UnifiedAgentFeedPage", () => {
  beforeEach(() => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve({
          proposals: [
            {
              id: "prop_1",
              agent_type: "Marketing Agent",
              type: "Proposal",
              status: "New",
              title: "Launch Summer Promo Campaign",
              description: "Based on last year's trends...",
              actions: [
                { label: "Approve & Launch", style: "primary" },
                { label: "Review Drafts", style: "secondary" }
              ],
              icon: "💡",
              color: "blue"
            },
            {
              id: "prop_2",
              agent_type: "Operations Agent",
              type: "Alert",
              status: "Action Needed",
              title: "Low Stock: Premium Fertilizer",
              description: "Inventory for Premium Fertilizer has dropped...",
              actions: [
                { label: "Approve Order ($450)", style: "primary" },
                { label: "Ignore for now", style: "secondary" }
              ],
              icon: "🛡️",
              color: "green"
            }
          ]
        })
      })
    ) as any;
  });

  it("renders the Unified Agent Feed header", async () => {
    render(<UnifiedAgentFeedPage />);
    expect(screen.getByText("Unified Agent Feed")).toBeInTheDocument();
  });

  it("renders agent proposals after loading", async () => {
    render(<UnifiedAgentFeedPage />);
    expect(screen.getByTestId("loading-state")).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText("Launch Summer Promo Campaign")).toBeInTheDocument();
      expect(screen.getByText("Low Stock: Premium Fertilizer")).toBeInTheDocument();
    });
  });

  it("renders action buttons", async () => {
    render(<UnifiedAgentFeedPage />);
    await waitFor(() => {
      expect(screen.getByText("Approve & Launch")).toBeInTheDocument();
      expect(screen.getByText("Review Drafts")).toBeInTheDocument();
      expect(screen.getByText("Approve Order ($450)")).toBeInTheDocument();
    });
  });

  it("handles empty state", async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve({ proposals: [] })
      })
    ) as any;

    render(<UnifiedAgentFeedPage />);
    await waitFor(() => {
      expect(screen.getByTestId("empty-state")).toBeInTheDocument();
    });
  });

  it("handles fetch error gracefully", async () => {
    global.fetch = vi.fn(() => Promise.reject("API Error")) as any;

    render(<UnifiedAgentFeedPage />);
    await waitFor(() => {
      expect(screen.getByTestId("empty-state")).toBeInTheDocument();
    });
  });
});
