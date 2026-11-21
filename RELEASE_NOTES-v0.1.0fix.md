# NexaCloud v0.1.0fix (Windows)

Versão interna mantida em **0.1.0**. Não é v2, não é v3, não é alpha: é a primeira
build que jogava tranquila, com um tema novo e o menu F1 melhorado. Os releases
`v0.2.0` e `v2.1fix` continuam publicados e intocados.

## Como ficou o gameplay

- **Somente controle dentro do jogo**, igual ao `shyim/openxcloud` original. O teclado
  e o mouse continuam funcionando normalmente para navegar nos menus do nosso app —
  eles não são enviados para o PC na nuvem.
- Conferi o git upstream (`shyim/openxcloud`, branch `main`): lá o teclado só mapeia
  botões do controle virtual e **não existe** chave para ativar teclado/mouse nativo
  no jogo. Mantive exatamente esse comportamento.
- Voltou ao padrão da primeira build: apenas **30/60 fps**, sem as opções 120/240 e
  sem a troca do padrão de apresentação para VSync/Fifo. Nada foi mexido no
  negotiation de streaming, WebRTC, vídeo ou na sessão persistente.

## O que mudou visualmente

- **Tema roxo** em toda a interface (fundo, superfícies, destaque, brilho e foco),
  mantendo o mesmo layout da 0.1.0.
- **Menu F1 aprimorado**: cronômetro de sessão, título, resolução/fps/bitrate,
  alternadores de estatísticas e upscale, status do controle e botão de sair da
  sessão. Continua abrindo com **F1** e fechando com **Esc**.

## Correção do Windows preservada

- Mantive a correção de credenciais do `v2.1fix`: se o Gerenciador de Credenciais do
  Windows rejeitar o bundle após o login, o app grava localmente e a sessão continua
  persistente.

## Download

- `NexaCloud-windows-x86_64.zip` — Windows x86_64 (extraia e rode `openxcloud.exe`)
- `SHA256SUMS` — hash SHA-256 do ZIP
