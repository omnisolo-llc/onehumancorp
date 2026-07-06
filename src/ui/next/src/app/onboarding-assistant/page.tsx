"use client";

import { useState, useRef, useEffect } from "react";
import { useRouter } from "next/navigation";
import { motion, AnimatePresence } from "framer-motion";
import { FiSend, FiImage, FiCheck, FiCpu, FiMessageSquare } from "react-icons/fi";
import { tenantId } from "@/lib/auth";

type Message = {
  id: string;
  role: "user" | "assistant";
  content: string;
  actionCards?: ActionCard[];
};

type ActionCard = {
  id: string;
  title: string;
  description: string;
  status: "pending" | "approved" | "rejected" | "processing";
  thumbnail?: string;
  type: "landing_page" | "payment" | "product" | "general";
};

export default function OnboardingAssistant() {
  const router = useRouter();
  const [messages, setMessages] = useState<Message[]>([
    {
      id: "welcome",
      role: "assistant",
      content: "Hi! I'm your OHC Onboarding Assistant. Let's get your business set up. What kind of business do you run, and what do you sell?",
    },
  ]);
  const [input, setInput] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const [imageUrl, setImageUrl] = useState("");

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() && !imageUrl) return;

    const userMsg: Message = {
      id: Date.now().toString(),
      role: "user",
      content: input,
    };

    if (imageUrl) {
        userMsg.content += `\n[Image: ${imageUrl}]`;
    }

    setMessages((prev) => [...prev, userMsg]);
    setInput("");
    setImageUrl("");
    setIsProcessing(true);

    try {
      const res = await fetch("/api/v1/onboarding/start_zero_click", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          prompt: userMsg.content,
          image_url: imageUrl || undefined,
        }),
      });

      if (!res.ok) {
        throw new Error("Failed to process zero-click onboarding");
      }

      const data = await res.json();

      const assistantMsg: Message = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: "I've drafted some initial configurations for you based on what you told me. Review the action cards below:",
        actionCards: [
          {
            id: `landing-${Date.now()}`,
            title: "Publish your landing page",
            description: "A preliminary storefront design tailored to your business.",
            status: "pending",
            type: "landing_page",
          },
          {
            id: `payment-${Date.now()}`,
            title: "Set up payments",
            description: "Connect Stripe or configure online payments to start accepting orders immediately.",
            status: "pending",
            type: "payment",
          },
          {
            id: `product-${Date.now()}`,
            title: "Add your first product",
            description: "I've drafted your first core offering based on your description.",
            status: "pending",
            type: "product",
          }
        ],
      };

      setMessages((prev) => [...prev, assistantMsg]);
    } catch (err) {
      console.error(err);
      setMessages((prev) => [
        ...prev,
        {
          id: Date.now().toString(),
          role: "assistant",
          content: "Sorry, I had trouble processing that request. Let's try again.",
        },
      ]);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleActionApprove = async (messageId: string, cardId: string) => {
    setMessages((prev) =>
      prev.map((msg) => {
        if (msg.id === messageId && msg.actionCards) {
          const updatedCards = msg.actionCards.map((card) =>
            card.id === cardId ? { ...card, status: "approved" as const } : card
          );

          const allApproved = updatedCards.every(c => c.status === "approved");

          if (allApproved) {
            setTimeout(() => {
                router.push("/dashboard");
            }, 1000);
          }

          return {
            ...msg,
            actionCards: updatedCards,
          };
        }
        return msg;
      })
    );
  };

  return (
    <div className="flex flex-col h-screen bg-gray-50 dark:bg-zinc-950 max-w-md mx-auto relative overflow-hidden shadow-2xl">
      <div className="flex-none p-4 bg-white/80 dark:bg-zinc-900/80 backdrop-blur-xl border-b border-gray-200 dark:border-zinc-800 z-10 sticky top-0">
        <h1 className="text-lg font-semibold flex items-center gap-2">
          <FiCpu className="text-blue-500" />
          Onboarding Assistant
        </h1>
        <p className="text-sm text-gray-500">Zero-Click Setup</p>
      </div>

      <div className="flex-grow overflow-y-auto p-4 space-y-6 bg-gradient-to-b from-gray-50 to-white dark:from-zinc-950 dark:to-zinc-900">
        <AnimatePresence>
          {messages.map((msg) => (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              key={msg.id}
              className={`flex flex-col ${
                msg.role === "user" ? "items-end" : "items-start"
              }`}
            >
              <div
                className={`max-w-[85%] rounded-2xl px-4 py-3 ${
                  msg.role === "user"
                    ? "bg-blue-600 text-white rounded-br-none"
                    : "bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-700 rounded-bl-none shadow-sm text-gray-800 dark:text-gray-100"
                }`}
              >
                <p className="whitespace-pre-wrap text-[15px] leading-relaxed">{msg.content}</p>
              </div>

              {msg.actionCards && msg.actionCards.length > 0 && (
                <div className="mt-4 w-full space-y-3">
                  {msg.actionCards.map((card) => (
                    <motion.div
                      key={card.id}
                      initial={{ opacity: 0, scale: 0.95 }}
                      animate={{ opacity: 1, scale: 1 }}
                      className="bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-700 rounded-xl p-4 shadow-sm"
                    >
                      <h3 className="font-medium text-gray-900 dark:text-white mb-1">
                        {card.title}
                      </h3>
                      <p className="text-sm text-gray-500 dark:text-gray-400 mb-4">
                        {card.description}
                      </p>
                      <div className="flex justify-end">
                        <button
                          onClick={() => handleActionApprove(msg.id, card.id)}
                          disabled={card.status === "approved"}
                          className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                            card.status === "approved"
                              ? "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400"
                              : "bg-blue-50 text-blue-600 hover:bg-blue-100 dark:bg-blue-900/30 dark:text-blue-400 dark:hover:bg-blue-900/50"
                          }`}
                        >
                          {card.status === "approved" ? (
                            <>
                              <FiCheck /> Approved
                            </>
                          ) : (
                            "Approve"
                          )}
                        </button>
                      </div>
                    </motion.div>
                  ))}
                </div>
              )}
            </motion.div>
          ))}

          {isProcessing && (
             <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="flex items-start"
              >
                <div className="bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-700 rounded-2xl rounded-bl-none px-4 py-3 shadow-sm flex items-center gap-2">
                   <div className="flex space-x-1">
                      <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
                      <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
                      <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
                   </div>
                </div>
             </motion.div>
          )}
        </AnimatePresence>
        <div ref={messagesEndRef} className="h-4" />
      </div>

      <div className="flex-none p-4 bg-white/80 dark:bg-zinc-900/80 backdrop-blur-xl border-t border-gray-200 dark:border-zinc-800 z-10 sticky bottom-0">
        <form onSubmit={handleSubmit} className="flex gap-2">
          <div className="relative flex-grow">
             <input
               type="text"
               value={input}
               onChange={(e) => setInput(e.target.value)}
               placeholder="Describe your business..."
               disabled={isProcessing}
               className="w-full bg-gray-100 dark:bg-zinc-800 border-none rounded-full pl-4 pr-10 py-3 text-sm focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
             />
             <button type="button" className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 transition-colors" onClick={() => {
                const url = prompt("Enter an image URL for your product:");
                if (url) setImageUrl(url);
             }}>
                 <FiImage />
             </button>
          </div>
          <button
            type="submit"
            disabled={!input.trim() || isProcessing}
            className="flex-none bg-blue-600 hover:bg-blue-700 text-white rounded-full p-3 disabled:opacity-50 disabled:hover:bg-blue-600 transition-colors shadow-sm"
          >
            <FiSend />
          </button>
        </form>
      </div>
    </div>
  );
}
