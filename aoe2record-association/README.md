# AoE2 Replay File Type Association

In Windows you can associate file types with programs so that when you double-click a file of that type, a specific program opens. For example, a .jpg file might open in the Photos app. You might have noticed that if you try to do this with `.aoe2record` files by clicking "Open with..." and choose Capture Age, it doesn't quite work. It will open Capture Age but not load the replay.

If you open a Windows terminal and paste the following [snippet](aoe2record-association.ps1), the file type association will be set up properly so that when you double-click an `.aoe2record` file, Capture Age will start and load directly into the replay.

```
$Extension = ".aoe2record"
$ProgId = "CaptureAgeLocal"
$captureAgePath = "$env:LOCALAPPDATA\Programs\CaptureAge\CaptureAge.exe"
$progCommand = "`"$captureAgePath`" `"captureage://spectate?aoe2record=%1`""
$keyPath = "HKEY_CURRENT_USER\SOFTWARE\Classes\$Extension\OpenWithProgids"
[Microsoft.Win32.Registry]::SetValue( $keyPath, $ProgId, ([byte[]]@()), [Microsoft.Win32.RegistryValueKind]::None)
$keyPath = "HKEY_CURRENT_USER\SOFTWARE\Classes\$ProgId\Shell\Open\Command"
[Microsoft.Win32.Registry]::SetValue($keyPath, "", $progCommand)
```

## Technical explanation

By convention, normally the way you open a file with a program is that you provide the path as the first argument when launching the program, for example `word.exe C:\some\path\my-document.docx`. However, Capture Age is not programmed to support this. However, they have programmed support for a custom protocol handler which allows you to open a URL of the form `captureage://spectate?aoe2record=<path>`. This URL is then passed in full as the first argument to Capture Age which will open the replay. This means that we just need to set up the file-type association slightly differently. Instead of using the common format of `"C:\path\to\captureage.exe" "%1"`, we instead use `"C:\path\to\captureage.exe" "captureage://spectate?aoe2record=%1"` as the association.

I have asked if Capture Age can add this association themselves, but until then, this work-around works.
