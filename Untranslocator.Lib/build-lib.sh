#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_PATH="$SCRIPT_DIR/Untranslocator.xcodeproj"
TARGET_NAME="Untranslocator"
CONFIGURATION="${CONFIGURATION:-Release}"
OUTPUT_ROOT="$SCRIPT_DIR/build"
OUTPUT_DIR="$OUTPUT_ROOT/output"
INCLUDE_DIR="$OUTPUT_DIR/include"

if [[ "${UNTRANSLOCATOR_ARCHS:-}" != "" ]]; then
  read -r -a ARCHS <<< "${UNTRANSLOCATOR_ARCHS}"
else
  ARCHS=("arm64" "x86_64")
fi

rm -rf "$OUTPUT_ROOT"
mkdir -p "$OUTPUT_DIR" "$INCLUDE_DIR"

LIB_INPUTS=()
for arch in "${ARCHS[@]}"; do
  ARCH_DIR="$OUTPUT_ROOT/$arch"
  ARCH_LIB="$ARCH_DIR/libUntranslocator.a"
  mkdir -p "$ARCH_DIR"

  xcodebuild \
    -project "$PROJECT_PATH" \
    -target "$TARGET_NAME" \
    -configuration "$CONFIGURATION" \
    -sdk macosx \
    -arch "$arch" \
    build \
    CONFIGURATION_BUILD_DIR="$ARCH_DIR" \
    > "$ARCH_DIR/build.log"

  if [[ ! -f "$ARCH_LIB" ]]; then
    echo "Expected output missing: $ARCH_LIB" >&2
    exit 1
  fi

  LIB_INPUTS+=("$ARCH_LIB")
done

OUTPUT_LIB="$OUTPUT_DIR/libUntranslocator.a"
if (( ${#LIB_INPUTS[@]} == 1 )); then
  cp "${LIB_INPUTS[0]}" "$OUTPUT_LIB"
else
  lipo -create "${LIB_INPUTS[@]}" -output "$OUTPUT_LIB"
fi

cp "$SCRIPT_DIR/Untranslocator/Untranslocator.h" "$INCLUDE_DIR/Untranslocator.h"
cp "$SCRIPT_DIR/Untranslocator/untranslocator_c.h" "$INCLUDE_DIR/untranslocator_c.h"

echo "Built static library: $OUTPUT_LIB"
echo "Exported headers in: $INCLUDE_DIR"
