import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { AssistantInsightsWidget } from "./AssistantInsightsWidget";
import { describe, it, expect, vi, beforeEach } from "vitest";

describe("AssistantInsightsWidget", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should show loading state initially", () => {
    global.fetch = vi.fn().mockImplementation(() => new Promise(() => {}));
    const { container } = render(<AssistantInsightsWidget />);
    expect(container.querySelector(".animate-pulse")).toBeInTheDocument();
  });

  it("should display the insight text on successful fetch", async () => {
    const mockInsight = "Your business is doing well.";
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ summary: mockInsight }),
    });

    render(<AssistantInsightsWidget />);

    await waitFor(() => {
      expect(screen.getByText(mockInsight)).toBeInTheDocument();
    });

    expect(screen.getByText("Approve & Send")).toBeInTheDocument();
  });

  it("should return null on fetch error", async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error("Failed"));

    const { container } = render(<AssistantInsightsWidget />);

    await waitFor(() => {
      expect(container.firstChild).toBeNull();
    });
  });

  it("should render nothing if summary is null", async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ summary: null }),
    });

    const { container } = render(<AssistantInsightsWidget />);

    await waitFor(() => {
      expect(container.firstChild).toBeNull();
    });
  });
});
