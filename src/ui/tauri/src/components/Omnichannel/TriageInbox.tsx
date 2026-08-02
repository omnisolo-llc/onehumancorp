import React from 'react';

export const TriageInbox: React.FC = () => {
    return (
        <div className="triage-inbox" style={{ maxWidth: '375px', margin: '0 auto' }}>
            <h2>Triage Feed</h2>
            <div className="conversation-card">
                <p>New message from customer...</p>
                <div className="draft-reply">
                    <em>AI Draft: Hello, how can I help you?</em>
                </div>
            </div>
        </div>
    );
};
