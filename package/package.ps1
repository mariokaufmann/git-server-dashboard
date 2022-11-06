$ErrorActionPreference = "Stop"

if (Test-Path .\target) {
    Remove-Item .\target -Recurse -Force
}
if (Test-Path .\branch-dashboard-windows.zip) {
    Remove-Item .\branch-dashboard-windows.zip -Force
}

New-Item -ItemType Directory -Path .\target | Out-Null
Copy-Item .\server\target\release\branch-dashboard.exe target\
Copy-Item .\ui\dist target\static -Recurse

Compress-Archive -Path target\* -DestinationPath branch-dashboard-windows.zip