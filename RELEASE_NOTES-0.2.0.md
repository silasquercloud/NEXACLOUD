# NexaCloud 0.2.0

## Novidades

- **Teclado e mouse nativos:** o movimento do mouse é enviado como delta relativo
  ao xCloud, girando a câmera como o modo F8 do cliente de navegador. Clicar na
  imagem do jogo ativa o modo relativo sem travar o cursor do sistema — correção
  necessária para Hyprland/Wayland, onde `CursorGrab::Locked` congelava o vídeo.
  **F8** alterna entre teclado/mouse nativo e gamepad virtual.
- **Menu F1 lateral:** painel estilo "control center" do PS5, ancorado à direita,
  com timer de sessão, estatísticas, teclado/mouse nativo, upscale e saída.
- **Identidade NexaCloud:** ícone novo, tema azul PS5/OmniDash, wallpaper
  navy/cyan próprio e todos os textos/títulos atualizados — nenhum nome de jogo
  específico é fixado no app.
- **Config fora do jogo:** alvo de 30/60/120/240 fps e opção de VSync. O present
  mode é negociado com segurança em drivers que só oferecem Fifo (HD 3000 GL).
- **Login no Windows com qualquer navegador:** a leitura do redirect é feita dos
  perfis do Chrome, Edge, Firefox, Brave, Vivaldi e Opera (Windows, Linux e
  macOS), com fallback para `python`/`py -3` quando `python3` não está no PATH.
  O app abre o navegador padrão do sistema em vez de exigir Chromium.

## Sem alterações

- Pipeline de streaming, WebRTC, autenticação e sessão persistente permanecem
  intactos. Padrões mantidos: 60 fps, Fifo/VSync, upscale/HUD conforme settings.

## Downloads

- `NexaCloud-linux-x86_64.tar.gz` — Linux x86_64 (extraia e rode `install.sh`)
- `NexaCloud-windows-x86_64.zip` — Windows x86_64 (extraia e rode `openxcloud.exe`)
- `SHA256SUMS` — hashes SHA-256 dos dois arquivos
