@echo off
set MANIFEST_PATH=%~dp0com.dexo.native.json

echo Registering Dexo Native Messaging Host for Google Chrome...
REG ADD "HKCU\Software\Google\Chrome\NativeMessagingHosts\com.dexo.native" /ve /t REG_SZ /d "%MANIFEST_PATH%" /f

echo Registering Dexo Native Messaging Host for Microsoft Edge...
REG ADD "HKCU\Software\Microsoft\Edge\NativeMessagingHosts\com.dexo.native" /ve /t REG_SZ /d "%MANIFEST_PATH%" /f

echo.
echo Dexo Native Messaging Host successfully installed!
pause
