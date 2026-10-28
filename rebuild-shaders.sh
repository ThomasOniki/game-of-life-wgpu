#!/bin/sh

dir="$(dirname "$(readlink -f -- "$0")")/shaders"

for file in "compute" "graphics"; do
	slangc "$dir/$file.slang" -o "$dir/$file.wgsl"
done
