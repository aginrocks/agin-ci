#!/bin/bash
openapi-generator generate -i http://localhost:8080/apidoc/openapi.json -g rust -o api_client --skip-validate-spec
ex -s -c '1i|#![allow(warnings)]' -c x api_client/src/lib.rs
