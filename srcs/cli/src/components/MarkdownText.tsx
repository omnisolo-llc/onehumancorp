import React from 'react';
import { Text, Box } from 'ink';

export interface MarkdownTextProps {
  content: string;
}

export const MarkdownText: React.FC<MarkdownTextProps> = ({ content }) => {
  const lines = content.split('\n');

  return (
    <Box flexDirection="column" marginY={1}>
      {lines.map((line, index) => {
        if (line.startsWith('# ')) {
          return (
            <Box key={index} marginBottom={1}>
              <Text bold color="cyan">{line.replace('# ', '')}</Text>
            </Box>
          );
        } else if (line.startsWith('## ')) {
          return (
            <Box key={index} marginBottom={1}>
              <Text bold color="blue">{line.replace('## ', '')}</Text>
            </Box>
          );
        } else if (line.startsWith('- ')) {
          return (
            <Box key={index} paddingLeft={2}>
              <Text color="gray">• </Text>
              <Text>{line.replace('- ', '')}</Text>
            </Box>
          );
        } else if (line.trim() === '') {
          return <Box key={index} height={1} />;
        }
        return (
          <Box key={index}>
            <Text>{line}</Text>
          </Box>
        );
      })}
    </Box>
  );
};
