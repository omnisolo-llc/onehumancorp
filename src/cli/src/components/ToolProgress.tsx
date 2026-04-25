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
    <Box flexDirection="column" marginY={1}>
      <Text bold color="magenta">Tools Executed:</Text>
      {tools.map((tool, index) => {
        let icon = '[ ]';
        let color = 'gray';

        if (tool.status === 'success') {
          icon = '[✓]';
          color = 'green';
        } else if (tool.status === 'error') {
          icon = '[x]';
          color = 'red';
        } else {
          icon = '[ ]';
          color = 'yellow';
        }

        return (
          <Box key={index}>
            <Text color={color}>{icon}</Text>
            <Text> {tool.name}</Text>
          </Box>
        );
      })}
    </Box>
  );
};
