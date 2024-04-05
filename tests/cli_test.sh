#!/usr/bin/env bash
echo "Running tests for 'pestfmt --stdin'..."

RED='\033[0;31m'
GREEN='\033[0;32m'
NC="\033[0m"

# Trim start and end whitespace
trim() {
    sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//'
}

assert_eq() {
    local expected=$1
    local acctual=$2
    if [ "$acctual" != "$expected" ]; then
        echo ""
        echo "Test failed"
        echo "---------------------------------------------------"
        printf "%bExpected:%b\n\n%s\n" "${RED}" "${NC}" "$expected"
        echo ""
        printf "%bAcctual:%b\n\n%s\n" "${GREEN}" "${NC}" "$acctual"
        echo "---------------------------------------------------"
        exit 1
    fi
}

assert_format_stdin() {
    local input=$1
    local expected=$2

    # If input is a file, read it
    if [ -f "$input" ]; then
        input=$(cat "$input")
    fi
    if [ -f "$expected" ]; then
        expected=$(cat "$expected")
    fi

    # Perform pestfmt --stdin test
    local acctual
    acctual=$(cargo run . --stdin <<< "$input" | trim)
    assert_eq "$expected" "$acctual"
}

assert_format() {
    local input=$1
    local expected=$2
    local acctual
    acctual=$(cargo run . "$input" | trim)
    assert_eq "$acctual" "$expected"
}

assert_format_stdin 'item={"a"}' 'item = { "a" }'
assert_format_stdin "tests/fixtures/json.actual.pest" "tests/fixtures/json.expected.pest"
assert_format "tests/fixtures/pest.expected.pest" "Formatted 0 files"

echo "All CLI tests passed."
