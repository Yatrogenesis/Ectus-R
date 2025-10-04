#!/usr/bin/env python3
"""Update AION-CR metrics in EXECUTIVE_REPORT_C_SUITE.md"""

import re

def update_metrics(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Track changes
    changes = []

    # 1. Update score 241.5 → 245-248
    original = content
    content = re.sub(
        r'Score: \*\*241\.5/255\*\*',
        'Score: **245-248/255**',
        content
    )
    if content != original:
        changes.append("Updated score 241.5 → 245-248")
        original = content

    # 2. Update LOC 187,471 → 202,856
    content = re.sub(
        r'187,471 LOC',
        '202,856 LOC',
        content
    )
    if content != original:
        changes.append("Updated LOC 187,471 → 202,856")
        original = content

    # 3. Update jurisdictions 25+ → 90-100
    content = re.sub(
        r'25\+ jurisdicciones',
        '90-100 jurisdicciones',
        content
    )
    if content != original:
        changes.append("Updated jurisdictions 25+ → 90-100")
        original = content

    # 4. Update regulations 647 → ~900-1,000
    content = re.sub(
        r'647 regulaciones',
        '~900-1,000 regulaciones',
        content
    )
    if content != original:
        changes.append("Updated regulations 647 → ~900-1,000")
        original = content

    # 5. Update competitive advantage 5-10x → 7-8x
    content = re.sub(
        r'5-10x mayor cobertura',
        '7-8x mayor cobertura regulatoria, 4-5x más jurisdicciones',
        content
    )
    if content != original:
        changes.append("Updated competitive advantage")
        original = content

    # 6. Update total LOC calculation
    # 294,187 (AION-R) + 142,366 (Ectus-R) + 202,856 (AION-CR) = 639,409
    content = re.sub(
        r'\*\*LOC Combinadas\*\* \| 624,024',
        '**LOC Combinadas** | 639,409',
        content
    )
    if content != original:
        changes.append("Updated total LOC 624,024 → 639,409")
        original = content

    # 7. Update IP valuation based on new LOC
    # 639,409 LOC × $50-100/LOC = $31.9M-$63.9M
    content = re.sub(
        r'Activo de IP valorado en \$31-62M',
        'Activo de IP valorado en $32-64M',
        content
    )
    if content != original:
        changes.append("Updated IP valuation")
        original = content

    # 8. Update AGI-AEF average score
    # (232.8 + 173.0 + 246.5) / 3 = 217.4
    content = re.sub(
        r'\*\*Score AGI-AEF Promedio\*\* \| 215\.8/255',
        '**Score AGI-AEF Promedio** | 217.4/255',
        content
    )
    if content != original:
        changes.append("Updated avg AGI-AEF score 215.8 → 217.4")
        original = content

    # 9. Update score explanations
    content = re.sub(
        r'241\.5/255 está a solo \*\*13\.5 puntos del máximo teórico\*\*',
        '245-248/255 está a solo **7-10 puntos del máximo teórico**',
        content
    )
    if content != original:
        changes.append("Updated score distance to max")
        original = content

    # 10. Update moat complexity calculation
    content = re.sub(
        r'Replicar 647 regulaciones × 25 jurisdicciones',
        'Replicar ~900-1,000 regulaciones × 90-100 jurisdicciones',
        content
    )
    if content != original:
        changes.append("Updated moat calculation")
        original = content

    # 11. Update legal review count
    content = re.sub(
        r'Legal review de 647 regulaciones',
        'Legal review de ~900-1,000 regulaciones',
        content
    )
    if content != original:
        changes.append("Updated legal review count")
        original = content

    # 12. Update conclusion statement
    content = re.sub(
        r'624,024 líneas de código Rust',
        '639,409 líneas de código Rust',
        content
    )
    if content != original:
        changes.append("Updated conclusion LOC")
        original = content

    content = re.sub(
        r'AION-CR score de 241\.5/255, base de datos de 647 regulaciones',
        'AION-CR score de 245-248/255, base de datos de ~900-1,000 regulaciones en 90-100 jurisdicciones',
        content
    )
    if content != original:
        changes.append("Updated final statement")
        original = content

    # 13. Update TOTAL score line
    content = re.sub(
        r'\*\*TOTAL: 241\.5/255 \(94\.7%\)',
        '**TOTAL: 245-248/255 (96.1-97.3%)',
        content
    )
    if content != original:
        changes.append("Updated TOTAL score percentage")
        original = content

    # Write updated content
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

    return changes

if __name__ == '__main__':
    file_path = 'D:/Ectus-R/EXECUTIVE_REPORT_C_SUITE.md'
    changes = update_metrics(file_path)

    print(f"Applied {len(changes)} updates to EXECUTIVE_REPORT_C_SUITE.md:")
    for i, change in enumerate(changes, 1):
        print(f"  {i}. {change}")
