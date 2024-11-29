$Extension = ".aoe2record"
$ProgId = "CaptureAgeLocal"
$captureAgePath = "$env:LOCALAPPDATA\Programs\CaptureAge\CaptureAge.exe"
$progCommand = "`"$captureAgePath`" `"captureage://spectate?aoe2record=%1`""
$keyPath = "HKEY_CURRENT_USER\SOFTWARE\Classes\$Extension\OpenWithProgids"
[Microsoft.Win32.Registry]::SetValue( $keyPath, $ProgId, ([byte[]]@()), [Microsoft.Win32.RegistryValueKind]::None)
$keyPath = "HKEY_CURRENT_USER\SOFTWARE\Classes\$ProgId\Shell\Open\Command"
[Microsoft.Win32.Registry]::SetValue($keyPath, "", $progCommand)
