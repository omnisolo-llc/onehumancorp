import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import GlobalCommerceSettingsPage from "./page";

describe("global commerce settings", () => {
  beforeEach(() => {
    vi.mocked(fetch).mockReset();
  });

  it("loads and saves persisted tenant currencies through the versioned API", async () => {
    vi.mocked(fetch)
      .mockResolvedValueOnce(Response.json({
        tenant: { base_currency: "EUR", enabled_currencies: ["EUR", "USD"] },
      }))
      .mockResolvedValueOnce(Response.json({ success: true }));

    render(<GlobalCommerceSettingsPage />);
    const user = userEvent.setup();

    expect(await screen.findByRole("combobox", { name: "Base currency" })).toHaveValue("EUR");
    expect(screen.getByRole("checkbox", { name: "USD" })).toBeChecked();
    await user.selectOptions(screen.getByRole("combobox", { name: "Base currency" }), "GBP");
    await user.click(screen.getByRole("checkbox", { name: "CAD" }));
    await user.click(screen.getByRole("button", { name: "Save changes" }));

    await waitFor(() => expect(fetch).toHaveBeenLastCalledWith(
      "/api/v1/settings/global-commerce",
      expect.objectContaining({
        method: "PUT",
        body: JSON.stringify({
          base_currency: "GBP",
          enabled_currencies: ["EUR", "USD", "GBP", "CAD"],
        }),
      }),
    ));
    expect(await screen.findByRole("status")).toHaveTextContent("Currency settings saved.");
  });

  it("shows an honest retry state when persisted settings are unavailable", async () => {
    vi.mocked(fetch)
      .mockResolvedValueOnce(Response.json({ error: "unavailable" }, { status: 503 }))
      .mockResolvedValueOnce(Response.json({
        tenant: { base_currency: "USD", enabled_currencies: ["USD"] },
      }));

    render(<GlobalCommerceSettingsPage />);
    const user = userEvent.setup();

    expect(await screen.findByRole("alert")).toHaveTextContent("Currency settings are unavailable.");
    await user.click(screen.getByRole("button", { name: "Retry" }));
    expect(await screen.findByRole("combobox", { name: "Base currency" })).toHaveValue("USD");
  });
});
