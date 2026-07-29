import React, { useState, useEffect } from 'react';

interface TriageItem {
  id: string;
  customer_id: string;
  source: string;
  priority: string;
  context: string;
  action_type: string;
  action_payload: string;
  created_at: string;
}

export const WorkTriageFeed: React.FC = () => {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadItems = async () => {
        setLoading(true);
        try {
            const res = await fetch('/api/v1/ui/triage');
            if (res.ok) {
                const data = await res.json();
                setItems(data);
            }
        } catch (e) {
            console.error(e);
        } finally {
            setLoading(false);
        }
    }
    loadItems();
  }, []);

  const handleSendAndRequestDeposit = (id: string) => {
    // Mock action
    setItems((prev) => prev.filter((item) => item.id !== id));
  };

  if (loading) {
      return <div>Loading triage feed...</div>;
  }

  return (
    <div style={{ maxWidth: '375px', margin: '0 auto', padding: '16px' }}>
      <h5 style={{ marginBottom: '16px' }}>
        Today's Work Triage
      </h5>
      {items.map((item) => (
        <div
          key={item.id}
          style={{
            marginBottom: '16px',
            backdropFilter: 'blur(10px)',
            backgroundColor: 'rgba(255, 255, 255, 0.7)',
            border: '1px solid rgba(255, 255, 255, 0.3)',
            padding: '16px',
            borderRadius: '8px',
          }}
        >
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
            <strong style={{ fontSize: '14px' }}>
              {item.customer_id}
            </strong>
            <span style={{ backgroundColor: '#007aff', color: 'white', padding: '2px 8px', borderRadius: '12px', fontSize: '12px' }}>{item.source}</span>
          </div>
          <p style={{ color: 'gray', marginBottom: '8px', fontSize: '14px' }}>
            <strong>Context:</strong> {item.context}
          </p>

          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(0, 122, 255, 0.05)',
              borderRadius: '8px',
              borderLeft: '4px solid #007aff',
              marginBottom: '16px',
            }}
          >
            <span style={{ color: '#007aff', display: 'block', marginBottom: '4px', fontWeight: 'bold', fontSize: '12px' }}>
              AI {item.action_type}
            </span>
            <p style={{ fontSize: '14px' }}>
              {item.action_payload}
            </p>
          </div>

          <button
            style={{
                width: '100%',
                padding: '12px',
                backgroundColor: '#007aff',
                color: 'white',
                border: 'none',
                borderRadius: '8px',
                cursor: 'pointer'
            }}
            onClick={() => handleSendAndRequestDeposit(item.id)}
          >
            Approve & Send
          </button>
        </div>
      ))}
      {items.length === 0 && (
        <p style={{ color: 'gray', textAlign: 'center' }}>
          You're all caught up!
        </p>
      )}
    </div>
  );
};
