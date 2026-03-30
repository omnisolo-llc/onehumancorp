import os
import re

def validate_design_docs():
    design_doc = "../../docs/features/modular-plugins/design-doc.md"
    roadmap = "../../docs/roadmap.md"

    # Check if files exist
    if not os.path.exists(design_doc):
        return False, "design-doc.md not found"

    if not os.path.exists(roadmap):
        return False, "roadmap.md not found"

    with open(design_doc, "r") as f:
        content = f.read()

    # Check for date stamp
    if not re.search(r'\*\*Date:\*\*\s*\d{4}-\d{2}-\d{2}', content):
        return False, "Date-stamp missing from design doc"

    # Check for Mermaid diagram
    if "```mermaid" not in content:
        return False, "Mermaid diagram missing from design doc"

    # Check for aesthetic tokens
    required_tokens = ["backdrop-filter: blur(15px) saturate(180%)", "background: rgba(255, 255, 255, 0.05)", "border: 1px solid rgba(255, 255, 255, 0.1)", "Outfit"]
    for token in required_tokens:
        if token not in content:
            return False, f"Aesthetic token missing: {token}"

    with open(roadmap, "r") as f:
        content = f.read()

    if "Capability Plugin Mesh" not in content:
        return False, "Capability Plugin Mesh not found in roadmap"

    return True, "All validations passed"

if __name__ == "__main__":
    success, msg = validate_design_docs()
    if not success:
        print(f"Validation failed: {msg}")
        exit(1)
    print("Architecture validation passed successfully.")
