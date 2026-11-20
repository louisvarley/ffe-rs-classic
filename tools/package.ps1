<#
.SYNOPSIS
  Stage and zip a playtest build: the two executables, the documents a tester
  needs, and a checksum list. No original game data, caches or music.

.DESCRIPTION
  Builds ffe-rs and ffe-launcher in release mode (unless -NoBuild), copies them
  with LICENSE, README.md, docs\CONTROLS.md and docs\LEGAL.md into
  dist\ffe-rs-<version>\, refuses to continue if anything that looks like a
  player-supplied or generated file ended up there, writes SHA256SUMS.txt and
  PACKAGE.txt, then zips the folder to dist\ffe-rs-<version>-win-x64.zip.

  The player puts an installed supported release in a `game` folder beside the
  executables (see PACKAGE.txt). The package carries the third-party notices
  (docs\THIRD_PARTY.md, THIRD_PARTY_LICENSES.txt, the sound bank's licence), but a public
  binary release also needs a relinking arrangement for the statically linked LGPL-2.1
  synthesiser, which is an open owner decision (burndown release-4).

.EXAMPLE
  .\tools\package.ps1
  .\tools\package.ps1 -NoBuild
#>
param(
    [switch]$NoBuild
)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
Set-Location $root

$manifest = Get-Content -LiteralPath (Join-Path $root 'crates\ffe-rs\Cargo.toml') -Raw
if ($manifest -notmatch '(?m)^version\s*=\s*"([^"]+)"') { throw 'Cannot read the ffe-rs version.' }
$version = $Matches[1]

if (-not $NoBuild) {
    cargo build --release --locked -p ffe-rs -p ffe-launcher
    if ($LASTEXITCODE -ne 0) { throw 'cargo build failed.' }
}

$name = "ffe-rs-$version"
$dist = Join-Path $root 'dist'
$stage = Join-Path $dist $name
$zip = Join-Path $dist "$name-win-x64.zip"
if (Test-Path -LiteralPath $stage) { Remove-Item -LiteralPath $stage -Recurse -Force }
if (Test-Path -LiteralPath $zip) { Remove-Item -LiteralPath $zip -Force }
New-Item -ItemType Directory -Force -Path (Join-Path $stage 'docs') | Out-Null

$files = @(
    @{ From = 'target\release\ffe-rs.exe';       To = 'ffe-rs.exe' },
    @{ From = 'target\release\ffe-launcher.exe'; To = 'ffe-launcher.exe' },
    @{ From = 'LICENSE';                         To = 'LICENSE' },
    @{ From = 'README.md';                       To = 'README.md' },
    @{ From = 'docs\CONTROLS.md';                To = 'docs\CONTROLS.md' },
    @{ From = 'docs\LEGAL.md';                   To = 'docs\LEGAL.md' },
    @{ From = 'docs\THIRD_PARTY.md';             To = 'docs\THIRD_PARTY.md' },
    @{ From = 'crates\ffe-audio\assets\soundfont\LICENSE'; To = 'GeneralUser-GS-LICENSE.txt' }
)
foreach ($file in $files) {
    $from = Join-Path $root $file.From
    if (-not (Test-Path -LiteralPath $from)) { throw "Missing $($file.From); build first or check the path." }
    Copy-Item -LiteralPath $from -Destination (Join-Path $stage $file.To)
}

# Third-party licences: the inventory must be current and acceptable, and the package
# carries every linked crate's licence and notice files.
python (Join-Path $root 'tools\third-party.py') --check
if ($LASTEXITCODE -ne 0) { throw 'Third-party licence check failed (see above).' }
python (Join-Path $root 'tools\third-party.py') --texts (Join-Path $stage 'THIRD_PARTY_LICENSES.txt')
if ($LASTEXITCODE -ne 0) { throw 'Could not write THIRD_PARTY_LICENSES.txt.' }

# Nothing the player supplies or the game generates may ship (CLAUDE.MD section 1).
$forbidden = '(?i)(^|\\)(game|FMVs|saves|recordings)(\\|$)|\.(hmp|bmp|avi|raw|sf2)$|^(FIRSTENC\.EXE|MISSION\.DAT|data\.bin|music\.bin|assets\.bin)$'
$bad = Get-ChildItem -LiteralPath $stage -Recurse -File |
    ForEach-Object { $_.FullName.Substring($stage.Length + 1) } |
    Where-Object { $_ -match $forbidden }
if ($bad) { throw "Refusing to package player data or generated files: $($bad -join ', ')" }

@"
ffe-rs $version (Windows x64 playtest build)

1. Put an installed, supported First Encounters release in a folder named
   "game" beside ffe-rs.exe (floppy English 1.02 or CD English 1.01). Do not put
   only the BIN/CUE image or a patch installer there.
2. Run ffe-launcher.exe (it checks the folder and prepares the music pack on
   first run) or ffe-rs.exe directly. Run ffe-rs.exe --help for the options.
3. Controls: docs\CONTROLS.md.

This package contains no original game data. Report only the build ID
(ffe-rs.exe --version), the test number, pass or fail, and short steps.
"@ | Set-Content -LiteralPath (Join-Path $stage 'PACKAGE.txt') -Encoding utf8

Get-ChildItem -LiteralPath $stage -Recurse -File |
    Where-Object { $_.Name -ne 'SHA256SUMS.txt' } |
    ForEach-Object {
        $hash = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
        "$hash  $($_.FullName.Substring($stage.Length + 1) -replace '\\', '/')"
    } | Set-Content -LiteralPath (Join-Path $stage 'SHA256SUMS.txt') -Encoding utf8

Compress-Archive -LiteralPath $stage -DestinationPath $zip
Write-Host "Packaged $zip"
Get-Content -LiteralPath (Join-Path $stage 'SHA256SUMS.txt')
