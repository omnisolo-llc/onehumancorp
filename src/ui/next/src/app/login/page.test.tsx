import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import Login from "./page";

const replace = vi.fn();
vi.mock("next/navigation", () => ({
  useRouter: () => ({ replace }),
  useSearchParams: () => new URLSearchParams("next=%2Forders%3Ftab%3Dopen"),
}));

global.fetch = vi.fn() as any;

describe("login page", () => {
  beforeEach(() => {
    replace.mockReset();
    vi.mocked(global.fetch).mockReset();
  });

  it("submits controlled credentials and navigates only after success", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(
      Response.json({ user: { id: "user-7", username: "Alice", roles: ["ADMIN"], organizationId: "tenant-7" }, next: "/orders?tab=open" }),
    );
    const storage = vi.spyOn(localStorage, "setItem");
    render(<Login />);
    const user = userEvent.setup();
    await user.type(screen.getByLabelText(/email or username/i), "Alice");
    await user.type(screen.getByLabelText(/^password$/i), "correct horse");
    await user.type(screen.getByLabelText(/organization/i), "tenant-7");
    await user.click(screen.getByRole("button", { name: /log in/i }));

    await waitFor(() => expect(replace).toHaveBeenCalledWith("/orders?tab=open"));
    expect(global.fetch).toHaveBeenCalledWith("/api/v1/auth/login?next=%2Forders%3Ftab%3Dopen", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ username: "Alice", password: "correct horse", organization_id: "tenant-7" }),
    });
    expect(storage).not.toHaveBeenCalled();
  });

  it("supports keyboard submission and omits an empty organization", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json({ user: {}, next: "/dashboard" }));
    render(<Login />);
    const user = userEvent.setup();
    await user.type(screen.getByLabelText(/email or username/i), "alice@example.test");
    await user.type(screen.getByLabelText(/^password$/i), "correct horse{Enter}");
    await waitFor(() => expect(replace).toHaveBeenCalledWith("/dashboard"));
    expect(JSON.parse(String(vi.mocked(global.fetch).mock.calls[0][1]?.body))).toEqual({
      username: "alice@example.test",
      password: "correct horse",
    });
  });

  it("disables duplicate submissions while pending", async () => {
    let resolve: ((value: Response) => void) | undefined;
    vi.mocked(global.fetch).mockImplementationOnce(() => new Promise<Response>((done) => { resolve = done; }));
    render(<Login />);
    const user = userEvent.setup();
    await user.type(screen.getByLabelText(/email or username/i), "Alice");
    await user.type(screen.getByLabelText(/^password$/i), "correct horse");
    const submit = screen.getByRole("button", { name: /log in/i });
    await user.click(submit);
    expect(screen.getByRole("button", { name: /signing in/i })).toBeDisabled();
    await user.click(screen.getByRole("button", { name: /signing in/i }));
    expect(global.fetch).toHaveBeenCalledTimes(1);
    resolve?.(Response.json({ user: {}, next: "/dashboard" }));
    await waitFor(() => expect(replace).toHaveBeenCalled());
  });

  it("announces one generic contained error and keeps credentials out of it", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(
      Response.json({ error: "invalid credentials" }, { status: 401 }),
    );
    render(<Login />);
    const user = userEvent.setup();
    await user.type(screen.getByLabelText(/email or username/i), "secret-user");
    await user.type(screen.getByLabelText(/^password$/i), "secret-password");
    await user.click(screen.getByRole("button", { name: /log in/i }));
    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent("We couldn't sign you in. Check your details and try again.");
    expect(alert).not.toHaveTextContent("secret-user");
    expect(alert).not.toHaveTextContent("secret-password");
    expect(replace).not.toHaveBeenCalled();
  });
});
