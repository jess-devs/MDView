; Instalador de MDView. Ver docs/features/distribucion/plan.md y decisiones.md
; (AD-29) para el porque de cada decision de aqui abajo.
;
; Compilar: ISCC.exe packaging\mdview.iss   (tras `cargo build --release`)

#define MyAppName "MDView"
#define MyAppVersion "0.1.0"
#define MyAppExeName "mdview.exe"

[Setup]
AppId={{9E5D4564-6158-44D3-B633-C4F21B89B8AB}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
DefaultDirName={localappdata}\Programs\{#MyAppName}
DisableProgramGroupPage=yes
SetupIconFile=mdview.ico
UninstallDisplayIcon={app}\mdview.ico
; RF-02.1 y RF-21.1 exigen que no haga falta administrador: todo el registro
; y el PATH que escribe este instalador viven en HKCU y en el perfil del
; usuario, nunca en HKLM ni en Archivos de programa.
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
OutputDir=..\target\installer
OutputBaseFilename=mdview-setup
Compression=lzma
SolidCompression=yes

[Files]
Source: "..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "mdview.ico"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
; ProgID propio de MDView para abrir .md, y su comando de apertura.
Root: HKCU; Subkey: "Software\Classes\MDView.md"; ValueType: string; ValueName: ""; ValueData: "Documento Markdown"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\MDView.md\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Flags: uninsdeletekey
; Anade el ProgID a la lista de "Abrir con" de .md. No se toca la clave
; UserChoice: esa decide la aplicacion predeterminada, y desde Windows 8
; solo el propio dialogo de Windows puede escribirla validamente (AD-29).
Root: HKCU; Subkey: "Software\Classes\.md\OpenWithProgids"; ValueType: string; ValueName: "MDView.md"; ValueData: ""; Flags: uninsdeletevalue

[Code]
const
  EnvironmentKey = 'Environment';
  WM_SETTINGCHANGE = $001A;
  SMTO_ABORTIFHUNG = $0002;

function SendMessageTimeoutA(hWnd: Longint; Msg: Longint; wParam: Longint;
  lParam: AnsiString; fuFlags: Longint; uTimeout: Longint; var lpdwResult: Longint): Longint;
  external 'SendMessageTimeoutA@user32.dll stdcall';

procedure RefreshEnvironment;
var
  ResultCode: Longint;
begin
  SendMessageTimeoutA(HWND_BROADCAST, WM_SETTINGCHANGE, 0, 'Environment',
    SMTO_ABORTIFHUNG, 5000, ResultCode);
end;

procedure EnvAddPath(Dir: string);
var
  Paths: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths) then
    Paths := '';
  if Pos(';' + Uppercase(Dir) + ';', ';' + Uppercase(Paths) + ';') > 0 then
    exit;
  if (Length(Paths) > 0) and (Paths[Length(Paths)] <> ';') then
    Paths := Paths + ';';
  Paths := Paths + Dir;
  RegWriteStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths);
  RefreshEnvironment;
end;

procedure EnvRemovePath(Dir: string);
var
  Paths: string;
  P: Integer;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths) then
    exit;
  P := Pos(';' + Uppercase(Dir) + ';', ';' + Uppercase(Paths) + ';');
  if P = 0 then
  begin
    // Puede ser el ultimo tramo, sin ';' final.
    P := Pos(Uppercase(Dir), Uppercase(Paths));
    if (P = 0) or (P + Length(Dir) - 1 <> Length(Paths)) then
      exit;
    Delete(Paths, P - 1, Length(Dir) + 1);
  end
  else
    Delete(Paths, P, Length(Dir) + 1);
  RegWriteStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths);
  RefreshEnvironment;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
    EnvAddPath(ExpandConstant('{app}'));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
    EnvRemovePath(ExpandConstant('{app}'));
end;
