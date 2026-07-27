"use client";

import React, { useState, useEffect } from "react";
import TriageFeed from "./TriageFeed";
import ConversationView from "./ConversationView";

export default function OmnichannelChatPage() {
  const [activeConversationId, setActiveConversationId] = useState<string | null>(null);
  const [tenantId, setTenantId] = useState<string>("");

  useEffect(() => {
    // Fetch the actual user profile to get the tenant_id
    fetch('/api/v1/auth/me')
      .then(res => res.json())
      .then(data => {
        if (data && data.organization_id) {
          setTenantId(data.organization_id);
        }
      })
      .catch(console.error);
  }, []);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-[#F8F9FA] font-inter py-10">
      <div className="w-[375px] min-h-[812px] glassmorphism shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
        {!activeConversationId ? (
          <TriageFeed
            tenantId={tenantId}
            onSelectConversation={setActiveConversationId}
          />
        ) : (
          <ConversationView
            conversationId={activeConversationId}
            tenantId={tenantId}
            onBack={() => setActiveConversationId(null)}
          />
        )}
      </div>
    </div>
  );
}
