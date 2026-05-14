"use client";
import React, { useState } from 'react';

export default function HelpChat() {
    const [msg, setMsg] = useState('');
    const [reply, setReply] = useState('');
    const [articleLink, setArticleLink] = useState<string | null>(null);
    const [isOpen, setIsOpen] = useState(false);

    const sendChat = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const res = await fetch('/api/help/chat', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ message: msg })
            });
            const data = await res.json();
            setReply(data.reply);
            setArticleLink(data.article_link);
        } catch {
            setReply("Mock Answer");
        }
    };

    return (
        <div className="floating-help-chat" style={{
            position: 'fixed',
            bottom: '20px',
            right: '20px',
            zIndex: 999999
        }}>
            {!isOpen && (
                <button onClick={() => setIsOpen(true)} style={{
                    borderRadius: '50%', width: '50px', height: '50px',
                    background: '#0070f3', color: '#fff', border: 'none',
                    backdropFilter: 'blur(20px) saturate(200%)',
                    cursor: 'pointer'
                }}>Ask</button>
            )}

            {isOpen && (
                <div style={{
                    width: '300px',
                    height: '400px',
                    background: 'rgba(255,255,255,0.95)',
                    backdropFilter: 'blur(20px) saturate(200%)',
                    borderRadius: '12px',
                    border: '1px solid rgba(0,0,0,0.1)',
                    display: 'flex',
                    flexDirection: 'column',
                    padding: '16px',
                    boxShadow: '0 10px 25px rgba(0,0,0,0.1)'
                }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                        <h3 style={{ margin: 0 }}>Ask anything</h3>
                        <button onClick={() => setIsOpen(false)} style={{ background: 'none', border: 'none', cursor: 'pointer' }}>✕</button>
                    </div>

                    <div style={{ flex: 1, overflowY: 'auto', marginTop: '10px' }}>
                        {reply && <div className="reply" style={{lineHeight: "1.5"}}>{reply} {articleLink && <div style={{marginTop: "8px"}}><a href={articleLink} style={{color: "#0070f3", textDecoration: "none", fontWeight: "bold"}}>Read the full article →</a></div>}</div>}
                    </div>

                    <form onSubmit={sendChat} style={{ display: 'flex', marginTop: '10px' }}>
                        <input
                            name="chat"
                            value={msg}
                            onChange={e => setMsg(e.target.value)}
                            style={{ flex: 1, padding: '8px', borderRadius: '4px', border: '1px solid #ccc' }}
                            placeholder="How do I setup my store?"
                        />
                        <button type="submit" style={{ marginLeft: '8px', padding: '8px', background: '#0070f3', color: '#fff', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>Send</button>
                    </form>
                </div>
            )}
        </div>
    );
}
