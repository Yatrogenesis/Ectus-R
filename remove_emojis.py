#!/usr/bin/env python3
"""Remove emojis from markdown and JSON files"""

import re
import os
import sys

def remove_emojis(text):
    """Remove common emojis used in documentation"""
    emoji_pattern = r'[🤖✅📝🚀🔍⚠️💡🎯📊🔧🐛✨🔥💻📈🌟⭐🎉🏆📌🔑💼🌐🛡️⚡🎨🔬🧪🏗️📦🔄🌍🌎🌏]'
    return re.sub(emoji_pattern, '', text)

def process_file(filepath):
    """Process a single file to remove emojis"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            original = f.read()

        cleaned = remove_emojis(original)

        if original != cleaned:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(cleaned)
            return True
        return False
    except Exception as e:
        print(f"Error processing {filepath}: {e}", file=sys.stderr)
        return False

def main():
    # Root markdown files to process
    root_files = [
        'EXECUTIVE_REPORT_C_SUITE.md',
        'INVESTOR_FAQ.md',
        'AUDIT_COMPARISON_ANALYSIS.md',
        'SESSION_SUMMARY_2025-10-03_FINAL.md',
        'QUANTUM_ML_VERIFICATION_FINAL.md',
        'README.md'
    ]

    os.chdir('D:/Ectus-R')

    processed = []
    for filename in root_files:
        if os.path.exists(filename):
            if process_file(filename):
                processed.append(filename)

    if processed:
        print(f"Removed emojis from {len(processed)} files:")
        for f in processed:
            print(f"  - {f}")
    else:
        print("No emojis found in specified files")

if __name__ == '__main__':
    main()
