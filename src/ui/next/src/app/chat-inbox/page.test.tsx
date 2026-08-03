import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import NativeChatInboxPage from "./page";

// Mock the AppShell to just render children
vi.mock("../components/AppShell", () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}));

describe("NativeChatInboxPage", () => {
  it("renders without crashing", async () => {
    // Setup a basic mock fetch that returns empty arrays
    global.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.includes("/inboxes")) {
        return { ok: true, json: async () => [] };
      }
      return { ok: true, json: async () => [] };
    });

    render(<NativeChatInboxPage />);

    expect(screen.getByText("Inboxes")).toBeInTheDocument();
    expect(screen.getByText("Conversations")).toBeInTheDocument();
    expect(screen.getByText("Messages")).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.queryByText("Loading inboxes...")).not.toBeInTheDocument();
    });
  });

  it("fetches and displays inboxes, conversations, and messages", async () => {
    global.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url === "/api/v1/chat_engine/inboxes") {
        return { ok: true, json: async () => [{ id: "inbox-1", name: "Main Inbox" }] };
      }
      if (url === "/api/v1/chat_engine/inboxes/inbox-1/conversations") {
        return { ok: true, json: async () => [{ id: "conv-1", status: "open" }] };
      }
      if (url === "/api/v1/chat_engine/conversations/conv-1/messages") {
        return { ok: true, json: async () => [{ id: "msg-1", content: "Hello customer", sender_type: "agent" }] };
      }
      return { ok: true, json: async () => [] };
    });

    render(<NativeChatInboxPage />);

    await waitFor(() => {
      expect(screen.getByText("Main Inbox")).toBeInTheDocument();
      expect(screen.getByText(/conv-1/)).toBeInTheDocument();
      expect(screen.getByText("Hello customer")).toBeInTheDocument();
    });
  });

  it("sends a new message", async () => {
    global.fetch = vi.fn().mockImplementation(async (url: string, options?: any) => {
      if (url === "/api/v1/chat_engine/inboxes") {
        return { ok: true, json: async () => [{ id: "inbox-1", name: "Main Inbox" }] };
      }
      if (url === "/api/v1/chat_engine/inboxes/inbox-1/conversations") {
        return { ok: true, json: async () => [{ id: "conv-1", status: "open" }] };
      }
      if (url === "/api/v1/chat_engine/conversations/conv-1/messages" && (!options || options.method === "GET")) {
        return { ok: true, json: async () => [] };
      }
      if (url === "/api/v1/chat_engine/conversations/conv-1/messages" && options?.method === "POST") {
        return { ok: true, json: async () => ({ id: "msg-new", content: JSON.parse(options.body).content, sender_type: "agent" }) };
      }
      return { ok: true, json: async () => [] };
    });

    render(<NativeChatInboxPage />);

    await waitFor(() => {
      expect(screen.getByText("Main Inbox")).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText("Type a message...");
    const button = screen.getByText("Send");

    await userEvent.type(input, "I am replying now");
    await userEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText("I am replying now")).toBeInTheDocument();
    });
  });
});
