#!/bin/bash
set -e

# GitHub release download URL
DOWNLOAD_URL="https://github.com/codervijo/journalview/releases/download/v0.0.28/journalview-latest-31.01.2025.zip"

# Define output file name
OUTPUT_FILE="journalview-latest-31.01.2025.zip"
ARCH="x86-64"
OS="unknown-linux-musl"

# Download the binary
echo "Downloading JournalView from: $DOWNLOAD_URL"
curl -L "$DOWNLOAD_URL" -o "$OUTPUT_FILE"

# Extract the ZIP file
echo "Extracting $OUTPUT_FILE..."
unzip "$OUTPUT_FILE" -d ./journalview

# Make executable
chmod +x ./journalview/bin/journalview-latest-${ARCH}-${OS}

echo "Download complete. Run ./bin/journalview-latest-${ARCH}-${OS} to start JournalView."


