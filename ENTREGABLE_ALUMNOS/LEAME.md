# Servidor MCP Unificado - Antigravity (Versión SOTA 2026)

Este ejecutable (`mcp_unificado.exe`) contiene todo lo necesario para que tus agentes de IA interactúen con el sistema (leer/escribir archivos, consultar Git, etc.) y ruteen peticiones a los cerebros del Tribunal (Groq, Gemini, Cerebras, NVIDIA, Kimi, OpenRouter, Ollama) sin hardcodeo de modelos y a velocidades ultrarrápidas.

---

## 🔒 Regla de Seguridad (Bóveda de Claves en Memoria Permanente)

> [!IMPORTANT]
> **No guardes tus API Keys en el repositorio git del proyecto.**
> Pide a tu IA que lea tus claves desde tu **Memoria Permanente** (`~/.gemini/config/PERMANENT_MEMORY.md`) e inyecte los valores en tu `config.json` local.

---

## ⚙️ Configuración Dinámica de Modelos (Sin Recompilar)

En tu `config.json` puedes especificar o cambiar los modelos activos en cualquier momento sin necesidad de volver a compilar:

```json
{
  "groq_model": "llama-3.3-70b-versatile",
  "gemini_model": "gemini-2.5-flash",
  "cerebras_model": "llama-3.3-70b",
  "nvidia_model": "meta/llama-3.3-70b-instruct",
  "kimi_model": "moonshot-v1-8k",
  "enable_groq": true,
  "enable_gemini": true,
  "enable_cerebras": true,
  "enable_nvidia": true,
  "enable_local_ops": true
}
```

*También puedes indicarle a tu IA que use un modelo distinto sobre la marcha especificando la propiedad `"modelo"` en la llamada a la herramienta MCP (ej. `ask_groq(mensaje="...", modelo="qwen-2.5-coder-32b")`).*

---

## 🚀 Instalación y Registro MCP

Registra el servidor en tu `mcp_config.json` de Antigravity (ubicado en `C:\Users\<tu_usuario>\.gemini\config\mcp_config.json` o settings de tu IDE):

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

---

## 🛠️ Compilar desde el Código Fuente

El código fuente en Rust se encuentra en la carpeta `src/`. Puedes auditarlo o compilarlo tú mismo ejecutando:
```bash
cargo build --release
```
