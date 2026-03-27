#!/usr/bin/env node

const { execSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const platform = process.platform;
const arch = process.arch;

const PLATFORMS = {
  "linux-x64": "geneser-linux-x64",
  "darwin-x64": "geneser-darwin-x64",
  "darwin-arm64": "geneser-darwin-arm64",
  "win32-x64": "geneser-windows-x64",
};

const key = `${platform}-${arch}`;
const pkgName = PLATFORMS[key];

if (!pkgName) {
  console.error(`geneser: unsupported platform ${key}`);
  process.exit(1);
}

let pkgDir;
try {
  pkgDir = path.dirname(require.resolve(`${pkgName}/package.json`));
} catch (e) {
  console.error(`geneser: could not find package ${pkgName}. It may not be installed.`);
  process.exit(1);
}

const binaryName = platform === "win32" ? "geneser.exe" : "geneser";
const src = path.join(pkgDir, binaryName);
const binDir = path.join(__dirname, "bin");
const dest = path.join(binDir, binaryName);

if (!fs.existsSync(src)) {
  console.error(`geneser: binary not found at ${src}`);
  process.exit(1);
}

fs.mkdirSync(binDir, { recursive: true });
fs.copyFileSync(src, dest);
fs.chmodSync(dest, 0o755);

// Create wrapper script for unix (so `geneser` bin entry works regardless of .exe)
if (platform !== "win32") {
  const wrapper = path.join(binDir, "geneser");
  if (wrapper !== dest) {
    // dest is already 'geneser' on unix, nothing extra needed
  }
} else {
  // On windows, write a cmd wrapper as the bin entry
  const cmdWrapper = path.join(binDir, "geneser");
  fs.writeFileSync(cmdWrapper, `@echo off\r\n"%~dp0geneser.exe" %*\r\n`);
}

console.log(`geneser: installed binary for ${key}`);
