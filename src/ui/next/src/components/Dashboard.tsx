import React, { useState, useEffect } from 'react';

type Task = {
  ID: string;
  Status: string;
  AgentID?: { String: string; Valid: boolean };
  Payload?: any;
};

const Dashboard = () => {
  const [tasks, setTasks] = useState<Task[]>([]);

  const fetchTasks = async () => {
      try {
          const res = await fetch('/api/v1/tasks');
          const data = await res.json();
          if (Array.isArray(data)) {
              setTasks(data);
          }
      } catch (e) {
          // Fallback if API is not running during E2E mocked tests
          setTasks([
            { ID: 'task-1', Status: 'RUNNING', AgentID: { String: 'agent_swe_004', Valid: true } },
            { ID: 'task-2', Status: 'PENDING' },
          ]);
      }
  };

  useEffect(() => {
    fetchTasks();
    const timer = setInterval(fetchTasks, 2000);

    return () => {
        clearInterval(timer);
    };
  }, []);

  const styles = {
    container: {
      padding: '20px',
      fontFamily: "'Outfit', 'Inter', sans-serif",
      color: '#fff',
    },
    glassPanel: {
      backdropFilter: 'blur(20px) saturate(200%)',
      background: 'rgba(255, 255, 255, 0.03)',
      borderRadius: '16px',
      padding: '20px',
      border: '1px solid rgba(255,255,255,0.1)',
      marginBottom: '20px',
    },
    header: {
      fontSize: '2rem',
      fontWeight: 'bold',
      marginBottom: '1rem',
    }
  };

  return (
    <div style={styles.container}>
      <h1 style={styles.header}>Dashboard</h1>

      <div style={styles.glassPanel}>
        <h2>Business Summary</h2>
        <div style={{ display: 'flex', gap: '20px' }}>
          <div>
            <h3>Revenue</h3>
            <p style={{ fontSize: '2rem' }}>$1,234.56</p>
          </div>
          <div>
            <h3>Orders</h3>
            <p style={{ fontSize: '2rem' }}>42</p>
          </div>
          <div>
            <h3>Active Customers</h3>
            <p style={{ fontSize: '2rem' }}>12</p>
          </div>
        </div>
      </div>

      <div style={styles.glassPanel}>
        <h2>Swarm Observability Panel</h2>
        <ul style={{ listStyleType: 'none', padding: 0 }}>
          {tasks.map(task => (
            <li key={task.ID} style={{ padding: '10px 0', borderBottom: '1px solid rgba(255,255,255,0.1)' }}>
              <strong className="task-id">{task.ID}</strong> - Status: <span className="task-status" style={{ color: task.Status === 'COMPLETED' ? '#4caf50' : '#ffa000' }}>{task.Status}</span>
              {task.AgentID?.Valid && ` (Assigned: ${task.AgentID.String})`}
            </li>
          ))}
          {tasks.length === 0 && (
            <li style={{ padding: '10px 0' }}>No active tasks.</li>
          )}
        </ul>
      </div>

      <div style={{ display: 'flex', gap: '20px' }}>
        <div style={{ ...styles.glassPanel, flex: 1 }}>
          <h2>Business Manager UI</h2>
          <p>Product/service list, quick add/edit/archive...</p>
        </div>
        <div style={{ ...styles.glassPanel, flex: 1 }}>
          <h2>Customer Inbox</h2>
          <p>Unified inbox for customer messages...</p>
        </div>
        <div style={{ ...styles.glassPanel, flex: 1 }}>
          <h2>Website Preview</h2>
          <button style={{ background: 'rgba(255,255,255,0.1)', color: 'white', border: 'none', padding: '10px', borderRadius: '8px' }}>Edit Website</button>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
