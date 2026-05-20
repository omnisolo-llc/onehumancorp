import React from 'react';
import { Text, Box } from 'ink';

export interface ToolItem {
  name: string;
  status: 'pending' | 'success' | 'error';
}

export interface ToolProgressProps {
  tools: ToolItem[];
}

export const ToolProgress: React.FC<ToolProgressProps> = ({ tools }) => {
  return (
    <Box flexDirection="column" marginY={1} borderStyle="round" borderColor="dim" padding={1}>
      <Box marginBottom={1}>
        <Text bold color="magenta">Tools Executed:</Text>
      </Box>
      {tools.map((tool, index) => {
        let icon = '[ ]';
        let color = 'gray';
        let isDim = false;

        if (tool.status === 'success') {
          icon = '[✓]';
          color = 'green';
        } else if (tool.status === 'error') {
          icon = '[x]';
          color = 'red';
        } else {
          icon = '[~]';
          color = 'yellow';
          isDim = true;
        }

        return (
          <Box key={index} paddingLeft={2}>
            <Text color={color} dimColor={isDim}>{icon}</Text>
            <Text dimColor={isDim}> {tool.name}</Text>
          </Box>
        );
      })}
    </Box>
  );
};
