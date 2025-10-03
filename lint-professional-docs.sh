#!/bin/bash
# Professional Documentation Linter
# Version: 1.0
# Date: 2025-10-03

FILE=$1

if [ -z "$FILE" ]; then
    echo "Usage: ./lint-professional-docs.sh <markdown-file>"
    exit 1
fi

if [ ! -f "$FILE" ]; then
    echo "Error: File '$FILE' not found"
    exit 1
fi

echo "=========================================="
echo "  Professional Documentation Linter"
echo "=========================================="
echo "File: $FILE"
echo "Date: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

ERRORS=0
WARNINGS=0

# 1. Check for emojis (Unicode ranges)
echo "[1/8] Checking for emojis..."
EMOJI_LINES=$(grep -n '[😀-🙏🌀-🗿🚀-🛿🇦-🇿]' "$FILE" 2>/dev/null)
if [ -n "$EMOJI_LINES" ]; then
    echo "❌ FAIL: Emojis found:"
    echo "$EMOJI_LINES"
    ((ERRORS++))
else
    echo "✅ PASS: No emojis detected"
fi
echo ""

# 2. Check for superlatives and exaggerations
echo "[2/8] Checking for superlatives/exaggerations..."
SUPERLATIVES=(
    "mejor del mundo"
    "revolucionario"
    "sin precedentes"
    "absolutamente"
    "increíble"
    "asombroso"
    "jamás"
    "clase mundial"
    "imposible de replicar"
    "líder absoluto"
    "dominancia total"
    "perfecto"
    "único en el mundo"
    "nunca antes visto"
    "excepcional"
    "extraordinario"
    "fenomenal"
    "espectacular"
)

for term in "${SUPERLATIVES[@]}"; do
    LINES=$(grep -in "$term" "$FILE" 2>/dev/null)
    if [ -n "$LINES" ]; then
        echo "⚠️  WARNING: Superlative '$term' found:"
        echo "$LINES"
        ((WARNINGS++))
    fi
done
echo "Superlatives check complete"
echo ""

# 3. Check for unsourced quantitative claims
echo "[3/8] Checking for potentially unsourced claims..."
CLAIMS=$(grep -E '[0-9]+%|[0-9]+x|\$[0-9]+[MBK]|ARR|TAM|ROI' "$FILE" | grep -v 'fuente\|basado en\|según\|benchmark\|estimado\|proyectado\|comparables' 2>/dev/null | head -10)
if [ -n "$CLAIMS" ]; then
    echo "⚠️  WARNING: Potential unsourced claims (verify these have supporting data):"
    echo "$CLAIMS"
    ((WARNINGS++))
else
    echo "✅ PASS: Quantitative claims appear sourced"
fi
echo ""

# 4. Check for ambiguous technical terms
echo "[4/8] Checking for ambiguous technical terms..."
AMBIGUOUS_TERMS=(
    "Quantum ML"
    "IA avanzada"
    "auto-optimización"
    "aprendizaje profundo"
    "clase mundial"
    "flagship"
    "hectocorn"
)

for term in "${AMBIGUOUS_TERMS[@]}"; do
    LINES=$(grep -in "$term" "$FILE" 2>/dev/null)
    if [ -n "$LINES" ]; then
        echo "⚠️  WARNING: Ambiguous term '$term' - consider clarifying:"
        echo "$LINES"
        ((WARNINGS++))
    fi
done
echo ""

# 5. Check for unexpanded acronyms (first 100 lines for performance)
echo "[5/8] Checking for unexpanded acronyms (sample)..."
ACRONYMS=("AGI-AEF" "TAM" "SAM" "ARR" "MRR" "ARPU" "LTV" "CAC" "FIPS" "NIST" "ML-KEM" "ML-DSA" "RBAC" "SOC2" "GDPR")

for acronym in "${ACRONYMS[@]}"; do
    FIRST_MENTION=$(grep -n "$acronym" "$FILE" | head -1)
    if [ -n "$FIRST_MENTION" ]; then
        LINE_NUM=$(echo "$FIRST_MENTION" | cut -d':' -f1)
        LINE_CONTENT=$(sed -n "${LINE_NUM}p" "$FILE")
        # Check if line contains expansion (parentheses)
        if ! echo "$LINE_CONTENT" | grep -q '(.*'"$acronym"'\|'"$acronym"'.*(' ; then
            echo "⚠️  WARNING: Acronym '$acronym' at line $LINE_NUM may not be expanded on first use"
            ((WARNINGS++))
        fi
    fi
done
echo ""

# 6. Check for very long sentences (>30 words)
echo "[6/8] Checking for overly long sentences..."
LONG_SENTENCES=0
while IFS= read -r line; do
    # Count words in line
    WORD_COUNT=$(echo "$line" | wc -w)
    if [ "$WORD_COUNT" -gt 30 ]; then
        echo "⚠️  WARNING: Sentence with $WORD_COUNT words (>30): ${line:0:80}..."
        ((LONG_SENTENCES++))
        ((WARNINGS++))
    fi
done < "$FILE"
if [ $LONG_SENTENCES -eq 0 ]; then
    echo "✅ PASS: No overly long sentences"
fi
echo ""

# 7. Check for marketing terms without quantification
echo "[7/8] Checking for unquantified competitive claims..."
MARKETING_TERMS=(
    "mejor que"
    "superior a"
    "supera a"
    "líder en"
    "domina"
    "único"
)

for term in "${MARKETING_TERMS[@]}"; do
    LINES=$(grep -in "$term" "$FILE" | grep -v '[0-9]x\|[0-9]%\|vs\.\|comparado' | head -5)
    if [ -n "$LINES" ]; then
        echo "⚠️  WARNING: Competitive claim '$term' without quantification:"
        echo "$LINES"
        ((WARNINGS++))
    fi
done
echo ""

# 8. Check for passive voice (sample check)
echo "[8/8] Checking for excessive passive voice (sample)..."
PASSIVE_COUNT=$(grep -c 'es considerado\|ha sido\|fue desarrollado\|son utilizados\|está siendo' "$FILE" 2>/dev/null)
if [ "$PASSIVE_COUNT" -gt 10 ]; then
    echo "⚠️  WARNING: High passive voice count ($PASSIVE_COUNT instances) - consider active voice"
    ((WARNINGS++))
else
    echo "✅ PASS: Passive voice usage acceptable"
fi
echo ""

# Final summary
echo "=========================================="
echo "  Linting Summary"
echo "=========================================="
echo "Errors:   $ERRORS"
echo "Warnings: $WARNINGS"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo "❌ FAIL: Document has $ERRORS critical errors"
    echo "Action required: Fix errors before publication"
    exit 1
elif [ $WARNINGS -gt 10 ]; then
    echo "⚠️  REVIEW NEEDED: $WARNINGS warnings detected"
    echo "Recommendation: Review and address warnings"
    exit 0
else
    echo "✅ PASS: Document meets professional standards"
    exit 0
fi
