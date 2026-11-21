# OpenXCloud para Android (APK)

Este APK é uma casca nativa em torno do **player oficial de xCloud**
(`https://www.xbox.com/play`). Ele existe porque o cliente Rust/WGPU de desktop
não tem backend de vídeo móvel, UI de toque nem token store via Android Keystore.

## O que é nativo aqui

- **Toque nativo**: o WebView entrega os eventos de toque do Android direto para
  o player, que os converte em relatórios `POINTER` do protocolo xCloud (o mesmo
  formato que o cliente desktop usa para ponteiro). Não há emulação de mouse.
- **Teclado nativo**: eventos de hardware são repassados ao player via
  `dispatchKeyEvent`, habilitando os relatórios `KEYBOARD` do xCloud em tablets
  com teclado.
- **Sessão persistente**: os cookies do WebView são mantidos entre aberturas,
  então o login Microsoft não precisa ser refeito toda vez.
- **Tela cheia e aceleração de hardware** com `FLAG_KEEP_SCREEN_ON`.

## O que NÃO é

Não é o renderizador Rust nativo. O decode H.264 e o WebRTC rodam dentro do
player da Microsoft, como no navegador — a diferença é que o app é um APK
instalável, sem barra de navegador, com toque de primeira classe e sessão salva.

## Como compilar

No GitHub Actions, o workflow `.github/workflows/android.yml` gera o APK e o
anexa aos releases como `OpenXCloud-android.apk`.

Localmente (requer Android SDK 35 e JDK 17):

```bash
cd android
gradle assembleRelease
# APK em: app/build/outputs/apk/release/app-release.apk
```
