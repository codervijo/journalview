#!/bin/bash
set -e

# Define output file name
BUILD_DATE="31.01.2025"
RELEASE_VERSION="v0.0.34"
OUTPUT_FILE="journalview-latest-${BUILD_DATE}.zip"
ARCH="x86-64"
OS="unknown-linux-musl"

# GitHub release download URL
DOWNLOAD_URL="https://github.com/codervijo/journalview/releases/download/${RELEASE_VERSION}/journalview-latest-${BUILD_DATE}.zip"

mkdir -p $HOME/.local/bin

# Download the binary
echo "Downloading JournalView from: $DOWNLOAD_URL"
curl -L "$DOWNLOAD_URL" -o "$OUTPUT_FILE"

# Extract the ZIP file
echo "Extracting $OUTPUT_FILE..."
unzip "$OUTPUT_FILE" -d ./journalview

# Make executable
chmod +x ./journalview/bin/journalview-latest-${ARCH}-${OS}
cp ./journalview/bin/journalview-latest-${ARCH}-${OS} ${HOME}/.local/bin/journalview
chmod +x $HOME/.local/bin/journalview

echo "Download complete. Run journalview to start JournalView."


