#!/bin/bash

find . -type f -name '*.rs' -or -name '*.css' | entr sh -c 'cargo run --bin build_html && pushd static/css/tailwindcss; yarn build; popd'

