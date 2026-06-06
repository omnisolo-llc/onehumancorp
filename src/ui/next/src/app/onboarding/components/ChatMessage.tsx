import React from 'react';
import { AgentAvatar } from './AgentAvatar';

export interface Message {
  id: string;
  role: 'agent' | 'user';
  text: string;
}

interface ChatMessageProps {
  message: Message;
}

export function ChatMessage({ message }: ChatMessageProps) {
  const isUser = message.role === 'user';

  return (
    <div className={`flex ${isUser ? 'justify-end' : 'justify-start'} mb-4 animate-fade-in`}>
      {!isUser && <AgentAvatar />}
      <div className={`max-w-[80%] p-3 rounded-[16px] text-sm ${isUser ? 'bg-[#0066FF] text-white rounded-tr-none shadow-md' : 'mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] rounded-tl-none border border-white/20'}`}>
        {message.text}
      </div>
    </div>
  );
}
