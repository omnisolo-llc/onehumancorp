"use client";

import React, { useEffect, useState, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { FiMessageSquare, FiX, FiSend, FiCheck } from "react-icons/fi";

export const ChatWidget: React.FC = () => {
    const [isOpen, setIsOpen] = useState(false);
    const [messages, setMessages] = useState<{ id: string; text: string; sender: "user" | "agent" | "draft" }[]>([]);
    const [inputValue, setInputValue] = useState("");
    const wsRef = useRef<WebSocket | null>(null);
    const messagesEndRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (isOpen && !wsRef.current) {
            const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
            const wsUrl = `${protocol}//${window.location.host}/api/v1/native-chat/ws`;
            const ws = new WebSocket(wsUrl);

            ws.onopen = () => {
                console.log("Connected to Native Chat");
            };

            ws.onmessage = (event) => {
                try {
                    const data = JSON.parse(event.data);
                    if (data.status === "received") {
                        setMessages((prev) => [
                            ...prev,
                            { id: Date.now().toString(), text: "Draft reply ready for approval", sender: "draft" },
                        ]);
                    }
                } catch (e) {
                    console.error("Error parsing websocket message", e);
                }
            };

            ws.onclose = () => {
                console.log("Disconnected from Native Chat");
                wsRef.current = null;
            };

            wsRef.current = ws;
        }

        return () => {
            if (wsRef.current && !isOpen) {
                wsRef.current.close();
                wsRef.current = null;
            }
        };
    }, [isOpen]);

    useEffect(() => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    }, [messages]);

    const handleSend = () => {
        if (!inputValue.trim()) return;

        setMessages((prev) => [...prev, { id: Date.now().toString(), text: inputValue, sender: "user" }]);

        if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
            wsRef.current.send(inputValue);
        }

        setInputValue("");
    };

    const handleApproveDraft = (id: string) => {
        setMessages((prev) =>
            prev.map((msg) =>
                msg.id === id ? { ...msg, text: "Approved: " + msg.text, sender: "agent" } : msg
            )
        );
    };

    return (
        <div className="fixed bottom-4 right-4 z-50">
            <AnimatePresence>
                {isOpen && (
                    <motion.div
                        initial={{ opacity: 0, y: 20, scale: 0.95 }}
                        animate={{ opacity: 1, y: 0, scale: 1 }}
                        exit={{ opacity: 0, y: 20, scale: 0.95 }}
                        transition={{ duration: 0.2 }}
                        className="mb-4 flex flex-col w-[350px] max-w-[calc(100vw-2rem)] h-[500px] max-h-[calc(100vh-6rem)] bg-white/70 dark:bg-zinc-900/70 backdrop-blur-xl border border-zinc-200/50 dark:border-zinc-800/50 rounded-2xl shadow-2xl overflow-hidden"
                    >
                        <div className="flex items-center justify-between px-4 py-3 border-b border-zinc-200/50 dark:border-zinc-800/50 bg-white/50 dark:bg-zinc-900/50">
                            <h3 className="font-semibold text-zinc-900 dark:text-zinc-100">Work Triage</h3>
                            <button
                                onClick={() => setIsOpen(false)}
                                className="p-2 text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100 rounded-full transition-colors"
                            >
                                <FiX size={20} />
                            </button>
                        </div>

                        <div className="flex-1 overflow-y-auto p-4 space-y-4">
                            {messages.map((msg) => (
                                <div
                                    key={msg.id}
                                    className={`flex ${
                                        msg.sender === "user" ? "justify-end" : "justify-start"
                                    }`}
                                >
                                    <div
                                        className={`max-w-[85%] rounded-2xl px-4 py-2 ${
                                            msg.sender === "user"
                                                ? "bg-blue-600 text-white"
                                                : msg.sender === "draft"
                                                ? "bg-amber-100/80 dark:bg-amber-900/30 text-amber-900 dark:text-amber-100 border border-amber-300/50 dark:border-amber-700/50 shadow-[0_0_15px_rgba(251,191,36,0.3)] ring-1 ring-amber-400/50"
                                                : "bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100"
                                        }`}
                                    >
                                        <p className="text-sm">{msg.text}</p>
                                        {msg.sender === "draft" && (
                                            <div className="mt-2 flex justify-end">
                                                <button
                                                    onClick={() => handleApproveDraft(msg.id)}
                                                    className="flex items-center space-x-1 text-xs font-medium text-amber-700 dark:text-amber-300 hover:text-amber-900 dark:hover:text-amber-100 transition-colors"
                                                >
                                                    <FiCheck size={14} />
                                                    <span>Approve</span>
                                                </button>
                                            </div>
                                        )}
                                    </div>
                                </div>
                            ))}
                            <div ref={messagesEndRef} />
                        </div>

                        <div className="p-4 border-t border-zinc-200/50 dark:border-zinc-800/50 bg-white/50 dark:bg-zinc-900/50">
                            <div className="flex items-center space-x-2">
                                <input
                                    type="text"
                                    value={inputValue}
                                    onChange={(e) => setInputValue(e.target.value)}
                                    onKeyDown={(e) => e.key === "Enter" && handleSend()}
                                    placeholder="Type a message..."
                                    className="flex-1 px-4 py-2 bg-zinc-100 dark:bg-zinc-800/50 border border-zinc-200 dark:border-zinc-700 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500/50 text-sm text-zinc-900 dark:text-zinc-100"
                                />
                                <button
                                    onClick={handleSend}
                                    className="p-2 bg-blue-600 text-white rounded-full hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500/50"
                                >
                                    <FiSend size={18} />
                                </button>
                            </div>
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>

            <button
                onClick={() => setIsOpen(!isOpen)}
                className="w-14 h-14 flex items-center justify-center bg-blue-600 text-white rounded-full shadow-lg hover:bg-blue-700 transition-all transform hover:scale-105 focus:outline-none focus:ring-4 focus:ring-blue-500/30"
                aria-label="Open chat"
            >
                {isOpen ? <FiX size={24} /> : <FiMessageSquare size={24} />}
            </button>
        </div>
    );
};
