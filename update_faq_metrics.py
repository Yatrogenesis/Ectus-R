#!/usr/bin/env python3
"""Update AION-CR metrics in INVESTOR_FAQ.md"""

import re
import sys

def update_faq_metrics(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    changes = []
    original = content

    # 1. Update score 241.5 → 245-248
    content = re.sub(r'241\.5/255', '245-248/255', content)
    if content != original:
        changes.append("Updated score 241.5 → 245-248")
        original = content

    # 2. Update LOC 187,471 → 202,856
    content = re.sub(r'187,471', '202,856', content)
    if content != original:
        changes.append("Updated LOC 187,471 → 202,856")
        original = content

    # 3. Update jurisdictions 25+ → 90-100
    content = re.sub(r'25\+ jurisdicciones', '90-100 jurisdicciones', content)
    if content != original:
        changes.append("Updated jurisdictions 25+ → 90-100")
        original = content

    # 4. Update regulations 647 → ~900-1,000
    content = re.sub(r'647 regulaciones', '~900-1,000 regulaciones', content)
    if content != original:
        changes.append("Updated regulations 647 → ~900-1,000")
        original = content

    # 5. Update competitive advantage comparisons
    content = re.sub(
        r'AION-CR: 647 regulaciones, 25\+ jurisdicciones',
        'AION-CR: ~900-1,000 regulaciones, 90-100 jurisdicciones',
        content
    )
    if content != original:
        changes.append("Updated competitive comparison")
        original = content

    # 6. Update total LOC 624,024 → 639,409
    content = re.sub(r'624,024', '639,409', content)
    if content != original:
        changes.append("Updated total LOC")
        original = content

    # 7. Update AGI-AEF average 215.8 → 217.4
    content = re.sub(r'215\.8', '217.4', content)
    if content != original:
        changes.append("Updated AGI-AEF average")
        original = content

    # 8. Update OneTrust comparison to reflect advantage
    content = re.sub(
        r'5-6x más regulaciones que OneTrust',
        '7-8x más regulaciones que OneTrust, 4-5x más jurisdicciones',
        content
    )
    if content != original:
        changes.append("Updated OneTrust comparison")
        original = content

    # Write updated content
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

    return changes

if __name__ == '__main__':
    file_path = 'D:/Ectus-R/INVESTOR_FAQ.md'

    try:
        changes = update_faq_metrics(file_path)
        # Use UTF-8 for stdout
        sys.stdout.reconfigure(encoding='utf-8')
        print(f"Applied {len(changes)} updates to INVESTOR_FAQ.md:")
        for i, change in enumerate(changes, 1):
            print(f"  {i}. {change}")
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
