# Todo List (Dioxus)

Um exemplo simples de lista de tarefas (Todo) construído com Dioxus 0.7 e Rust.

O que você precisa para rodar
- Rust toolchain (stable) com `cargo`.
- Dioxus CLI (`dx`) instalada. Eu recomendo instalar com:
```dioxus/README.md#L1-3
curl -sSL http://dioxus.dev/install.sh | sh
```
- Para testar no Android:
  - Android SDK / platform-tools (com `adb`) e um emulador AVD ou dispositivo físico com USB debugging ativado.
  - Java JDK (necessário para builds nativos).
- Opcional: ImageMagick se você quiser gerar/ajustar ícones localmente.

Como rodar (modo desenvolvimento)
1. Entre no diretório do projeto Dioxus:
```dioxus/README.md#L1-6
cd dioxus
```
2. Rode o servidor de desenvolvimento:
```dioxus/README.md#L7-11
# usa o default_platform definido em Dioxus.toml
dx serve

# ou, explicitamente para Android:
# dx serve --platform android
```

Estrutura básica do projeto
```dioxus/README.md#L12-22
dioxus/
├─ assets/
│  ├─ icons/         # ícones do app
│  ├─ main.css
├─ src/
│  ├─ main.rs        # código principal da aplicação
├─ Cargo.toml
├─ Dioxus.toml       # configuração do dioxus (platform, icons, identifier)
├─ README.md
```

Dicas rápidas
- Verifique dispositivos conectados com `adb devices`.
- Se o ícone não atualizar no emulador, tente desinstalar a app antiga ou reiniciar o launcher/emulador.
- Se quiser, eu posso ajudar a criar um script para gerar ícones automaticamente a partir de um PNG/SVG base.
