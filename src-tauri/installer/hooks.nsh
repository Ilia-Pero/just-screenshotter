; Remembers the language chosen in the installer: the program speaks it at first.
!macro NSIS_HOOK_POSTINSTALL
  ${If} $UpdateMode <> 1
  ${AndIf} $PassiveMode <> 1
  ${AndIfNot} ${Silent}
    Push $0
    Push $1
    StrCpy $0 ""
    ${IfThen} $LANGUAGE = 1033 ${|} StrCpy $0 "en" ${|}
    ${IfThen} $LANGUAGE = 1049 ${|} StrCpy $0 "ru" ${|}
    ${IfThen} $LANGUAGE = 3082 ${|} StrCpy $0 "es" ${|}
    ${IfThen} $LANGUAGE = 1032 ${|} StrCpy $0 "el" ${|}
    ${IfThen} $LANGUAGE = 1037 ${|} StrCpy $0 "he" ${|}
    ${IfThen} $LANGUAGE = 2052 ${|} StrCpy $0 "zh" ${|}
    ${IfThen} $LANGUAGE = 1041 ${|} StrCpy $0 "ja" ${|}
    ${IfThen} $LANGUAGE = 1042 ${|} StrCpy $0 "ko" ${|}
    ${IfThen} $LANGUAGE = 3098 ${|} StrCpy $0 "sr" ${|}
    ${IfThen} $LANGUAGE = 1036 ${|} StrCpy $0 "fr" ${|}
    ${IfThen} $LANGUAGE = 1031 ${|} StrCpy $0 "de" ${|}
    ${IfThen} $LANGUAGE = 1040 ${|} StrCpy $0 "it" ${|}
    FileOpen $1 "$INSTDIR\language" w
    FileWrite $1 $0
    FileClose $1
    Pop $1
    Pop $0
  ${EndIf}
!macroend

; Removes what the installer itself did not bring.
!macro NSIS_HOOK_PREUNINSTALL
  ${If} $UpdateMode <> 1
    Delete "$INSTDIR\language"
  ${EndIf}
!macroend

; Forgets that the program starts with the system.
!macro NSIS_HOOK_POSTUNINSTALL
  ${If} $UpdateMode <> 1
    DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "${PRODUCTNAME}"
    DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run" "${PRODUCTNAME}"
  ${EndIf}
!macroend
