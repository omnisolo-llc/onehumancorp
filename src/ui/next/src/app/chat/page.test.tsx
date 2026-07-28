import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi, beforeEach } from "vitest";
import NativeChatInboxPage from "./page";
import '@testing-library/jest-dom';

// Mock AppShell to avoid importing complex layout stuff
vi.mock("../components/AppShell", () => ({
  AppShell: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
}));

describe("NativeChatInboxPage", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("renders conversations list and allows selecting a conversation", async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === "/api/v1/chat-inbox/conversations") {
        return Promise.resolve({
          ok: true,
          json: () =>
            Promise.resolve([
              { id: "conv1", status: "open", created_at: "2023-10-01", updated_at: "2023-10-01", contact_name: "John Doe" },
            ]),
        });
      }
      if (url === "/api/v1/chat-inbox/conversations/conv1/messages") {
        return Promise.resolve({
          ok: true,
          json: () =>
            Promise.resolve([
              { id: "msg1", sender_type: "contact", content: "Hello", created_at: "2023-10-01" },
            ]),
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
    });

    render(<NativeChatInboxPage />);

    // Wait for the conversation to appear in the sidebar
    await waitFor(() => {
      expect(screen.getByText("John Doe")).toBeInTheDocument();
    });

    // Select the conversation
    const convBtn = screen.getByTestId("conversation-conv1");
    await userEvent.click(convBtn);

    // Wait for the message to load
    await waitFor(() => {
      expect(screen.getByText("Hello")).toBeInTheDocument();
    });
  });

  it("can send a new message", async () => {
    let messageSent = false;
    global.fetch = vi.fn().mockImplementation((url, options) => {
      if (url === "/api/v1/chat-inbox/conversations") {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([{ id: "conv2", status: "open", created_at: "2023-10-01", updated_at: "2023-10-01" }]),
        });
      }
      if (url === "/api/v1/chat-inbox/conversations/conv2/messages" && (!options || options.method === "GET")) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([]),
        });
      }
      if (url === "/api/v1/chat-inbox/conversations/conv2/messages" && options?.method === "POST") {
        messageSent = true;
        const body = JSON.parse(options.body as string);
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ id: "msg2", sender_type: "agent", content: body.content, created_at: "2023-10-02" }),
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
    });

    render(<NativeChatInboxPage />);
    await waitFor(() => {
      expect(screen.getByTestId("conversation-conv2")).toBeInTheDocument();
    });

    await userEvent.click(screen.getByTestId("conversation-conv2"));

    // Wait for empty state or chat input
    await waitFor(() => {
      expect(screen.getByTestId("chat-input")).toBeInTheDocument();
    });

    // Type a message
    const input = screen.getByTestId("chat-input");
    await userEvent.type(input, "Here is a reply");

    // Click send
    const sendBtn = screen.getByTestId("chat-send");
    await userEvent.click(sendBtn);

    // Verify it sent
    await waitFor(() => {
      expect(messageSent).toBe(true);
      expect(screen.getByText("Here is a reply")).toBeInTheDocument();
    });
  });
});
