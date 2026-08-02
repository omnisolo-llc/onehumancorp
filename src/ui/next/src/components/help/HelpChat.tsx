import React from 'react';
import DOMPurify from 'dompurify';

type ChatMessage = { id: string; role: "bot" | "user"; text: string; linkUrl?: string; linkTitle?: string };

interface HelpChatProps {
  chatMessages: ChatMessage[];
  chatInput: string;
  setChatInput: (val: string) => void;
  handleChatSubmit: (e: React.FormEvent) => void;
  clearChat: () => void;
}

export function HelpChat({ chatMessages, chatInput, setChatInput, handleChatSubmit, clearChat }: HelpChatProps) {
  return (
    <div className="flex flex-col h-full backdrop-blur-[30px] saturate-[210%] bg-[rgba(255,255,255,0.65)] border-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.7)] dark:border-[rgba(255,255,255,0.1)] border rounded-xl p-2 shadow-sm">
      <div className="flex justify-end p-2 border-b border-white/30 dark:border-white/10">
        {chatMessages.length > 1 && (
          <button
            onClick={clearChat}
            className="text-xs font-bold text-gray-500 hover:text-gray-800 transition-colors bg-white/40 hover:bg-white/60 px-3 py-1.5 rounded-full"
            aria-label="Clear chat"
          >
            Clear
          </button>
        )}
      </div>
      <div className="flex-1 space-y-4 overflow-y-auto pr-2 pb-2 mt-2">
        {chatMessages.map((msg) => {
          const className = `p-3 rounded-2xl text-sm w-4/5 ${
            msg.role === "bot"
              ? "backdrop-blur-[30px] saturate-[210%] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] shadow-sm text-blue-900 rounded-tl-none"
              : "backdrop-blur-[30px] saturate-[210%] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] shadow-sm text-gray-800 rounded-tr-none ml-auto"
          }`;
          return msg.role === "bot" ? (
            <div key={msg.id} className={className}>
              <div dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(msg.text) }} />
              {msg.linkUrl && (
                <div className="mt-2 pt-2 border-t border-blue-100">
                  <a href={msg.linkUrl} className="text-blue-600 font-medium hover:underline text-xs">Read the full article →</a>
                </div>
              )}
            </div>
          ) : (
            <div key={msg.id} className={className}>{msg.text}</div>
          );
        })}
      </div>
      <form onSubmit={handleChatSubmit} className="mt-4 flex gap-2 pt-3 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
        <input
          type="text"
          placeholder="Ask anything..."
          value={chatInput}
          onChange={(e) => setChatInput(e.target.value)}
          className="flex-1 p-3 border border-[rgba(255,255,255,0.4)] rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] shadow-[0_4px_24px_rgba(0,0,0,0.04)] min-h-[44px]"
        />
        <button type="submit" disabled={!chatInput.trim()} className="bg-blue-600/90 backdrop-blur-[30px] saturate-[210%] text-white p-3 rounded-xl hover:bg-blue-700/90 shadow-sm active:scale-95 transition-all disabled:opacity-50 disabled:cursor-not-allowed min-w-[44px] min-h-[44px] flex items-center justify-center" aria-label="Send message">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
        </button>
      </form>
    </div>
  );
}
