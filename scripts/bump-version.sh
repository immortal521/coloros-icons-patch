#!/usr/bin/env bash
set -x

# 参数: patch | minor | major | uxicons[-patch|-minor|-major]
BUMP=$1
if [[ -z "$BUMP" ]]; then
  echo "Usage: $0 [patch|minor|major|uxicons[-patch|-minor|-major]]"
  exit 1
fi

# 主版本 bump
if [[ "$BUMP" != uxicons* ]]; then
  VERSION=$(<VERSION)
  IFS='.' read -r MAJOR MINOR PATCH <<<"${VERSION//[[:space:]]/}"
  MAJOR=${MAJOR:-0}
  MINOR=${MINOR:-0}
  PATCH=${PATCH:-0}

  case $BUMP in
  major)
    ((MAJOR++))
    MINOR=0
    PATCH=0
    ;;
  minor)
    ((MINOR++))
    PATCH=0
    ;;
  patch) ((PATCH++)) ;;
  esac

  NEW_VERSION="$MAJOR.$MINOR.$PATCH"
  echo "$NEW_VERSION" >VERSION
  echo "Bumped main version to $NEW_VERSION"

  # 更新 module.prop
  sed -i "s/^version=.*/version=$NEW_VERSION/" module/module.prop
  sed -i "s/^versionCode=.*/versionCode=$PATCH/" module/module.prop

  # 更新 package.json
  jq ".version=\"$NEW_VERSION\"" webui/package.json >webui/package.tmp.json
  mv webui/package.tmp.json webui/package.json

  # 更新 Cargo.toml
  CARGO_TOML="cip/Cargo.toml"
  sed -i -E "s/^version\s*=\s*\"[^\"]+\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"

  git add VERSION module/module.prop webui/package.json "$CARGO_TOML"
  git commit -m "Bump version to $NEW_VERSION"

  if ! git rev-parse "v$NEW_VERSION" >/dev/null 2>&1; then
    git tag -a "v$NEW_VERSION" -m "Release $NEW_VERSION"
  else
    echo "Tag v$NEW_VERSION already exists, skipping tag creation."
  fi
fi

# uxicons bump
if [[ "$BUMP" == uxicons* ]]; then
  UX_VERSION=$(<uxicons/VERSION)
  IFS='.' read -r MAJOR MINOR PATCH <<<"${UX_VERSION//[[:space:]]/}"
  MAJOR=${MAJOR:-0}
  MINOR=${MINOR:-0}
  PATCH=${PATCH:-0}

  case $BUMP in
  uxicons | uxicons-patch) ((PATCH++)) ;;
  uxicons-minor)
    ((MINOR++))
    PATCH=0
    ;;
  uxicons-major)
    ((MAJOR++))
    MINOR=0
    PATCH=0
    ;;
  esac

  NEW_UXVERSION="$MAJOR.$MINOR.$PATCH"
  echo "$NEW_UXVERSION" | tee uxicons/VERSION
  echo "Bumped uxicons version to $NEW_UXVERSION"

  # 更新锁文件
  echo "uxicons=$NEW_UXVERSION" | tee uxicons.lock

  git add uxicons/VERSION uxicons.lock
  git commit -m "Bump uxicons to $NEW_UXVERSION"

  if ! git rev-parse "uxicons/v$NEW_UXVERSION" >/dev/null 2>&1; then
    git tag -a "uxicons/v$NEW_UXVERSION" -m "Release uxicons $NEW_UXVERSION"
  else
    echo "Tag uxicons/v$NEW_UXVERSION already exists, skipping tag creation."
  fi
fi
