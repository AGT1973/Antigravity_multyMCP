# 🧭 Antigravity Multi-Agent Orchestrator (Version 2026-09-30 SOTA)

## 📌 Novedades Principales (Arquitectura Dinámica Sin Hardcodeo)

1. **Zero Hardcoded Models:** Ningún modelo de IA está hardcodeado en los binarios compilados en Rust ni en los scripts de Python. Los modelos se configuran dinámicamente desde `config.json` / `student_config.json` / `hubs_master_catalog.json` o mediante el argumento opcional `modelo` en cada llamada a las herramientas. Si un proveedor actualiza sus modelos, **no se requiere recompilar**.
2. **Bóveda de Seguridad (Anti-Leak):** Cero llaves de API expuestas en el repositorio. Las llaves se almacenan en la **Memoria Permanente del Alumno** (`~/.gemini/config/PERMANENT_MEMORY.md`) fuera del árbol de Git.
3. **Auto-Instalación Agéntica:** La IA del alumno puede autoinstalar el servidor MCP ejecutando el protocolo especificado en `AUTO_INSTALL_PROMPT_PARA_IAS_ALUMNOS.md`.

---

## 🤖 Protocolo de Auto-Instalación para IAs de Alumnos

Si le pides a tu IA (Antigravity, Cursor, Claude Desktop, Windsurf, etc.) que instale este repositorio, dale la siguiente instrucción:

> [!TIP]
> *"Por favor lee el archivo [AUTO_INSTALL_PROMPT_PARA_IAS_ALUMNOS.md](AUTO_INSTALL_PROMPT_PARA_IAS_ALUMNOS.md), compila el puente MCP en Rust, solicita mis API keys para guardarlas en mi Memoria Permanente (`PERMANENT_MEMORY.md`), genera mi `config.json` local sin exponer credenciales en Git y registra la herramienta MCP en mi IDE."*

---

## 🛠️ Instalación Manual Paso a Paso

### 1. Compilación del Puente en Rust (Recomendado)
```bash
cargo build --release
```
*(Requiere tener Rust instalado: [https://rustup.rs/](https://rustup.rs/))*

El binario resultante se ubicará en `target/release/mcp_rust_bridge.exe` (Windows) o `target/release/mcp_rust_bridge` (Linux/macOS).

### 2. Configurar el Servidor MCP en tu IDE / Antigravity
En tu archivo de configuración de MCP (`mcp_config.json`):

```json
{
  "mcpServers": {
    "antigravity_multymcp": {
      "command": "C:/Ruta/Absoluta/A/Antigravity_multyMCP/target/release/mcp_rust_bridge.exe",
      "args": []
    }
  }
}
```

### 3. Cargar tus API Keys Gratuitas
Copia `config_template.json` como `config.json` e inserta tus claves (o haz que tu IA las cargue automáticamente desde tu `PERMANENT_MEMORY.md`):

- **Groq (Gratuito):** [https://console.groq.com/keys](https://console.groq.com/keys)
- **Google Gemini (Gratuito):** [https://aistudio.google.com/app/apikey](https://aistudio.google.com/app/apikey)
- **Cerebras WSE (Gratuito):** [https://cloud.cerebras.ai/](https://cloud.cerebras.ai/)
- **NVIDIA NIM (Gratuito):** [https://build.nvidia.com/](https://build.nvidia.com/)
- **Moonshot Kimi:** [https://platform.kimi.ai/](https://platform.kimi.ai/)
- **HuggingFace:** [https://huggingface.co/settings/tokens](https://huggingface.co/settings/tokens)
- **OpenRouter:** [https://openrouter.ai/keys](https://openrouter.ai/keys)

---

## 💬 El Prompt Maestro para el Orquestador

Copia el siguiente prompt en la carpeta `.agents/` de tu proyecto:

> "Eres el Orquestador principal de mi ecosistema Antigravity. Tu objetivo es ayudarme a investigar, programar y aprender. Tienes a tu disposición un hub MCP en Rust que te conecta a los cerebros de IA del Tribunal (NVIDIA NIM, Kimi, Groq, Gemini, Cerebras, OpenRouter, etc.) de forma simultánea. Cuando necesites verificar una idea arquitectónica compleja, consulta al Tribunal (ej: `ask_groq`, `ask_gemini`, `ask_cerebras`) y contrasta sus respuestas. Para tareas pesadas locales, cuentas con Ollama en modo nocturno. Nuestro trabajo es de co-work: propone, critica y nunca dudes en señalar errores conceptuales o matemáticos."

---
*Diseñado para uso pedagógico - Actualizado Septiembre 2026.*
