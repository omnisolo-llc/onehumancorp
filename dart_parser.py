import sys
import os

screens = [
    "srcs/app/lib/screens/handoffs_screen.dart",
    "srcs/app/lib/screens/landing_screen.dart",
    "srcs/app/lib/screens/pipelines_screen.dart",
    "srcs/app/lib/screens/integrations_screen.dart",
    "srcs/app/lib/screens/scaling_screen.dart",
    "srcs/app/lib/screens/security_screen.dart",
    "srcs/app/lib/screens/dashboard_screen.dart",
    "srcs/app/lib/screens/ai_config_screen.dart",
    "srcs/app/lib/screens/settings_screen.dart",
    "srcs/app/lib/screens/agents_screen.dart",
    "srcs/app/lib/screens/service_screen.dart",
    "srcs/app/lib/screens/meetings_screen.dart",
    "srcs/app/lib/screens/cost_dashboard_screen.dart",
    "srcs/app/lib/screens/swarm_memory_screen.dart",
    "srcs/app/lib/screens/skills_screen.dart",
    "srcs/app/lib/screens/channels_screen.dart",
    "srcs/app/lib/screens/wizard_screen.dart"
]

def remove_direct_properties(body_str, prop_names):
    # Removes properties like "color:" from body_str, but ONLY if they are at the top-level
    # within the Card() arguments, i.e., at nesting level 0.

    out_parts = []
    idx = 0
    nesting_parens = 0
    nesting_brackets = 0
    nesting_braces = 0

    current_chunk = ""

    while idx < len(body_str):
        char = body_str[idx]

        if char == '(': nesting_parens += 1
        elif char == ')': nesting_parens -= 1
        elif char == '[': nesting_brackets += 1
        elif char == ']': nesting_brackets -= 1
        elif char == '{': nesting_braces += 1
        elif char == '}': nesting_braces -= 1

        if nesting_parens == 0 and nesting_brackets == 0 and nesting_braces == 0 and char == ',':
            current_chunk += char
            out_parts.append(current_chunk)
            current_chunk = ""
            idx += 1
            continue

        current_chunk += char
        idx += 1

    if current_chunk:
        out_parts.append(current_chunk)

    final_chunks = []
    for chunk in out_parts:
        stripped = chunk.strip()
        should_keep = True
        for prop in prop_names:
            if stripped.startswith(prop + ":") or stripped.startswith(prop + " :"):
                should_keep = False
                break
        if should_keep:
            final_chunks.append(chunk)

    return "".join(final_chunks)

def process_file(filepath):
    if not os.path.exists(filepath):
        return

    with open(filepath, 'r') as f:
        content = f.read()

    if "Card(" not in content:
        return

    print(f"Refactoring {filepath}")

    # Add import
    if "glass_card.dart" not in content:
        import_statement = "import 'package:ohc_app/widgets/glass_card.dart';\n"
        last_import_idx = content.rfind("import ")
        if last_import_idx != -1:
            end_of_line = content.find(";", last_import_idx)
            content = content[:end_of_line + 1] + "\n" + import_statement + content[end_of_line + 1:]
        else:
            content = import_statement + content

    out_text = ""
    idx = 0
    while True:
        card_idx = content.find("Card(", idx)
        if card_idx == -1:
            out_text += content[idx:]
            break

        if card_idx > 0 and content[card_idx-1].isalpha():
            out_text += content[idx:card_idx+5]
            idx = card_idx + 5
            continue

        out_text += content[idx:card_idx] + "GlassCard("

        start_body = card_idx + 5
        parens = 1
        curr = start_body

        # Fast forward until we reach the closing paren of Card()
        while curr < len(content) and parens > 0:
            if content[curr] == '(': parens += 1
            elif content[curr] == ')': parens -= 1
            curr += 1

        end_body = curr

        card_body = content[start_body:end_body-1]

        card_body = remove_direct_properties(card_body, ["elevation", "shape", "color", "surfaceTintColor", "shadowColor"])

        out_text += card_body + ")"
        idx = end_body

    with open(filepath, "w") as f:
        f.write(out_text)

for s in screens:
    process_file(s)
