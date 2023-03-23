$ErrorActionPreference = "Stop"

if (Test-Path .\target) {
    Remove-Item .\target -Recurse -Force
}
if (Test-Path .\git-server-dashboard-windows.zip) {
    Remove-Item .\git-server-dashboard-windows.zip -Force
}

New-Item -ItemType Directory -Path .\target | Out-Null
Copy-Item .\server\target\release\git-server-dashboard.exe target\
Copy-Item .\ui\dist target\static -Recurse

Compress-Archive -Path target\* -DestinationPath git-server-dashboard-windows.zip