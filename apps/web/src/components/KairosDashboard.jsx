import React, { useEffect, useState } from 'react';
import { GlassCard } from './GlassCard';

export function KairosDashboard() {
  const [tasks, setTasks] = useState([]);
  const [meshStream, setMeshStream] = useState([]);
  const [autoDream, setAutoDream] = useState([]);

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:8080/api/kairos/stream');

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === 'mesh:tasks') {
          setTasks(prev => [{ id: Date.now(), ...data }, ...prev].slice(0, 50));
        } else if (data.type === 'mesh:coordination') {
          setMeshStream(prev => [{ id: Date.now(), ...data }, ...prev].slice(0, 50));
        } else {
          setAutoDream(prev => [{ id: Date.now(), ...data }, ...prev].slice(0, 50));
        }
      } catch (err) {
        console.error('Failed to parse event', err);
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div style={{ display: 'flex', gap: '24px', padding: '24px' }}>
      <div style={{ flex: 1 }}>
        <h2 style={{ fontFamily: 'Outfit', fontSize: '20px' }}>Shared Task Queue</h2>
        <GlassCard>
          {tasks.length === 0 ? <p>No tasks yet...</p> : (
            <ul>
              {tasks.map(task => (
                <li key={task.id} style={{ fontFamily: 'Inter' }}>
                  {task.event} - {task.status}
                </li>
              ))}
            </ul>
          )}
        </GlassCard>
      </div>

      <div style={{ flex: 1 }}>
        <h2 style={{ fontFamily: 'Outfit', fontSize: '20px' }}>Teammate Mesh Stream</h2>
        <GlassCard>
          {meshStream.length === 0 ? <p>No mesh events yet...</p> : (
            <ul>
              {meshStream.map(msg => (
                <li key={msg.id} style={{ fontFamily: 'Inter' }}>
                  {msg.event} - {msg.status}
                </li>
              ))}
            </ul>
          )}
        </GlassCard>
      </div>

      <div style={{ flex: 1 }}>
        <h2 style={{ fontFamily: 'Outfit', fontSize: '20px' }}>AutoDream Memory</h2>
        <GlassCard>
          {autoDream.length === 0 ? <p>No memory events yet...</p> : (
            <ul>
              {autoDream.map(mem => (
                <li key={mem.id} style={{ fontFamily: 'Inter' }}>
                  {mem.event} - {mem.status}
                </li>
              ))}
            </ul>
          )}
        </GlassCard>
      </div>
    </div>
  );
}
