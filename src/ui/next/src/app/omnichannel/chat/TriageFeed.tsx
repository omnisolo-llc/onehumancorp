"use client";

import React, { useState, useEffect } from "react";

export default function TriageFeed({ onSelectConversation, tenantId }: any) {
  const [conversations, setConversations] = useState<any[]>([]);
  const [inboxes, setInboxes] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchInboxesAndConversations();
  }, []);

  const fetchInboxesAndConversations = async () => {
    try {
      setLoading(true);
      // Fetch inboxes
      const inboxesRes = await fetch("/api/v1/omnichannel/inboxes");
      let inboxesData = [];
      if (inboxesRes.ok) {
        inboxesData = await inboxesRes.json();
      }

      if (inboxesData.length === 0) {
        // Create default inbox
        const createRes = await fetch("/api/v1/omnichannel/inboxes", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ name: "Primary Inbox" })
        });
        if (createRes.ok) {
          const newInbox = await createRes.json();
          inboxesData.push(newInbox);
        }
      }

      setInboxes(inboxesData);

      if (inboxesData.length > 0) {
        const inboxId = inboxesData[0].id;
        const convRes = await fetch(`/api/v1/omnichannel/inboxes/${inboxId}/conversations`);
        if (convRes.ok) {
          const convData = await convRes.json();
          setConversations(convData);
        }
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const createConversation = async () => {
    if (inboxes.length === 0) return;
    const inboxId = inboxes[0].id;
    // We would normally select a contact, but for this implementation we simulate one.
    // The API requires a contact_id, we will generate a dummy UUID
    // Use an actual UUID generated client-side or from a real backend call.
    // Since we lack a contact selector in this UI mockup, generate a valid UUID.
    const contactId = crypto.randomUUID();

    try {
      const res = await fetch(`/api/v1/omnichannel/inboxes/${inboxId}/conversations`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ contact_id: contactId })
      });
      if (res.ok) {
        const newConv = await res.json();
        setConversations([newConv, ...conversations]);
        onSelectConversation(newConv.id);
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="flex flex-col h-full bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1]">
      <div className="pt-12 pb-4 px-6 border-b border-white/40 flex justify-between items-center">
        <h1 className="text-xl font-bold font-outfit text-gray-900">Unified Inbox</h1>
        <button
          onClick={createConversation}
          className="text-xs bg-[#0066FF] hover:bg-blue-700 text-white font-bold py-1 px-3 rounded-full"
          data-testid="new-conversation-btn"
        >
          + New
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-4">
        {loading && <p className="text-sm text-gray-500">Loading...</p>}
        {!loading && conversations.length === 0 && (
          <div className="text-center mt-10">
            <p className="text-sm text-gray-500">No active conversations.</p>
          </div>
        )}

        {conversations.map((conv) => (
          <div
            key={conv.id}
            onClick={() => onSelectConversation(conv.id)}
            className="p-4 mb-2 bg-white/80 rounded-xl shadow-sm border border-gray-100 cursor-pointer hover:bg-white transition-colors"
            data-testid={`conv-item-${conv.id}`}
          >
            <div className="flex justify-between items-center mb-1">
              <span className="font-bold text-sm text-gray-900">Contact {conv.contact_id.substring(0,6)}</span>
              <span className="text-xs text-gray-400">
                {new Date(conv.updated_at).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
              </span>
            </div>
            <p className="text-xs text-gray-500 truncate">Status: {conv.status}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
