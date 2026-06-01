Unicode true
!include "MUI2.nsh"

!ifndef VERSION
  !define VERSION "0.0.0"
!endif
!define ROOT "..\..\.."

Name "codex123"
OutFile "${ROOT}\dist\windows\codex123-${VERSION}-windows-x64-setup.exe"
InstallDir "$LOCALAPPDATA\Programs\codex123"
InstallDirRegKey HKCU "Software\codex123" "InstallDir"
RequestExecutionLevel admin
SetCompressor /SOLID lzma

!define MUI_ICON "${ROOT}\apps\codex-plus-manager\src-tauri\icons\icon.ico"
!define MUI_UNICON "${ROOT}\apps\codex-plus-manager\src-tauri\icons\icon.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "SimpChinese"
!insertmacro MUI_LANGUAGE "English"

Section "Install"
  SetOutPath "$INSTDIR"

  nsExec::ExecToLog 'taskkill /IM codex123.exe /F'
  Pop $0
  nsExec::ExecToLog 'taskkill /IM codex123-manager.exe /F'
  Pop $0
  nsExec::ExecToLog 'taskkill /IM codex-plus-plus.exe /F'
  Pop $0
  nsExec::ExecToLog 'taskkill /IM codex-plus-plus-manager.exe /F'
  Pop $0

  File "${ROOT}\dist\windows\app\codex123.exe"
  File "${ROOT}\dist\windows\app\codex123-manager.exe"

  Delete "$DESKTOP\Codex++.lnk"
  Delete "$DESKTOP\Codex++ 管理工具.lnk"
  Delete "$DESKTOP\Codex++ 绠＄悊宸ュ叿.lnk"
  Delete "$SMPROGRAMS\Codex++\Codex++.lnk"
  Delete "$SMPROGRAMS\Codex++\Codex++ 管理工具.lnk"
  Delete "$SMPROGRAMS\Codex++\Codex++ 绠＄悊宸ュ叿.lnk"
  Delete "$SMPROGRAMS\Codex++\卸载 Codex++.lnk"
  RMDir "$SMPROGRAMS\Codex++"

  CreateShortcut "$DESKTOP\codex123.lnk" "$INSTDIR\codex123.exe" "" "$INSTDIR\codex123.exe"
  CreateShortcut "$DESKTOP\codex123 管理工具.lnk" "$INSTDIR\codex123-manager.exe" "" "$INSTDIR\codex123-manager.exe"
  CreateDirectory "$SMPROGRAMS\codex123"
  CreateShortcut "$SMPROGRAMS\codex123\codex123.lnk" "$INSTDIR\codex123.exe" "" "$INSTDIR\codex123.exe"
  CreateShortcut "$SMPROGRAMS\codex123\codex123 管理工具.lnk" "$INSTDIR\codex123-manager.exe" "" "$INSTDIR\codex123-manager.exe"
  CreateShortcut "$SMPROGRAMS\codex123\卸载 codex123.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\codex123-manager.exe"

  WriteUninstaller "$INSTDIR\uninstall.exe"
  WriteRegStr HKCU "Software\codex123" "InstallDir" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\codex123" "DisplayName" "codex123"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\codex123" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\codex123" "Publisher" "jzy5999-cpu"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\codex123" "DisplayIcon" "$INSTDIR\codex123-manager.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\codex123" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\codex123" "UninstallString" "$INSTDIR\uninstall.exe"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Codex++"
  DeleteRegKey HKCU "Software\Codex++"
SectionEnd

Section "Uninstall"
  nsExec::ExecToLog 'taskkill /IM codex123.exe /F'
  Pop $0
  nsExec::ExecToLog 'taskkill /IM codex123-manager.exe /F'
  Pop $0

  Delete "$DESKTOP\codex123.lnk"
  Delete "$DESKTOP\codex123 管理工具.lnk"
  Delete "$SMPROGRAMS\codex123\codex123.lnk"
  Delete "$SMPROGRAMS\codex123\codex123 管理工具.lnk"
  Delete "$SMPROGRAMS\codex123\卸载 codex123.lnk"
  RMDir "$SMPROGRAMS\codex123"

  Delete "$INSTDIR\codex123.exe"
  Delete "$INSTDIR\codex123-manager.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"

  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\codex123"
  DeleteRegKey HKCU "Software\codex123"
SectionEnd
