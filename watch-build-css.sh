#!/bin/bash

find src -type f -name '*.rs' | entr sh -c 'cargo run --bin build_html && pushd static/css/tailwindcss; yarn build; popd'
