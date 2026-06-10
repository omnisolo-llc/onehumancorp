import React, { useState, useEffect } from 'react';
import { Box, Text } from 'ink';

export const AgentProtocol: React.FC = () => {
  const [tasks, setTasks] = useState<{ task_id: string; input: string }[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('http://127.0.0.1:8080/rpc', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        jsonrpc: '2.0',
        method: 'ap_list_tasks',
        params: {},
        id: 1,
      }),
    })
      .then((res) => res.json())
      .then((data) => {
        if (data.result && Array.isArray(data.result)) {
          setTasks(data.result);
        } else {
          setError('Error loading tasks');
        }
      })
      .catch((err) => {
        setError('Network Error: ' + err.message);
      });
  }, []);

  return (
    <Box flexDirection="column" padding={1} borderStyle="round" borderColor="cyan">
      <Text color="cyan" bold>Agent Protocol Tasks</Text>
      {error && <Text color="red">{error}</Text>}
      {!error && tasks.length === 0 && <Text>No tasks found.</Text>}
      {tasks.map((t) => (
        <Box key={t.task_id} flexDirection="row">
          <Text color="green">Task: {t.task_id}</Text>
          <Text> - {t.input}</Text>
        </Box>
      ))}
    </Box>
  );
};
