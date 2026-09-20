import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import ProviderConnections from "./ProviderConnections";
const json = (value: unknown, status = 200) => Promise.resolve({ ok: status < 400, json: async () => value });

describe("verified provider connections", () => {
  beforeEach(() => { vi.stubGlobal("fetch", vi.fn()); });
  afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });

  it("does not turn an unverified status into a connected provider", async () => {
    vi.mocked(fetch).mockImplementation(() => json({ success: true, integrations: [{ id: "openai_api", status: "pending", usable: true }] }) as ReturnType<typeof fetch>);
    render(<ProviderConnections />);
    await waitFor(() => expect(fetch).toHaveBeenCalled());
    expect(within(screen.getByRole("article", { name: "OpenAI API" })).getByText("Not connected")).toBeVisible();
    expect(screen.queryByText("Verified connection")).toBeNull();
  });

  it("sends only the entered key and requires verified usable confirmation", async () => {
    let verified = false;
    vi.mocked(fetch).mockImplementation((url, options) => {
      if (options?.method === "POST") { verified = true; return json({ success: true, status: "verified", usable: true }) as ReturnType<typeof fetch>; }
      return json({ success: true, integrations: verified ? [{ id: "openai_api", status: "verified", usable: true }] : [] }) as ReturnType<typeof fetch>;
    });
    const storage = vi.spyOn(Storage.prototype, "setItem");
    render(<ProviderConnections />);
    fireEvent.click(screen.getByRole("button", { name: "Configure OpenAI API" }));
    const input = screen.getByLabelText("OpenAI API API key");
    expect(input).toHaveAttribute("type", "password");
    expect(screen.getByRole("button", { name: "Verify and save key" })).toBeDisabled();
    fireEvent.change(input, { target: { value: "  sk-test-only-not-a-real-key  " } });
    fireEvent.click(screen.getByRole("button", { name: "Verify and save key" }));
    await waitFor(() => expect(fetch).toHaveBeenCalledWith("/api/v1/integrations/openai_api/connect", expect.objectContaining({
      method: "POST", body: JSON.stringify({ api_token: "sk-test-only-not-a-real-key" }),
    })));
    expect(await screen.findByText("Verified connection")).toBeVisible();
    expect(screen.queryByLabelText("OpenAI API API key")).toBeNull();
    expect(storage).not.toHaveBeenCalled();
  });

  it("clears the key on failed verification without reflecting an upstream secret", async () => {
    vi.mocked(fetch).mockImplementation((_url, options) => (options?.method === "POST"
      ? json({ success: true, status: "pending", usable: false, error: "sk-secret-echo" })
      : json({ success: true, integrations: [] })) as ReturnType<typeof fetch>);
    render(<ProviderConnections />);
    fireEvent.click(screen.getByRole("button", { name: "Configure Stripe payments" }));
    const input = screen.getByLabelText("Stripe payments API key");
    fireEvent.change(input, { target: { value: "sk-secret-echo" } });
    fireEvent.click(screen.getByRole("button", { name: "Verify and save key" }));
    expect(await screen.findByText(/The key could not be verified/)).toBeVisible();
    expect(input).toHaveValue("");
    expect(screen.queryByText("sk-secret-echo")).toBeNull();
    expect(screen.queryByText("Verified connection")).toBeNull();
  });

  it("requires an explicit disconnect confirmation and does not hide a revocation failure", async () => {
    vi.mocked(fetch).mockImplementation((_url, options) => (options?.method === "DELETE"
      ? json({ success: false }, 503)
      : json({ success: true, integrations: [{ id: "stripe", status: "verified", usable: true }] })) as ReturnType<typeof fetch>);
    render(<ProviderConnections />);
    fireEvent.click(await screen.findByRole("button", { name: "Disconnect Stripe payments" }));
    expect(fetch).not.toHaveBeenCalledWith("/api/v1/integrations/stripe", expect.objectContaining({ method: "DELETE" }));
    fireEvent.click(screen.getByRole("button", { name: "Confirm disconnect" }));
    expect(await screen.findByText(/could not be revoked/)).toBeVisible();
    expect(screen.getByText("Verified connection")).toBeVisible();
  });

  it("rechecks expired verification through the owner-authenticated endpoint", async () => {
    let refreshed = false;
    vi.mocked(fetch).mockImplementation((_url, options) => {
      if (options?.method === "POST") { refreshed = true; return json({ success: true, status: "verified", usable: true }) as ReturnType<typeof fetch>; }
      return json({ success: true, integrations: [{ id: "stripe", status: refreshed ? "verified" : "reauthentication_required", usable: refreshed }] }) as ReturnType<typeof fetch>;
    });
    render(<ProviderConnections />);
    fireEvent.click(await screen.findByRole("button", { name: "Recheck Stripe payments" }));
    await waitFor(() => expect(fetch).toHaveBeenCalledWith("/api/v1/integrations/stripe/verify", { method: "POST" }));
    expect(await screen.findByText("Verified connection")).toBeVisible();
  });
});
