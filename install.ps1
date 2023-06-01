Set-StrictMode -Version latest

[string]$licerOS = ""
[string]$licerVersion = "0.1.0"
[string]$installDir = "$($Env:USERPROFILE)\bin"
[string]$installedVersion = (licer -v 2> $null)?.Substring(7, 5)

enum MessageType {
    Info
    Success
    Warning
    Error
}

function Print([string]$out, [ValidateRange(0, 3)]$type) {
    switch ($type) {
        ([MessageType]::Info) {
            Write-Host "$($PSStyle.Bold)$($PSStyle.Foreground.Blue)INFO$($PSStyle.Reset) $($out)"
        }
        ([MessageType]::Success) {
            Write-Host "$($PSStyle.Bold)$($PSStyle.Foreground.Green)SUCCESS$($PSStyle.Reset) $($out)"
        }
        ([MessageType]::Warning) {
            Write-Host "$($PSStyle.Bold)$($PSStyle.Foreground.Yellow)WARN$($PSStyle.Reset) $($out)"
        }
        ([MessageType]::Error) {
            Write-Host "$($PSStyle.Bold)$($PSStyle.Foreground.Red)ERROR$($PSStyle.Reset) $($out)"
        }
    }
}

switch -Regex (($PSVersionTable.OS + $Env:PROCESSOR_ARCHITECTURE).ToLower()) {
    "darwin|linux" {
        Print "To install Licer on Linux or macOS, please use the install.sh script as described within the readme!" ([MessageType]::Error)
        exit 1
    }
    "windows.*amd64" {
        $licerOS = "x86_64-pc-windows-msvc"
    }
    "windows.*arm64" {
        $licerOS = "aarch64-pc-windows-msvc"
    }
    default {
        Print "Licer is not precompiled for your system! You may clone and compile it from here: https://github.com/zahtec/licer" ([MessageType]::Error)
        exit 1
    }
}

if ([convert]::ToInt16($licerVersion.Replace(".", "")) -ge [convert]::ToInt16($installedVersion.Replace(".", ""))) {
    Write-Host "You are attempting to install $($PSStyle.Bold)Licer v$($licerVersion)$($PSStyle.Reset) when $($PSStyle.Bold)Licer v$($installedVersion)$($PSStyle.Reset), the same or later version, is already installed."

    if ((Read-Host "Proceed? [Y/n] ") -ne "Y") {
        exit 0
    }
}

try {
    [string[]]$split = $installDir -split "\\"
    New-Item -Path ($split[0..($split.Length - 2)] -join "\") -Name $split[-1] -ItemType "directory" -Force
} catch {
    Print "Failed to find or create $($installDir)!" ([MessageType]::Error)
    exit 1
}

try {
    Print "Downloading and extracting Licer v$($licerVersion)..." ([MessageType]::Info)

    [string]$temp = "$(Env:$TEMP)\licer-v$($licerVersion)-$($licerOS).tar.gz"

    Invoke-RestMethod -Uri "https://github.com/zahtec/licer/releases/download/v$($licerVersion)/licer-v$($licerVersion)-$($licerOS).tar.gz" -Method Get -OutFile $temp > $null
    tar -xf $temp -C $installDir
    Remove-Item -Path $temp
} catch {
    Print "Failed to download and extract Licer v$($licerVersion)" ([MessageType]::Error)
    exit 1
}

if ($Env:PATH -notlike "*$($installDir)*") {
    Print "Please add $($installDir) to your PATH variable in order to utilize Licer globally!" ([MessageType]::Error)
    Print "Licer v$($licerVersion) installed with errors. $($PSStyle.Bold)Please check above to ensure usage capabilities.$($PSStyle.Reset)" ([MessageType]::Warning)
} else {
    Print "Licer v$($licerVersion) installed successfully!" ([MessageType]::Success)
}

exit 0
