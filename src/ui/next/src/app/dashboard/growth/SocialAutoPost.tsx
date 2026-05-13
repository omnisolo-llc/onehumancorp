'use client';
import React, { useState } from 'react';

export default function SocialAutoPost() {
    const [prompt, setPrompt] = useState('');
    const [generatedContent, setGeneratedContent] = useState('');
    const [postId, setPostId] = useState('');
    const [status, setStatus] = useState('');

    const handleGenerate = async () => {
        const res = await fetch('/api/v1/growth/social/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ product_id: prompt, platforms: ['instagram', 'x'] })
        });
        if (res.ok) {
            const data = await res.json();
            setGeneratedContent(data.content);
            setPostId(data.post_id);
        } else if (res.status === 402) {
            setStatus('Soft limit reached. Please upgrade to unlock more AI posting.');
        }
    };

    const handleApprove = async () => {
        const res = await fetch('/api/v1/growth/social/approve', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ post_id: postId, approved: true })
        });
        if (res.ok) setStatus('Post approved and scheduled!');
    };

    return (
        <div className="bg-white shadow rounded-lg p-6 border-t-4 border-indigo-500">
            <h2 className="text-xl font-semibold mb-2">Social Media Auto-Posting (AI)</h2>
            <p className="text-sm text-gray-500 mb-4">Connect Instagram/X. Agent auto-generates posts.</p>
            <input
                className="w-full border rounded p-2 mb-4"
                value={prompt}
                onChange={e => setPrompt(e.target.value)}
                placeholder="Enter product or milestone to post about..."
            />
            <button onClick={handleGenerate} className="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded">
                Generate AI Post
            </button>

            {generatedContent && (
                <div className="mt-4 p-4 border rounded bg-indigo-50">
                    <p className="italic text-gray-700">"{generatedContent}"</p>
                    <button onClick={handleApprove} className="mt-3 bg-green-500 text-white px-3 py-1 rounded text-sm">
                        1-Tap Approve
                    </button>
                </div>
            )}
            {status && <p className="mt-2 text-sm text-red-600 font-medium">{status}</p>}
        </div>
    );
}
