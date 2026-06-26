import React, { useRef, useEffect } from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './IconLabel';

interface StepChatProps {
  chatMessages: {role: string, content: string, image_url?: string}[];
  setChatMessages: React.Dispatch<React.SetStateAction<{role: string, content: string, image_url?: string}[]>>;
  chatInput: string;
  setChatInput: React.Dispatch<React.SetStateAction<string>>;
  chatImageUrl: string;
  setChatImageUrl: React.Dispatch<React.SetStateAction<string>>;
  handleSendChatMessage: () => void;
  syncStateToBackend: (state: any) => void;
}

export function StepChat({
  chatMessages,
  setChatMessages,
  chatInput,
  setChatInput,
  chatImageUrl,
  setChatImageUrl,
  handleSendChatMessage,
  syncStateToBackend
}: StepChatProps) {
  const { updateState, isLoading } = useOnboardingStore();
  const chatMessagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (chatMessagesEndRef.current && typeof chatMessagesEndRef.current.scrollIntoView === 'function') {
      chatMessagesEndRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [chatMessages]);

  return (
    <div className="flex flex-col flex-1 animate-fade-in w-full h-full max-h-full">
      <button onClick={() => { updateState({ step: -2 }); syncStateToBackend({ step: -2 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
      </button>
      <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center">Setup Assistant</h2>
      <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-4 leading-relaxed max-w-sm mx-auto">
        Talk to our AI to build your business.
      </p>

      <div className="flex flex-col flex-1 gap-4 overflow-hidden w-full max-w-full">
        <div id="chat-messages" className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] flex-1 overflow-y-auto p-4 text-[#1D1D1F] dark:text-[#F5F5F7] text-left space-y-4">
          {chatMessages.length === 0 && (
            <div className="mb-2"><strong>Assistant:</strong> What do you do? (e.g. I bake custom vegan cakes in Austin)</div>
          )}
          {chatMessages.map((msg, index) => (
            <div key={index} className={`mb-2 ${msg.role === 'user' ? 'text-[#0066FF]' : 'text-[#333] dark:text-[#A1A1A6]'}`}>
              <strong>{msg.role === 'user' ? 'You' : 'Assistant'}:</strong> {msg.content}
              {msg.image_url && <><br /><span className="text-xs text-gray-500 dark:text-gray-400">[Attached Image: {msg.image_url}]</span></>}
            </div>
          ))}
          {isLoading && (
            <div className="mb-2 text-[#333] dark:text-[#A1A1A6]">
               <span className="flex items-center gap-2">
                <svg className="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <strong>Assistant:</strong> Thinking...
              </span>
            </div>
          )}
          <div ref={chatMessagesEndRef} />
        </div>

        <div className="flex flex-col gap-2 shrink-0">
          <input
            type="url"
            id="chat-image-url"
            value={chatImageUrl}
            onChange={(e) => setChatImageUrl(e.target.value)}
            className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms] border border-white/40 dark:border-white/10 focus:border-[#0066FF] min-h-[44px]"
            placeholder="Image URL (Optional)"
            inputMode="url"
            autoComplete="url"
            enterKeyHint="next"
          />
          <div className="flex gap-2 w-full">
            <button
              id="chat-upload-btn"
              className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] min-w-[44px] min-h-[44px] flex items-center justify-center text-[#1D1D1F] dark:text-[#F5F5F7] hover:border-gray-400 dark:hover:border-gray-500 transition-all duration-[250ms] active:scale-[0.98]"
              onClick={() => {
                const url = prompt("Enter image URL");
                if (url) setChatImageUrl(url);
              }}
              title="Upload Image"
              aria-label="Upload Image"
            >
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
                <circle cx="8.5" cy="8.5" r="1.5"></circle>
                <polyline points="21 15 16 10 5 21"></polyline>
              </svg>
            </button>
            <input
              type="text"
              id="chat-input"
              value={chatInput}
              onChange={(e) => setChatInput(e.target.value)}
              onKeyDown={(e) => {
                 if (e.key === 'Enter') handleSendChatMessage();
              }}
              className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none flex-1 transition-all duration-[250ms] border border-white/40 dark:border-white/10 focus:border-[#0066FF] min-h-[44px]"
              placeholder="Type a message..."
              enterKeyHint="send"
            />
            <button
              id="chat-send-btn"
              onClick={handleSendChatMessage}
              disabled={isLoading}
              className="bg-[#0066FF] text-white font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] px-4 shrink-0 disabled:opacity-50 rounded-[8px]"
            >
              Send
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
