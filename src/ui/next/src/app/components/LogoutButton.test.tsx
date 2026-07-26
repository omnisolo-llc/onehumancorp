import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { LogoutButton } from "./LogoutButton";

const replace = vi.fn();
const refresh = vi.fn();
vi.mock("next/navigation", () => ({
  useRouter: () => ({ replace, refresh }),
}));

describe("LogoutButton", () => {
  beforeEach(() => {
    replace.mockReset();
    refresh.mockReset();
    global.fetch = vi.fn();
  });

  it("posts logout once and returns to login", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json({ ok: true }) as any);
    render(<LogoutButton />);
    await userEvent.click(screen.getByRole("button", { name: /log out/i }));
    await waitFor(() => expect(replace).toHaveBeenCalledWith("/login"));
    expect(global.fetch).toHaveBeenCalledWith("/api/v1/auth/logout", { method: "POST" });
    expect(refresh).toHaveBeenCalled();
  });

  it("announces failure and remains usable when the endpoint cannot clear the cookie", async () => {
    vi.mocked(global.fetch).mockRejectedValueOnce(new Error("offline"));
    render(<LogoutButton />);
    await userEvent.click(screen.getByRole("button", { name: /log out/i }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Logout failed. Please try again.");
    expect(screen.getByRole("button", { name: /log out/i })).toBeEnabled();
    expect(replace).not.toHaveBeenCalled();
  });
});
