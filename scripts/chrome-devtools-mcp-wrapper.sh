#!/usr/bin/env nix-shell
#! nix-shell -E "let pkgs = import (builtins.fetchTarball { url = \"https://github.com/nixos/nixpkgs/tarball/25.11\"; sha256 = \"1zn1lsafn62sz6azx6j735fh4vwwghj8cc9x91g5sx2nrg23ap9k\"; }) {}; chrome-devtools-mcp = pkgs.stdenv.mkDerivation { pname = \"chrome-devtools-mcp\"; version = \"0.20.3\"; src = pkgs.fetchurl { url = \"https://registry.npmjs.org/chrome-devtools-mcp/-/chrome-devtools-mcp-0.20.3.tgz\"; sha256 = \"1vz22g7cddwnd594la3i6v23hx90dbm2khabx7ncqsmnb9v5nzv1\"; }; sourceRoot = \"package\"; dontBuild = true; installPhase = ''mkdir -p $out/lib/chrome-devtools-mcp $out/bin; cp -r . $out/lib/chrome-devtools-mcp/; echo \"#!/bin/sh\" > $out/bin/chrome-devtools-mcp; echo \"exec ${pkgs.nodejs_22}/bin/node $out/lib/chrome-devtools-mcp/build/src/bin/chrome-devtools-mcp.js \\\"\\$@\\\"\" >> $out/bin/chrome-devtools-mcp; chmod +x $out/bin/chrome-devtools-mcp''; }; in pkgs.mkShell { buildInputs = [ chrome-devtools-mcp ]; }"
#! nix-shell -i bash
# shellcheck shell=bash
# Connects to an existing Brave session with remote debugging on port 9222.
# Does NOT launch a new browser.

set -euo pipefail

exec chrome-devtools-mcp \
  --no-usage-statistics \
  --no-performance-crux \
  --browserUrl http://127.0.0.1:9222
