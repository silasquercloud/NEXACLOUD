# NexaCloud v2.1fix (Windows)

Correção da build Windows do v2. **Somente o instalador Windows mudou**; a build Linux
do v0.2.0 continua válida e não precisa ser reinstalada.

## O que foi corrigido

- **Erro `token store: write credentials_v1` após entrar:** no Windows, o Gerenciador
  de Credenciais tem limite de tamanho para cada credencial e pode rejeitar o bundle
  completo (`credentials_v1`) quando ele contém MSA + refresh + LPT + tokens de
  streaming. O app agora detecta a falha e grava esse bundle em um arquivo local do
  usuário, continuando o login normalmente. A sessão segue persistente entre
  reinícios. Se o Gerenciador de Credenciais voltar a funcionar, o app volta a usá-lo
  e remove a cópia local.
- Nenhuma alteração no fluxo de login com navegador: continua abrindo o navegador
  padrão e reconhecendo o redirect do Xbox em Chrome, Edge, Firefox, Brave, Vivaldi e
  Opera. Quem já instalou o Python no Windows não precisa fazer mais nada; o Python
  continua sem ser necessário para o próprio app.

## Sem alterações

- Streaming, WebRTC, teclado/mouse nativos, menu F1, F8, 30/60/120/240 fps e VSync.
- A correção do cursor no Hyprland (sem `CursorGrab::Locked`) permanece intacta.

## Downloads

- `NexaCloud-windows-x86_64.zip` — Windows x86_64 (extraia e rode `openxcloud.exe`)
- `SHA256SUMS` — hash SHA-256 do ZIP Windows
