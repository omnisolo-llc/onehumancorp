import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { LogoutButton } from "./LogoutButton";
import { hasVerifiedOfflineQueueOwner, invalidateQueueOwner, readQueueOwner } from "../../lib/sync/queueIdentity";

const { navigateToPublicAuth } = vi.hoisted(() => ({ navigateToPublicAuth: vi.fn() }));
vi.mock("@/lib/auth/publicNavigation", () => ({ navigateToPublicAuth }));

describe("LogoutButton", () => {
  beforeEach(() => {
    navigateToPublicAuth.mockReset();
    vi.mocked(fetch).mockReset();
  });

  it("posts logout once and replaces the document only after acknowledgement", async () => {
    let acknowledge!: (response: Response) => void;
    vi.mocked(fetch).mockReturnValueOnce(new Promise(resolve => { acknowledge = resolve; }));
    render(<LogoutButton />);
    await userEvent.click(screen.getByRole("button", { name: /log out/i }));
    expect(screen.getByRole("button", { name: "Logging out…" })).toBeDisabled();
    expect(navigateToPublicAuth).not.toHaveBeenCalled();
    await userEvent.click(screen.getByRole("button", { name: "Logging out…" }));
    expect(fetch).toHaveBeenCalledTimes(1);
    expect(fetch).toHaveBeenCalledWith("/api/v1/auth/logout", { method: "POST" });
    acknowledge(Response.json({ ok: true }));
    await waitFor(() => expect(navigateToPublicAuth).toHaveBeenCalledWith("/login", true));
    expect(navigateToPublicAuth).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("button", { name: "Logging out…" })).toBeDisabled();
  });

  it("announces failure and remains usable when the endpoint cannot clear the cookie", async () => {
    vi.mocked(fetch).mockRejectedValueOnce(new Error("offline"));
    render(<LogoutButton />);
    await userEvent.click(screen.getByRole("button", { name: /log out/i }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Logout failed. Please try again.");
    expect(screen.getByRole("button", { name: /log out/i })).toBeEnabled();
    expect(navigateToPublicAuth).not.toHaveBeenCalled();
  });

  it("does not navigate when logout returns an unsuccessful response", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({ error: "unavailable" }, { status: 503 }));
    render(<LogoutButton />);
    await userEvent.click(screen.getByRole("button", { name: /log out/i }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Logout failed. Please try again.");
    expect(screen.getByRole("button", { name: /log out/i })).toBeEnabled();
    expect(navigateToPublicAuth).not.toHaveBeenCalled();
  });

  it('invalidates the same-tab offline owner before document logout navigation', async () => {
    invalidateQueueOwner();
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({ userId: 'old', tenantId: 't', expiresAt: Date.now() + 60000 }));
    await readQueueOwner();
    let verifiedAtNavigation: boolean | undefined;
    navigateToPublicAuth.mockImplementationOnce(() => {
      verifiedAtNavigation = hasVerifiedOfflineQueueOwner();
    });
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({ ok: true }));
    render(<LogoutButton />);
    await userEvent.click(screen.getByRole('button', { name: /log out/i }));
    await waitFor(() => expect(navigateToPublicAuth).toHaveBeenCalledWith('/login', true));
    expect(verifiedAtNavigation).toBe(false);
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    await expect(readQueueOwner()).rejects.toThrow('identity');
    vi.restoreAllMocks();
  });

});
