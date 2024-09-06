# --------------------------------------------------
# .NET Publishing Assistant
# Copyright (c) 2024 Kotran Contributors all rights reserved.
# --------------------------------------------------

[CmdletBinding(PositionalBinding = $false)]
Param(
    [string][Alias('v')]$verbosity = "minimal",
    [string][Alias('t')]$target = "",
    [bool][Alias('e')]$excludeSymbols = $true,
    [switch]$noLogo,
    [switch]$help,
    
    [string]$productName = "Unknown Product",
    [string]$productVersion = "1.0.0.0",
    [string]$fileDesc = "Unknown File Description",
    [string]$fileVersion = "1.0.0.0",
    [string]$company = "Unknown Corporation",
    [string]$copyright = "Unknown Copyright",
    
    [Parameter(ValueFromRemainingArguments = $true)][String[]]$properties
)

function Invoke-ExitWithExitCode([int] $exitCode) {
    if ($ci -and $prepareMachine) {
        Stop-Processes
    }

    exit $exitCode
}

function Invoke-Hello {
    if ($nologo) {
        return
    }
    
    Write-Host ".NET Publishing Assistant" -ForegroundColor White
    Write-Host "Copyright (c) 2024 Kotran Contributors all rights reserved." -ForegroundColor White
    Write-Host ""
}

function Invoke-Help {
    Write-Host "Common settings:"
    Write-Host "  -verbosity <value>         Msbuild verbosity: q[uiet], m[inimal], n[ormal], d[etailed], and diag[nostic] (short: -v)"
    Write-Host "  -target <value>            Name of a solution or project file to build (short: -s)"
    Write-Host "  -excludeSymbols <value>    If it is true, exclude debug symbols(*.pdb) (short: -e)"
    Write-Host "  -noLogo                    Doesn't display the startup banner or the copyright message"
    Write-Host "  -help                      Print help and exit"
    Write-Host ""
    
    Write-Host "Descriptions:"
    Write-Host "  -productName <value>       Product name"
    Write-Host "  -productVersion <value>    Product version"
    Write-Host "  -fileDesc <value>          File description"
    Write-Host "  -fileVersion <value>       File version"
    Write-Host "  -company <value>           Company name"
    Write-Host "  -copyright <value>         Copyright information"
    Write-Host ""
}

function Initialize-Script {
    # Check the target
    if ([string]::IsNullOrEmpty($target) -eq $True) {
        Write-Host "The target path is empty. The assistant publishes all of the projects." -ForegroundColor Green
        $Script:TargetPath = (Resolve-Path -Path "$($PSScriptRoot)\..\*.sln").ToString()
        return
    }

    if ((Test-Path "$($PSScriptRoot)\..\src\$($target)") -eq $False) {
        Write-Host "Target $($PSScriptRoot)\..\src\$($target) does not found." -ForegroundColor Red
        Invoke-ExitWithExitCode 1
    } else {
        $Script:TargetPath = (Resolve-Path -Path "$($PSScriptRoot)\..\src\$($target)").ToString()
    }
}

function Invoke-Publish {
    if ($excludeSymbols -eq $true) {
        dotnet publish $Script:TargetPath -p:Configuration=Release -p:DebugType=None -p:DebugSymbols=false -p:Product=$productName -p:Version=$productVersion -p:AssemblyTitle=$fileDesc -p:AssemblyVersion=$fileVersion -p:Company=$company -p:Copyright=$copyright $properties --verbosity $verbosity --nologo
    } else {
        dotnet publish $Script:TargetPath -p:Configuration=Release -p:Product=$productName -p:Version=$productVersion -p:AssemblyTitle=$fileDesc -p:AssemblyVersion=$fileVersion -p:Company=$company -p:Copyright=$copyright $properties --verbosity $verbosity --nologo
    }

    if ($lastExitCode -ne 0) {
        Write-Host "Publishing failed." -ForegroundColor Red
        Invoke-ExitWithExitCode $LastExitCode
    }
}

if ($help) {
    Invoke-Help

    exit 0
}

[timespan]$execTime = Measure-Command {
    Invoke-Hello | Out-Default
    Initialize-Script | Out-Default
    Invoke-Publish | Out-Default
}

Write-Host "Finished in " -NoNewline
Write-Host "$($execTime.Minutes) min, $($execTime.Seconds) sec, $($execTime.Milliseconds) ms." -ForegroundColor Cyan

Write-Host "Finished at " -NoNewline
Write-Host "$(Get-Date -UFormat "%Y. %m. %d. %R")." -ForegroundColor Cyan