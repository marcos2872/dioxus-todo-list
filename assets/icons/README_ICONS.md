# README_ICONS

Este arquivo descreve os ícones usados pelo app e fornece orientações para adicionar placeholders e novos tamanhos para Android. O repositório já contém placeholders gerados automaticamente (veja a lista abaixo) e também ícones de fallback.

Existentes no repositório (fallback/web e placeholders gerados):
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.png`
- `ic_launcher_mdpi.png`    — placeholder gerado (48x48)
- `ic_launcher_hdpi.png`    — placeholder gerado (72x72)
- `ic_launcher_xhdpi.png`   — placeholder gerado (96x96)
- `ic_launcher_xxhdpi.png`  — placeholder gerado (144x144)
- `ic_launcher_xxxhdpi.png` — placeholder gerado (192x192)
- `icon-512.png`            — placeholder gerado (512x512)

Observações sobre formatos e qualidade
- Use PNG com fundo transparente quando possível.
- Para ícones vetoriais, prefira SVG para gerar as variantes PNG em várias resoluções.
- Mantenha contrastes e margens seguras (safe zone) para evitar cortes nas bordas em diferentes lançadores.

Como eu gerei os placeholders (ImageMagick)
- Gereis placeholders simples diretamente no repositório (cores e texto indicativo) usando ImageMagick. Os comandos usados (executados no diretório `dioxus`) foram equivalentes a:
```sh
convert -size 48x48   xc:'#2563eb' -gravity center -fill white -pointsize 10  -annotate 0 'mdpi'  assets/icons/ic_launcher_mdpi.png
convert -size 72x72   xc:'#1d4ed8' -gravity center -fill white -pointsize 14  -annotate 0 'hdpi'  assets/icons/ic_launcher_hdpi.png
convert -size 96x96   xc:'#0ea5e9' -gravity center -fill white -pointsize 18  -annotate 0 'xhdpi'  assets/icons/ic_launcher_xhdpi.png
convert -size 144x144  xc:'#06b6d4' -gravity center -fill white -pointsize 26  -annotate 0 'xxhdpi' assets/icons/ic_launcher_xxhdpi.png
convert -size 192x192  xc:'#10b981' -gravity center -fill white -pointsize 32  -annotate 0 'xxxhdpi' assets/icons/ic_launcher_xxxhdpi.png
convert -size 512x512  xc:'#f97316' -gravity center -fill white -pointsize 96  -annotate 0 '512'    assets/icons/icon-512.png
```
- Observação: dependendo da versão do ImageMagick, poderá aparecer uma advertência sugerindo usar `magick` em vez de `convert`. Ambos são equivalentes; se preferir, troque `convert` por `magick` antes do comando.

Atenção: estes placeholders são visuais simples para teste. Substitua por ícones finais de alta qualidade (fundo transparente, com margens corretas) antes de publicar.

Integração com `Dioxus.toml`
- A lista `application.icon` em `dioxus/Dioxus.toml` já foi atualizada para incluir os arquivos acima. Exemplo (já presente no arquivo):
```toml
[application]
icon = [
  "assets/icons/ic_launcher_mdpi.png",
  "assets/icons/ic_launcher_hdpi.png",
  "assets/icons/ic_launcher_xhdpi.png",
  "assets/icons/ic_launcher_xxhdpi.png",
  "assets/icons/ic_launcher_xxxhdpi.png",
  "assets/icons/icon-512.png",
  "assets/icons/icon.png",
  "assets/icons/128x128.png",
  "assets/icons/32x32.png",
]
```

Testando no Android
- Para desenvolvimento: `dx serve --platform android`
- Após adicionar ou substituir os ícones, teste o app no emulador/dispositivo para verificar se o ícone é carregado corretamente.
- Para builds finais (APK / App Bundle), use os comandos de empacotamento da CLI do Dioxus (verifique a documentação/localmente qual comando `dx` disponível em sua versão).

Boas práticas
- Forneça pelo menos até `xxhdpi`/`xxxhdpi` para melhor cobertura de dispositivos modernos.
- Mantenha um ícone grande (512x512) para representações em lojas ou screenshots.
- Versione os ícones no controle de versão para reproduzibilidade.

Se quiser, eu posso:
- substituir os placeholders por imagens geradas a partir do seu `icon.png` (se você preferir que eu redimensione a imagem `icon.png` existente ao invés dos placeholders gerados), ou
- preparar scripts (Makefile/steps) para gerar automaticamente as variantes a partir de um SVG/PNG de alta resolução.