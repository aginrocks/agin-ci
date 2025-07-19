#!/bin/bash
openapi-generator generate -i http://localhost:8080/apidoc/openapi.json -g rust -o api_client --skip-validate-spec
sed -i '' $'1i\\\n#![allow(warnings)]\n' api_client/src/lib.rs
