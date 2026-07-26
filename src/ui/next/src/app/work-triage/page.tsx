"use client";

import { useEffect, useState } from "react";
import { getChatFeed } from "../api/v1/chat/chatBackend";

export default function WorkTriagePage() {
    const [feed, setFeed] = useState<any[]>([]);

    useEffect(() => {
        const fetchFeed = async () => {
            const data = await getChatFeed();
            setFeed(data);
        };
        fetchFeed();
    }, []);

    return (
        <div style={{ maxWidth: "375px", margin: "0 auto", padding: "16px" }}>
            <h1>Work Triage</h1>
            {feed.length === 0 ? (
                <p>No active conversations.</p>
            ) : (
                feed.map((conv, idx) => (
                    <div key={idx} style={{ border: "1px solid #ccc", padding: "16px", marginBottom: "8px", borderRadius: "8px" }}>
                        <p><strong>Status:</strong> {conv.status}</p>
                        <button>Approve & Send</button>
                    </div>
                ))
            )}
        </div>
    );
}
