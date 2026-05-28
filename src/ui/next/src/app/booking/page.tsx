'use client';
import { useState, useEffect, useRef } from 'react';
import Link from 'next/link';
import { WithTooltip } from '../../components/TooltipRegistry';

type Message = {
  id: number;
  sender: 'ai' | 'user';
  text: string;
  type?: 'text' | 'slots' | 'confirmation';
  slots?: string[];
};

export default function BookingFlow() {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: 1,
      sender: 'ai',
      text: 'Hi there! 👋 I am the AI assistant for Carlos Handyworks. How can we help you today?',
    }
  ]);
  const [inputValue, setInputValue] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    if (typeof messagesEndRef.current?.scrollIntoView === 'function') messagesEndRef.current.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isTyping]);

  const handleSend = () => {
    if (!inputValue.trim()) return;

    const userMessage: Message = {
      id: Date.now(),
      sender: 'user',
      text: inputValue.trim()
    };

    setMessages(prev => [...prev, userMessage]);
    setInputValue('');
    setIsTyping(true);

    // Simulate AI response logic
    setTimeout(() => {
      setIsTyping(false);
      if (messages.length === 1) {
        setMessages(prev => [...prev, {
          id: Date.now() + 1,
          sender: 'ai',
          text: 'Thanks for letting me know. I can definitely help schedule a time for Carlos to come take a look at that. Here are a few available times based on his calendar:',
          type: 'slots',
          slots: ['Tomorrow at 10:00 AM', 'Tomorrow at 2:00 PM', 'Friday at 9:00 AM']
        }]);
      } else {
        setMessages(prev => [...prev, {
          id: Date.now() + 1,
          sender: 'ai',
          text: 'I can help with that. Are you looking to schedule an appointment?',
          type: 'slots',
          slots: ['Tomorrow at 10:00 AM', 'Tomorrow at 2:00 PM', 'Friday at 9:00 AM']
        }]);
      }
    }, 1500);
  };

  const handleSlotSelect = (slot: string) => {
    setMessages(prev => [...prev, {
      id: Date.now(),
      sender: 'user',
      text: slot
    }]);
    setIsTyping(true);

    setTimeout(() => {
      setIsTyping(false);
      setMessages(prev => [...prev, {
        id: Date.now() + 1,
        sender: 'ai',
        text: `Great! I've provisionally booked Carlos for ${slot}. To confirm the appointment, please click below to submit the $50 deposit.`,
        type: 'confirmation'
      }]);
    }, 1500);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSend();
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] font-inter flex justify-center items-center p-4 sm:p-6 relative overflow-hidden">
      {/* Decorative background blurs mimicking Apple style */}
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-blue-400 rounded-[8px] blur-[120px] opacity-20 pointer-events-none"></div>
      <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-purple-400 rounded-[8px] blur-[120px] opacity-20 pointer-events-none"></div>

      <div className="w-full max-w-[375px] h-[812px] max-h-[90vh] bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-2xl rounded-[16px] overflow-hidden flex flex-col relative z-10">

        {/* Header */}
        <div className="px-6 py-5 border-b border-gray-200/50 flex items-center justify-between bg-white/40 shrink-0">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-[8px] bg-gradient-to-tr from-blue-500 to-indigo-500 p-0.5 shadow-sm">
              <div className="w-full h-full bg-white rounded-[8px] flex items-center justify-center text-lg shadow-inner">
                🤖
              </div>
            </div>
            <div>
              <h1 className="text-[17px] font-semibold text-[#1D1D1F] font-outfit leading-tight">Carlos' Assistant</h1>
              <div className="flex items-center gap-1.5 mt-0.5">
                <span className="w-1.5 h-1.5 rounded-[8px] bg-[#34C759]"></span>
                <span className="text-[12px] text-gray-500 font-medium">Online</span>
              </div>
            </div>
          </div>
          <Link href="/dashboard" className="w-8 h-8 rounded-[8px] bg-gray-100 flex items-center justify-center text-gray-500 hover:bg-gray-200 transition-colors">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
          </Link>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-5 flex flex-col gap-5 scroll-smooth">
          {messages.map((msg) => (
            <div key={msg.id} className={`flex flex-col ${msg.sender === 'user' ? 'items-end' : 'items-start'}`}>
              <div className={`
                max-w-[85%] px-4 py-3 text-[15px] leading-[1.4] shadow-sm
                ${msg.sender === 'user'
                  ? 'bg-[#0071E3] text-white rounded-[8px] rounded-br-[2px]'
                  : 'bg-white border border-gray-100 text-[#1D1D1F] rounded-[8px] rounded-bl-[2px]'
                }
              `}>
                {msg.text}
              </div>

              {/* Slots Options */}
              {msg.type === 'slots' && msg.slots && (
                <div className="mt-3 flex flex-col gap-2 w-full max-w-[85%]">
                  {msg.slots.map((slot, idx) => (
                    <button
                      key={idx}
                      onClick={() => handleSlotSelect(slot)}
                      className="px-4 py-2.5 text-sm font-semibold text-[#0071E3] bg-blue-50 hover:bg-blue-100 border border-blue-100 rounded-[8px] transition-colors text-left shadow-sm"
                    >
                      {slot}
                    </button>
                  ))}
                </div>
              )}

              {/* Confirmation / Payment */}
              {msg.type === 'confirmation' && (
                <div className="mt-3 w-full max-w-[85%] bg-white border border-gray-200 rounded-[16px] p-4 shadow-sm">
                  <div className="flex items-center gap-3 mb-3">
                    <div className="w-10 h-10 rounded-[8px] bg-green-100 flex items-center justify-center text-green-600">
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                    </div>
                    <div>
                      <div className="text-[15px] font-semibold text-[#1D1D1F]">Appointment Pending</div>
                      <div className="text-[13px] text-gray-500">Awaiting deposit</div>
                    </div>
                  </div>
                  <WithTooltip id="payment-tooltip" defaultText="Securely process the deposit to confirm the booking.">
                    <button id="pay-deposit-btn" className="w-full py-2.5 bg-[#1D1D1F] text-white text-[15px] font-semibold rounded-[8px] hover:bg-black transition-colors shadow-md">
                      Pay $50 Deposit
                    </button>
                  </WithTooltip>
                </div>
              )}
            </div>
          ))}

          {isTyping && (
            <div className="flex items-start">
              <div className="bg-white border border-gray-100 px-4 py-3 rounded-[8px] rounded-bl-[2px] shadow-sm flex items-center gap-1.5 h-[44px]">
                <span className="w-1.5 h-1.5 bg-gray-400 rounded-[8px] animate-bounce" style={{ animationDelay: '0ms' }}></span>
                <span className="w-1.5 h-1.5 bg-gray-400 rounded-[8px] animate-bounce" style={{ animationDelay: '150ms' }}></span>
                <span className="w-1.5 h-1.5 bg-gray-400 rounded-[8px] animate-bounce" style={{ animationDelay: '300ms' }}></span>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* Input Area */}
        <div className="p-4 bg-white/60 border-t border-gray-200/50 shrink-0">
          <div className="relative flex items-center">
            <input
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder="Type your message..."
              className="w-full pl-4 pr-12 py-3 bg-white border border-gray-200 rounded-[8px] text-[15px] text-[#1D1D1F] placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-[#0071E3]/20 focus:border-[#0071E3] transition-all shadow-sm"
            />
            <button
              onClick={handleSend}
              disabled={!inputValue.trim()}
              className="absolute right-1.5 w-[36px] h-[36px] bg-[#0071E3] text-white rounded-[8px] flex items-center justify-center disabled:opacity-50 disabled:bg-gray-300 transition-colors shadow-sm"
            >
              <svg className="w-4 h-4 ml-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
            </button>
          </div>
          <div className="text-center mt-2">
            <span className="text-[10px] text-gray-400 font-medium tracking-wide">POWERED BY OHC AI</span>
          </div>
        </div>

      </div>
    </div>
  );
}
