import React, { useState } from 'react';
import { HelpCenterData } from './HelpCenterData';

export const HelpCenterWidget: React.FC = () => {
    const [isOpen, setIsOpen] = useState(false);
    const [searchQuery, setSearchQuery] = useState('');
    const [activeArticle, setActiveArticle] = useState<string | null>(null);

    const toggleOpen = () => setIsOpen(!isOpen);

    const filteredArticles = HelpCenterData.articles.filter(article =>
        article.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        article.content.toLowerCase().includes(searchQuery.toLowerCase())
    );

    return (
        <>
            <button
                onClick={toggleOpen}
                style={{
                    position: 'fixed',
                    bottom: '20px',
                    right: '20px',
                    width: '60px',
                    height: '60px',
                    borderRadius: '50%',
                    backgroundColor: '#0070f3',
                    color: 'white',
                    border: 'none',
                    fontSize: '24px',
                    cursor: 'pointer',
                    boxShadow: '0 4px 12px rgba(0,0,0,0.15)',
                    zIndex: 9999
                }}
                aria-label="Open Help Center"
            >
                ?
            </button>

            {isOpen && (
                <div style={{
                    position: 'fixed',
                    bottom: '90px',
                    right: '20px',
                    width: '350px',
                    maxHeight: '600px',
                    height: '80vh',
                    backgroundColor: 'white',
                    borderRadius: '12px',
                    boxShadow: '0 8px 30px rgba(0,0,0,0.12)',
                    display: 'flex',
                    flexDirection: 'column',
                    overflow: 'hidden',
                    zIndex: 9999,
                    fontFamily: 'Inter, sans-serif'
                }}>
                    <div style={{
                        padding: '20px',
                        backgroundColor: '#f8f9fa',
                        borderBottom: '1px solid #eaeaea',
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center'
                    }}>
                        <h2 style={{ margin: 0, fontSize: '18px', fontWeight: 600 }}>Help Center</h2>
                        {activeArticle && (
                            <button
                                onClick={() => setActiveArticle(null)}
                                style={{ background: 'none', border: 'none', color: '#0070f3', cursor: 'pointer' }}
                            >
                                Back
                            </button>
                        )}
                    </div>

                    {!activeArticle ? (
                        <div style={{ padding: '20px', overflowY: 'auto', flex: 1 }}>
                            <input
                                type="text"
                                placeholder="Search for help..."
                                value={searchQuery}
                                onChange={(e) => setSearchQuery(e.target.value)}
                                style={{
                                    width: '100%',
                                    padding: '10px 15px',
                                    borderRadius: '8px',
                                    border: '1px solid #eaeaea',
                                    marginBottom: '20px',
                                    boxSizing: 'border-box'
                                }}
                            />

                            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                                {filteredArticles.map(article => (
                                    <div
                                        key={article.id}
                                        onClick={() => setActiveArticle(article.id)}
                                        style={{
                                            padding: '15px',
                                            borderRadius: '8px',
                                            border: '1px solid #eaeaea',
                                            cursor: 'pointer',
                                            transition: 'background-color 0.2s',
                                        }}
                                        onMouseEnter={(e) => e.currentTarget.style.backgroundColor = '#f8f9fa'}
                                        onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'white'}
                                    >
                                        <h3 style={{ margin: '0 0 5px 0', fontSize: '16px', color: '#333' }}>{article.title}</h3>
                                        <p style={{ margin: 0, fontSize: '14px', color: '#666', display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical', overflow: 'hidden' }}>
                                            {article.summary}
                                        </p>
                                    </div>
                                ))}
                            </div>
                        </div>
                    ) : (
                        <div style={{ padding: '20px', overflowY: 'auto', flex: 1 }}>
                            {(() => {
                                const article = HelpCenterData.articles.find(a => a.id === activeArticle);
                                return article ? (
                                    <div>
                                        <h1 style={{ marginTop: 0, fontSize: '20px' }}>{article.title}</h1>
                                        <div
                                            style={{ lineHeight: 1.6, color: '#444' }}
                                            dangerouslySetInnerHTML={{ __html: article.content }}
                                        />
                                    </div>
                                ) : null;
                            })()}
                        </div>
                    )}

                    <div style={{
                        padding: '15px',
                        borderTop: '1px solid #eaeaea',
                        backgroundColor: '#f8f9fa',
                        textAlign: 'center'
                    }}>
                        <button style={{
                            width: '100%',
                            padding: '10px',
                            backgroundColor: 'white',
                            border: '1px solid #eaeaea',
                            borderRadius: '8px',
                            cursor: 'pointer',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            gap: '8px',
                            fontWeight: 500
                        }}>
                            Ask AI Support Agent
                        </button>
                    </div>
                </div>
            )}
        </>
    );
};
