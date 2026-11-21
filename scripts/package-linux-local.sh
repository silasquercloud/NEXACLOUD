#!/usr/bin/env bash
# Pacote Linux portátil para release: binário + launcher + desktop entry + instalador.
# Rode depois de `cargo build --locked --release -p xcloud-app`.
set -euo pipefail
cd "$(dirname "$0")/.."

source "$HOME/.cargo/env"
target_dir=$(cargo metadata --locked --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')
binary="$target_dir/release/openxcloud"
test -x "$binary" || { echo "Binário não encontrado: $binary" >&2; exit 1; }

version=$(cargo metadata --locked --no-deps --format-version 1 | python3 -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"] == "xcloud-app"))')
arch=x86_64
name="OpenXCloud-linux-${arch}"
stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/$name"

cp "$binary" "$stage/$name/openxcloud"
cp LICENSE THIRD_PARTY_NOTICES.md "$stage/$name/"
mkdir -p "$stage/$name/assets/icons"
cp assets/icons/openxcloud-256.png "$stage/$name/assets/icons/"

cat > "$stage/$name/launch-openxcloud.sh" <<'EOF'
#!/usr/bin/env bash
# Launcher portátil: usa o binário ao lado deste script.
set -euo pipefail
DIR="$(cd -- "$(dirname -- "$0")" && pwd)"
export WGPU_BACKEND="${WGPU_BACKEND:-gl}"
export RUST_LOG="${RUST_LOG:-info}"
exec "$DIR/openxcloud" --present fifo "$@"
EOF
chmod +x "$stage/$name/launch-openxcloud.sh"

cat > "$stage/$name/install.sh" <<'EOF'
#!/usr/bin/env bash
# Instala o launcher, o ícone e a entrada no menu de aplicativos do usuário.
set -euo pipefail
DIR="$(cd -- "$(dirname -- "$0")" && pwd)"
DEST="$HOME/.local"
mkdir -p "$DEST/bin" "$DEST/share/applications" "$DEST/share/icons/hicolor/256x256/apps"
cp "$DIR/openxcloud" "$DEST/bin/openxcloud"
cp "$DIR/launch-openxcloud.sh" "$DEST/bin/launch-openxcloud.sh"
cp "$DIR/assets/icons/openxcloud-256.png" "$DEST/share/icons/hicolor/256x256/apps/openxcloud.png"
cat > "$DEST/share/applications/openxcloud.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Version=1.0
Name=OpenXCloud (Fortnite)
GenericName=Xbox Cloud Gaming
Comment=Cliente nativo otimizado para hardware gráfico modesto
Exec=$DEST/bin/launch-openxcloud.sh
Icon=$DEST/share/icons/hicolor/256x256/apps/openxcloud.png
Terminal=false
Categories=Game;Network;
Keywords=xcloud;fortnite;xbox;gamepass;
StartupNotify=true
DESKTOP
chmod +x "$DEST/bin/openxcloud" "$DEST/bin/launch-openxcloud.sh"
command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$DEST/share/applications" || true
echo "Instalado. Abra OpenXCloud no menu de aplicativos."
EOF
chmod +x "$stage/$name/install.sh"

cat > "$stage/$name/README-LINUX.md" <<EOF
# OpenXCloud $version (Linux x86_64)

Extraia este arquivo e rode \`./install.sh\` para instalar no seu usuário, ou
execute \`./launch-openxcloud.sh\` diretamente sem instalar.

Requisitos: sessão Wayland/X11 com driver OpenGL funcional. O launcher já usa
\`WGPU_BACKEND=gl\` e \`--present fifo\`, o caminho estável em hardware modesto.

Pressione **F1** durante uma partida para abrir o menu (timer, estatísticas,
entrada nativa e voltar ao gamepad virtual).
EOF

mkdir -p dist
tar -czf "dist/${name}.tar.gz" -C "$stage" "$name"
cp "dist/${name}.tar.gz" "$HOME/Downloads/${name}.tar.gz"
echo "Pronto: ~/Downloads/${name}.tar.gz"
