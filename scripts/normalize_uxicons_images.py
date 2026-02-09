#!/usr/bin/env python3
import argparse
from pathlib import Path
from PIL import Image


def normalize_png(path: Path, dry_run: bool, verbose: bool) -> bool:
    """
    Return True if would change / changed.
    """
    if not path.exists() or not path.is_file():
        if verbose:
            print(f"skip (missing): {path}")
        return False

    if path.suffix.lower() != ".png":
        if verbose:
            print(f"skip (not png): {path}")
        return False

    try:
        with Image.open(path) as im:
            # Ensure RGBA
            rgba = im.convert("RGBA")

            if dry_run:
                would_change = (im.mode != "RGBA") or True
                if verbose:
                    print(f"would normalize: {path} (mode {im.mode} -> RGBA)")
                return would_change

            tmp = path.with_suffix(".tmp.png")
            rgba.save(tmp, format="PNG", optimize=True)
            tmp.replace(path)

            if verbose:
                print(f"normalized: {path}")
            return True

    except Exception as e:
        print(f"ERROR: {path}: {e}")
        raise


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true", help="Do not modify, just report")
    ap.add_argument("--verbose", action="store_true")
    ap.add_argument("files", nargs="*", help="Files to process (relative or absolute)")
    args = ap.parse_args()

    changed = False
    if not args.files:
        if args.verbose:
            print("no files provided, nothing to do")
        return 0

    for f in args.files:
        p = Path(f)
        p = p.resolve() if p.is_absolute() else (Path.cwd() / p)
        did = normalize_png(p, args.dry_run, args.verbose)
        changed = changed or did

    if args.dry_run and changed:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
