#!/bin/sh

TEMP=$(mktemp -d)
# terminal colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

cleanup() {
    rm -rf "$TEMP"
    rm -rf "$DIFF"
}

cargo b --release
EXECUTABLE=$(find target/release -type f -executable -name "uwulang")

cargo t

$EXECUTABLE tests/cases/hellOwOrld.uwu > $TEMP/hellOwOrld.out
$EXECUTABLE tests/cases/sqwuare10000.uwu > $TEMP/sqwuare10000.out

diff tests/cases/hellOwOrld.out $TEMP > $TEMP/diff.out
diff tests/cases/sqwuare10000.out $TEMP >> $TEMP/diff.out


if [ -s $TEMP/diff.out ]; then
    echo -e "${RED}Tests failed${NC}"
    echo "Showing diff output below"
    cat $TEMP/diff.out
    cleanup
    exit 1
else
    echo -e "${GREEN}Tests passed${NC}"
    cleanup
    exit 0
fi
