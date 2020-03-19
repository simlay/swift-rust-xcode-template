#!/bin/sh

# Credit goes to https://stackoverflow.com/a/42602808

OLDNAME=swift-rust-xcode-template
NEWNAME={{project-name}}

export LC_CTYPE=C
export LANG=C
find . -type f ! -path ".*/.*" -exec sed -i '' -e "s/${OLDNAME}/${NEWNAME}/g" {} +

mv "${OLDNAME}.xcodeproj" "${NEWNAME}.xcodeproj"
mv "${OLDNAME}" "${NEWNAME}"
mv "${OLDNAME}Tests" "${NEWNAME}"
mv "${OLDNAME}UITests" "${NEWNAME}"
