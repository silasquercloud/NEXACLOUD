#!/usr/bin/env bash
# Local Android APK build for Arch/Omarchy.
# Prerequisites (run once):
#   sudo pacman -S --needed jdk17-openjdk
#   mkdir -p ~/.android-sdk
#   curl -L -o /tmp/cmdline-tools.zip https://dl.google.com/android/repository/commandlinetools-linux-12266719_latest.zip
#   unzip -q /tmp/cmdline-tools.zip -d ~/.android-sdk/cmdline-tools
#   mv ~/.android-sdk/cmdline-tools/cmdline-tools ~/.android-sdk/cmdline-tools/latest
#   ~/.android-sdk/cmdline-tools/latest/bin/sdkmanager --licenses
#   ~/.android-sdk/cmdline-tools/latest/bin/sdkmanager "platforms;android-35" "build-tools;35.0.0" "platform-tools"
set -euo pipefail
cd "$(dirname "$0")/../android"

export ANDROID_HOME="$HOME/.android-sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export JAVA_HOME="/usr/lib/jvm/java-17-openjdk"

printf 'sdk.dir=%s\n' "$ANDROID_HOME" > local.properties

./gradlew --no-daemon assembleRelease

apk="app/build/outputs/apk/release/app-release.apk"
cp "$apk" "$HOME/Downloads/OpenXCloud-android.apk"
echo "Pronto: ~/Downloads/OpenXCloud-android.apk"
