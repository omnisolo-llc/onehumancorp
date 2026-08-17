import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import InteractiveQuotePage from "./page";

vi.mock("next/navigation", () => ({
  useParams: () => ({ id: "quote-7" }),
}));

const quoteResponse = {
  quote: {
    id: "quote-7",
    status: "SENT",
    total_amount_cents: 12000,
    required_deposit_cents: 4000,
  },
  line_items: [
    { id: "line-1", description: "Site visit", unit_price_cents: 12000, quantity: 1 },
  ],
};

describe("interactive quote", () => {
  beforeEach(() => {
    vi.mocked(fetch).mockReset();
  });

  it("loads and accepts the real quote through versioned endpoints", async () => {
    vi.mocked(fetch)
      .mockResolvedValueOnce(Response.json(quoteResponse))
      .mockResolvedValueOnce(Response.json({ status: "ACCEPTED" }));

    render(<InteractiveQuotePage />);
    const user = userEvent.setup();

    expect(await screen.findByText(/Site visit/)).toBeVisible();
    expect(fetch).toHaveBeenNthCalledWith(1, "/api/v1/quotes/quote-7", expect.objectContaining({
      cache: "no-store",
    }));
    await user.click(screen.getByRole("button", { name: "Accept quote" }));

    await waitFor(() => expect(fetch).toHaveBeenLastCalledWith(
      "/api/v1/quotes/quote-7/accept",
      expect.objectContaining({ method: "POST" }),
    ));
    expect(await screen.findByRole("status")).toHaveTextContent("Quote accepted");
  });

  it("does not fabricate quote data when the service is unavailable", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({ error: "unavailable" }, { status: 503 }));
    render(<InteractiveQuotePage />);

    expect(await screen.findByRole("alert")).toHaveTextContent("This quote is unavailable.");
    expect(screen.queryByText("Site visit")).toBeNull();
  });
});
