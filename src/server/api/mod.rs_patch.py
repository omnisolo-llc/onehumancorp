import os
with open("src/server/api/mod.rs", "r") as f:
    c = f.read()
if "pub mod work_triage;" not in c:
    with open("src/server/api/mod.rs", "w") as f:
        f.write(c + "\npub mod work_triage;\n")
