with open('.agent-task/missions/1775029506_hybrid_rag_sync.yml', 'r') as f:
    content = f.read()

new_content = "status: DONE\nagent: jules\n" + content

with open('.agent-task/missions/1775029506_hybrid_rag_sync.yml', 'w') as f:
    f.write(new_content)
