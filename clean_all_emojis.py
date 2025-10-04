#!/usr/bin/env python3
"""Remove all emojis from markdown files in Ectus-R"""

import re
import os
import sys

def remove_emojis(text):
    """Remove all emojis from text"""
    # Extended emoji pattern
    emoji_pattern = re.compile("["
        u"\U0001F600-\U0001F64F"  # emoticons
        u"\U0001F300-\U0001F5FF"  # symbols & pictographs
        u"\U0001F680-\U0001F6FF"  # transport & map symbols
        u"\U0001F1E0-\U0001F1FF"  # flags (iOS)
        u"\U00002700-\U000027BF"  # Dingbats
        u"\U0001F900-\U0001F9FF"  # Supplemental Symbols and Pictographs
        u"\U0001FA70-\U0001FAFF"  # Symbols and Pictographs Extended-A
        u"\U00002600-\U000026FF"  # Miscellaneous Symbols
        u"\U0001F004"  # Mahjong Tile Red Dragon
        "]+", flags=re.UNICODE)

    return emoji_pattern.sub('', text)

def clean_file(filepath):
    """Clean emojis from a file"""
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
        print(f"Error: {filepath}: {e}", file=sys.stderr)
        return False

def main():
    os.chdir('D:/Ectus-R')

    # Get all .md files in root
    md_files = [f for f in os.listdir('.') if f.endswith('.md')]

    cleaned = []
    for filename in md_files:
        if clean_file(filename):
            cleaned.append(filename)

    if cleaned:
        sys.stdout.reconfigure(encoding='utf-8')
        print(f"Cleaned {len(cleaned)} files:")
        for f in sorted(cleaned):
            print(f"  - {f}")
    else:
        print("No emojis found")

if __name__ == '__main__':
    main()
