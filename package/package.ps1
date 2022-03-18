$ErrorActionPreference = "Stop"

if (Test-Path .\target) {
    Remove-Item .\target -Recurse -Force
}
if (Test-Path .\gitlab-branch-dashboard-windows.zip) {
    Remove-Item .\gitlab-branch-dashboard-windows.zip -Force
}

New-Item -ItemType Directory -Path .\target | Out-Null
Copy-Item .\server\target\release\gitlab-branch-dashboard.exe target\
Copy-Item .\ui\dist target\static -Recurse