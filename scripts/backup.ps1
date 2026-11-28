$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $PSScriptRoot
$BackupDir = Join-Path $Root "backups"
$Timestamp = Get-Date -Format "yyyyMMdd_HHmmss"

$ZipPath = Join-Path $BackupDir "backup_$Timestamp.zip"
$TempDir = Join-Path $env:TEMP "my_project_backup_$Timestamp"

New-Item -ItemType Directory -Force -Path $BackupDir | Out-Null
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

try {
    $Files = Get-ChildItem -LiteralPath $Root -Recurse -File -Force |
        Where-Object {
            $Relative = $_.FullName.Substring($Root.Length).TrimStart('\')
            $Parts = $Relative -split '\\'

            $Parts -notcontains ".git" -and
            $Parts -notcontains ".vs" -and
            $Parts -notcontains "target" -and
            $Parts -notcontains "backups" -and
            $Parts -notcontains "logs" -and
            $Parts -notcontains "secrets" -and
            $_.Name -notmatch '^\.env' -and
            $_.Name -ne "private_key.pem" -and
            $_.Name -ne "license_modified.json" -and
            $_.Name -ne "main_backup.txt" -and
            $_.Extension -notin @(".enc", ".key", ".exe", ".tmp", ".bak") -and
            ($_.Extension -ne ".pem" -or $_.Name -eq "public_key.pem")
        }

    foreach ($File in $Files) {
        $Relative = $File.FullName.Substring($Root.Length).TrimStart('\')
        $Destination = Join-Path $TempDir $Relative
        $Parent = Split-Path -Parent $Destination

        New-Item -ItemType Directory -Force -Path $Parent | Out-Null
        Copy-Item -LiteralPath $File.FullName -Destination $Destination
    }

    Compress-Archive -Path "$TempDir\*" `
        -DestinationPath $ZipPath `
        -CompressionLevel Optimal

    $Hash = (Get-FileHash -LiteralPath $ZipPath -Algorithm SHA256).Hash

    Set-Content -Path "$ZipPath.sha256" `
        -Value "$Hash  $(Split-Path $ZipPath -Leaf)"

    Write-Host "BACKUP CREATED SUCCESSFULLY!" -ForegroundColor Green
    Write-Host "Archive: $ZipPath"
    Write-Host "SHA-256: $Hash"
}
finally {
    if (Test-Path $TempDir) {
        Remove-Item -LiteralPath $TempDir -Recurse -Force
    }
}