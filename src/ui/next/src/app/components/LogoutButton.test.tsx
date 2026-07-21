import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { LogoutButton } from "./LogoutButton";

const replace = vi.fn();
const refresh = vi.fn();
vi.mock("next/navigation", () => ({
  useRouter: () => ({ replace, refresh }),
}));

global.fetch = vi.fn();

describe("LogoutButton", () => {
  beforeEach(() => {
    replace.mockReset();
    refresh.mockReset();
    if (vi.isMockFunction(fetch)) vi.mocked(fetch).mockReset();
  });

  it("posts logout once and returns to login", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({ ok: true }));
    render(<LogoutButton />);
    await userEvent.click(screen.getByRole("button", { name: /log out/i }));
    await waitFor(() => expect(replace).toHaveBeenCalledWith("/login"));
    expect(fetch).toHaveBeenCalledWith("/api/v1/auth/logout", { method: "POST" });
    expect(refresh).toHaveBeenCalled();
  });

  it("announces failure and remains usable when the endpoint cannot clear the cookie", async () => {
    vi.mocked(fetch).mockRejectedValueOnce(new Error("offline"));
    render(<LogoutButton />);
    await userEvent.click(screen.getByRole("button", { name: /log out/i }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Logout failed. Please try again.");
    expect(screen.getByRole("button", { name: /log out/i })).toBeEnabled();
    expect(replace).not.toHaveBeenCalled();
  });
});
