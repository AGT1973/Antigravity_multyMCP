# Servidor MCP Unificado - Antigravity

Este ejecutable (`mcp_unificado.exe`) contiene todo lo necesario para que tus agentes de IA puedan interactuar con el sistema (leer/escribir archivos, consultar Git, etc.) y rutear sus peticiones a diferentes cerebros (Groq, Gemini, Ollama, OpenRouter, etc.) a velocidades ultrarrápidas y sin consumir memoria extra.

**No necesitas instalar Python ni ninguna librería.**

## Instalación y Uso

1. **Guarda ambos archivos** (`mcp_unificado.exe` y `config.json`) en una misma carpeta donde no los vayas a borrar.
2. Abre `config.json` con cualquier editor de texto (Notepad, VSCode).
3. **Pega tus API Keys** (claves) en los servicios que quieras usar. Si no tienes clave para un servicio, déjalo vacío o ponlo en `false`.
4. **Configura los booleanos**: Abajo en el archivo verás opciones como `"enable_groq": true`. Pon en `true` los servicios que deseas que tus agentes utilicen, y en `false` los que no.
5. Registra el MCP en tu archivo `mcp_config.json` de Antigravity (ubicado normalmente en `C:\Users\<tu_usuario>\.gemini\config\mcp_config.json`):

```json
{
  "mcpServers": {
    "mcp-unificado-rust": {
      "command": "C:/RUTA/A/TU/CARPETA/mcp_unificado.exe",
      "args": []
    }
  }
}
```

*Recuerda cambiar `C:/RUTA/A/TU/CARPETA/` por la ruta real donde guardaste este ejecutable. ¡Usa barras normales (`/`)!*

## Compilar desde el código fuente
Si deseas inspeccionar cómo está hecho, auditar su seguridad o añadirle soporte para leer PDFs/Excel, tienes disponible el código fuente en Rust en la carpeta `src`. Podrás compilarlo tú mismo ejecutando: `cargo build --release`.
