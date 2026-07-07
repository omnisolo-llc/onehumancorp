import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi, beforeEach } from "vitest";
import { CFOAgentCard } from "./CFOAgentCard";

describe("CFOAgentCard", () => {
  beforeEach(() => {
    global.fetch = vi.fn();
  });

  it("renders safe to spend data", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        money_in: 500,
        money_out: 100,
        tax_safe: 50,
      }),
    });

    render(<CFOAgentCard />);

    await waitFor(() => {
      expect(screen.getByText("Profit & Tax Card")).toBeInTheDocument();
    });

    expect(screen.getByText("$500.00")).toBeInTheDocument();
    expect(screen.getByText("$100.00")).toBeInTheDocument();
    expect(screen.getByText("$50.00")).toBeInTheDocument();
  });
});
