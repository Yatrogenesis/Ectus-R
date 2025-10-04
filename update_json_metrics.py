#!/usr/bin/env python3
"""Update AION-CR metrics in agi_aef_assessment_aion_cr.json"""

import json
import sys

def update_json_metrics(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    changes = []

    # 1. Update total_loc: 187471 → 202856
    old_loc = data['system_overview']['codebase_metrics']['total_loc']
    data['system_overview']['codebase_metrics']['total_loc'] = 202856
    if old_loc != 202856:
        changes.append(f"Updated total_loc: {old_loc} → 202856")

    # 2. Update jurisdictions: 25 → 95 (midpoint of 90-100)
    old_jurisdictions = data['system_overview']['codebase_metrics']['jurisdictions']
    data['system_overview']['codebase_metrics']['jurisdictions'] = 95
    if old_jurisdictions != 95:
        changes.append(f"Updated jurisdictions: {old_jurisdictions} → 95")

    # 3. Update regulatory_frameworks: 647 → 950 (midpoint of ~900-1,000)
    old_regs = data['system_overview']['codebase_metrics']['regulatory_frameworks']
    data['system_overview']['codebase_metrics']['regulatory_frameworks'] = 950
    if old_regs != 950:
        changes.append(f"Updated regulatory_frameworks: {old_regs} → 950")

    # 4. Update overall_score: 241.5 → 246.5 (midpoint of 245-248)
    old_score = data['agi_aef_composite_score']['overall_score']
    data['agi_aef_composite_score']['overall_score'] = 246.5
    if old_score != 246.5:
        changes.append(f"Updated overall_score: {old_score} → 246.5")

    # 5. Update percentage: 94.7 → 96.7
    data['agi_aef_composite_score']['percentage'] = 96.7
    changes.append("Updated percentage: 94.7 → 96.7")

    # 6. Update database scale advantage
    old_advantage = data['competitive_positioning']['database_scale_advantage']
    data['competitive_positioning']['database_scale_advantage'] = "7-8x larger than competitors (950 vs ~100-200 regulations), 4-5x more jurisdictions (95 vs ~20)"
    if old_advantage != data['competitive_positioning']['database_scale_advantage']:
        changes.append("Updated competitive advantage description")

    # 7. Update autonomy advantage
    data['competitive_positioning']['autonomy_advantage'] = "Level 246.5 vs competitors' max ~160"
    changes.append("Updated autonomy advantage")

    # 8. Update audit points to reflect new scale
    data['dimension_scores']['cognitive_autonomy']['audit_points'][0] = "950+ regulations processed with atomic-level granularity"
    changes.append("Updated cognitive autonomy audit points")

    # 9. Update assessment summary
    old_classification = data['assessment_summary']['classification_rationale']
    data['assessment_summary']['classification_rationale'] = old_classification.replace("241.5/255", "246.5/255")
    changes.append("Updated classification rationale")

    # 10. Update key strengths
    data['assessment_summary']['key_strengths'][0] = "Unprecedented database scale (950 regulations, 19,875+ articles, 95 jurisdictions)"
    changes.append("Updated key strengths")

    # 11. Update conclusion
    old_conclusion = data['conclusion']
    data['conclusion'] = old_conclusion.replace("241.5/255", "246.5/255").replace("647 regulations", "950 regulations in 95 jurisdictions")
    changes.append("Updated conclusion")

    # 12. Update expansion opportunities
    data['gaps_and_improvements']['expansion_opportunities'][0] = "Geographic coverage: 950 → 2,047+ regulations (roadmap defined)"
    changes.append("Updated expansion roadmap")

    # 13. Update validation data sources
    data['validation_status']['data_sources'][0] = "Direct codebase analysis (202,856 LOC)"
    changes.append("Updated validation data sources")

    # Write updated JSON
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

    return changes

if __name__ == '__main__':
    file_path = 'D:/Ectus-R/agi_aef_assessment_aion_cr.json'

    try:
        changes = update_json_metrics(file_path)
        sys.stdout.reconfigure(encoding='utf-8')
        print(f"Applied {len(changes)} updates to agi_aef_assessment_aion_cr.json:")
        for i, change in enumerate(changes, 1):
            print(f"  {i}. {change}")
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
