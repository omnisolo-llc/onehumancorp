"use client";

import React, { useState } from 'react';

export default function HelpChatFloatingButton() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="fixed bottom-4 right-4 z-50">
      {isOpen && (
        <div className="bg-white p-4 rounded-lg shadow-lg border w-80 h-96 flex flex-col mb-4">
          <div className="font-bold border-b pb-2 mb-2">Help Agent</div>
          <div className="flex-1 overflow-y-auto">
            <p className="text-sm">Hi! I am your AI Support Agent. How can I help you today?</p>
          </div>
          <input type="text" placeholder="Ask anything..." className="border rounded px-2 py-1 w-full" />
        </div>
      )}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="bg-blue-600 text-white rounded-full w-12 h-12 flex items-center justify-center shadow-lg"
      >
        ?
      </button>
    </div>
  );
}
