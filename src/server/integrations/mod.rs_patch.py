import os
with open("src/server/integrations/mod.rs", "r") as f:
    c = f.read()
if "pub use ::server_integrations_taxjar as taxjar;" not in c:
    c = c.replace("// pub use ::server_integrations_taxjar as taxjar;", "pub use ::server_integrations_taxjar as taxjar;")
    with open("src/server/integrations/mod.rs", "w") as f:
        f.write(c)
