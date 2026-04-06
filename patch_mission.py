with open('.agent-task/missions/1775249001_palette_refactor_cards_glassmorphism.md', 'r') as f:
    content = f.read()
content = content.replace("status: PENDING", "status: DONE")
with open('.agent-task/missions/1775249001_palette_refactor_cards_glassmorphism.md', 'w') as f:
    f.write(content)
