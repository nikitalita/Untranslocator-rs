#!/usr/bin/env python3

import pathlib
import re
import sys


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("Usage: sync-cargo-version.py <ref-name>")

    ref_name = sys.argv[1]
    match = re.fullmatch(
        r"v?([0-9]+)\.([0-9]+)\.([0-9]+)(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?",
        ref_name,
    )
    if match is None:
        raise SystemExit(
            f"Tag '{ref_name}' is not a valid semver tag prefixed with v (example: v1.2.3)."
        )

    version = ref_name.removeprefix("v")
    cargo_toml = pathlib.Path("Cargo.toml")
    content = cargo_toml.read_text(encoding="utf-8")

    updated, count = re.subn(
        r'(?m)^version = ".*"$',
        f'version = "{version}"',
        content,
        count=1,
    )
    if count != 1:
        raise SystemExit("Could not update version in Cargo.toml")

    _ = cargo_toml.write_text(updated, encoding="utf-8")


if __name__ == "__main__":
    main()
