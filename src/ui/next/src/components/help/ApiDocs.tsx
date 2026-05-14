"use client";
import React, { useEffect, useState } from 'react';

export default function ApiDocs() {
    const [specUrl, setSpecUrl] = useState<string | null>(null);

    useEffect(() => {
        const host = typeof window !== 'undefined' ? window.location.origin : 'https://api.onehumancorp.com';
        setSpecUrl(`${host}/api/docs/openapi.json`);
    }, []);

    return (
        <div className="api-docs advanced-section" style={{ padding: '20px' }}>
            <h1>API Documentation</h1>
            <p style={{ color: '#666', marginBottom: '20px' }}>For advanced users connecting custom checkouts and integrations.</p>

            <div className="swagger-ui-container" style={{
                border: '1px solid #e2e8f0',
                borderRadius: '12px',
                height: '800px',
                overflow: 'hidden',
                boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)'
            }}>
                {specUrl && (
                    <iframe
                        src={`https://petstore.swagger.io/?url=${encodeURIComponent(specUrl)}`}
                        width="100%"
                        height="100%"
                        frameBorder="0"
                        title="Interactive API Reference"
                    />
                )}
            </div>
        </div>
    );
}
