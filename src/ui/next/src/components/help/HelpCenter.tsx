"use client";
import React, { useState, useEffect, useMemo } from 'react';

export default function HelpCenter() {
    const [articles, setArticles] = useState<{id: string, title: string, content: string, topic: string, keywords: string[]}[]>([]);
    const [searchTerm, setSearchTerm] = useState('');
    const [activeTopic, setActiveTopic] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        fetch('/api/help/articles')
            .then(res => res.json())
            .then(data => { setArticles(data); setIsLoading(false); })
            .catch(() => setIsLoading(false));
    }, []);

    const topics = useMemo(() => {
        const t = new Set<string>();
        articles.forEach(a => t.add(a.topic));
        return Array.from(t);
    }, [articles]);

    const filtered = useMemo(() => {
        return articles.filter(a => {
            const matchesSearch = a.title.toLowerCase().includes(searchTerm.toLowerCase()) || a.content.toLowerCase().includes(searchTerm.toLowerCase()) || a.keywords.some(k => k.toLowerCase().includes(searchTerm.toLowerCase()));
            const matchesTopic = activeTopic ? a.topic === activeTopic : true;
            return matchesSearch && matchesTopic;
        });
    }, [articles, searchTerm, activeTopic]);

    const renderMarkdown = (text: string) => {
        let html = text
            .replace(/^### (.*$)/gim, '<h3>$1</h3>')
            .replace(/^## (.*$)/gim, '<h2>$1</h2>')
            .replace(/^# (.*$)/gim, '<h1>$1</h1>')
            .replace(/^> (.*$)/gim, '<blockquote>$1</blockquote>')
            .replace(/\*\*(.*?)\*\*/gim, '<b>$1</b>')
            .replace(/\*(.*?)\*/gim, '<i>$1</i>')
            .replace(/!\[(.*?)\]\((.*?)\)/gim, "<img alt='$1' src='$2' />")
            .replace(/\[(.*?)\]\((.*?)\)/gim, "<a href='$2'>$1</a>")
            .replace(/\n$/gim, '<br />');
        return { __html: html.trim() };
    };

    if (isLoading) return <div style={{ padding: '20px' }}>Loading Help Center...</div>;

    return (
        <div className="help-center-container" style={{ backdropFilter: 'blur(20px) saturate(200%)', padding: '32px', maxWidth: '1200px', margin: '0 auto', fontFamily: "'Inter', sans-serif" }}>
            <h1 style={{ fontSize: '2.5rem', marginBottom: '8px', fontFamily: "'Outfit', sans-serif" }}>Help Center</h1>
            <p style={{ color: '#666', marginBottom: '32px', fontSize: '1.1rem' }}>Find answers, guides, and tutorials to help you grow your business.</p>
            <div style={{ display: 'flex', gap: '24px', flexWrap: 'wrap' }}>
                <aside style={{ flex: '1 1 250px', minWidth: '250px' }}>
                    <div style={{ position: 'sticky', top: '24px' }}>
                        <div style={{ marginBottom: '24px' }}>
                            <input type="text" placeholder="Search articles or keywords..." value={searchTerm} onChange={e => setSearchTerm(e.target.value)} style={{ width: '100%', padding: '12px 16px', borderRadius: '8px', border: '1px solid #e2e8f0', fontSize: '1rem', outline: 'none', boxShadow: '0 2px 4px rgba(0,0,0,0.02)' }} />
                        </div>
                        <h3 style={{ fontSize: '0.9rem', textTransform: 'uppercase', color: '#888', letterSpacing: '0.05em', marginBottom: '12px' }}>Topics</h3>
                        <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
                            <li style={{ marginBottom: '8px' }}>
                                <button onClick={() => setActiveTopic(null)} style={{ background: 'none', border: 'none', padding: '8px 12px', width: '100%', textAlign: 'left', borderRadius: '6px', cursor: 'pointer', fontWeight: activeTopic === null ? '600' : '400', backgroundColor: activeTopic === null ? '#f1f5f9' : 'transparent', color: activeTopic === null ? '#0f172a' : '#475569', transition: 'all 0.2s ease' }}>All Topics</button>
                            </li>
                            {topics.map(topic => (
                                <li key={topic} style={{ marginBottom: '8px' }}>
                                    <button onClick={() => setActiveTopic(topic)} style={{ background: 'none', border: 'none', padding: '8px 12px', width: '100%', textAlign: 'left', borderRadius: '6px', cursor: 'pointer', fontWeight: activeTopic === topic ? '600' : '400', backgroundColor: activeTopic === topic ? '#f1f5f9' : 'transparent', color: activeTopic === topic ? '#0f172a' : '#475569', transition: 'all 0.2s ease' }}>{topic}</button>
                                </li>
                            ))}
                        </ul>
                    </div>
                </aside>
                <main style={{ flex: '3 1 600px' }}>
                    {filtered.length === 0 ? (
                        <div style={{ textAlign: 'center', padding: '48px', background: 'rgba(255,255,255,0.5)', borderRadius: '12px', border: '1px dashed #cbd5e1' }}>
                            <h3 style={{ color: '#64748b' }}>No articles found</h3>
                            <button onClick={() => {setSearchTerm(''); setActiveTopic(null);}} style={{ marginTop: '16px', padding: '8px 16px', background: '#f1f5f9', border: 'none', borderRadius: '6px', cursor: 'pointer', color: '#0f172a' }}>Clear Filters</button>
                        </div>
                    ) : (
                        <div style={{ display: 'grid', gap: '20px' }}>
                            {filtered.map(a => (
                                <article key={a.id} style={{ background: '#ffffff', padding: '24px', borderRadius: '12px', border: '1px solid #e2e8f0', boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.05), 0 2px 4px -1px rgba(0, 0, 0, 0.03)', transition: 'transform 0.2s ease, box-shadow 0.2s ease' }}>
                                    <div style={{ display: 'flex', alignItems: 'center', marginBottom: '12px' }}>
                                        <span style={{ fontSize: '0.8rem', background: '#e2e8f0', color: '#475569', padding: '4px 8px', borderRadius: '4px', fontWeight: '500' }}>{a.topic}</span>
                                    </div>
                                    <h2 style={{ fontSize: '1.5rem', marginBottom: '12px', color: '#0f172a', fontFamily: "'Outfit', sans-serif" }}>{a.title}</h2>
                                    <div style={{ color: '#334155', lineHeight: '1.6', fontSize: '1.05rem' }} dangerouslySetInnerHTML={renderMarkdown(a.content)} />
                                </article>
                            ))}
                        </div>
                    )}
                </main>
            </div>
        </div>
    );
}
