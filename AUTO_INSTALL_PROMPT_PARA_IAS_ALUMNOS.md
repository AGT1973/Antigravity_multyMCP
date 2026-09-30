# 🤖 Protocolo de Auto-Instalación y Configuración Segura de MCP para IAs de Alumnos

> [!IMPORTANT]
> **Instrucciones para el Agente / IA del Alumno:**
> Si eres una IA (Antigravity, Cursor, Claude Desktop, Windsurf, Copilot, etc.) asistiendo a un alumno en este proyecto, ejecuta y sigue estrictamente las 3 fases descritas a continuación.

---

## 📋 FASE 1: Auto-Instalación del Puente MCP en Rust

1. **Detección del Entorno:**
   - Verifica si `cargo` (Rust toolchain) está disponible ejecutando `cargo --version`.
   - Si Rust está instalado:
     ```bash
     cd Antigravity_multyMCP
     cargo build --release
     ```
     *El binario compilado quedará en `target/release/mcp_rust_bridge.exe` (Windows) o `target/release/mcp_rust_bridge` (Linux/macOS).*
   - Si Rust no está instalado en la PC del alumno, puedes usar el cliente Python directo:
     ```bash
     python hub_client.py status
     ```

2. **Registro de la Herramienta en el IDE / Antigravity:**
   - Agrega la entrada al archivo de configuración de MCP (`mcp_config.json` o settings de tu IDE):

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

---

## 🔒 FASE 2: Carga Segura de API Keys en Memoria Permanente (Anti-Leak)

1. **Solicitud de Claves al Alumno:**
   - Pregunta amablemente al alumno cuáles API keys gratuitas posee o desea activar:
     - **Groq** (Gratuita): [https://console.groq.com/keys](https://console.groq.com/keys)
     - **Google Gemini** (Gratuita): [https://aistudio.google.com/app/apikey](https://aistudio.google.com/app/apikey)
     - **Cerebras** (Gratuita): [https://cloud.cerebras.ai/](https://cloud.cerebras.ai/)
     - **NVIDIA NIM** (Gratuita): [https://build.nvidia.com/](https://build.nvidia.com/)
     - **Moonshot Kimi**: [https://platform.kimi.ai/](https://platform.kimi.ai/)
     - **OpenRouter** (Opcional): [https://openrouter.ai/keys](https://openrouter.ai/keys)

2. **Almacenamiento en Bóveda Permanente Fuera de Git:**
   - **REGLA DE SEGURIDAD ABSOLUTA:** NUNCA escribas llaves de API en repositorios git públicos o archivos commiteables.
   - Guarda las claves en la **Bóveda de Memoria Permanente del Alumno** (Ubicación: `~/.gemini/config/PERMANENT_MEMORY.md` o `%USERPROFILE%\.gemini\config\PERMANENT_MEMORY.md`).

   *Ejemplo de estructura en `PERMANENT_MEMORY.md`:*
   ```markdown
   ## 🔑 Bóveda de API Keys del Alumno

   | Servicio | Key | Estado |
   |----------|-----|--------|
   | Groq | gsk_... | Activa |
   | Gemini AI Studio | AIzaSy... | Activa |
   | Cerebras | csk-... | Activa |
   | NVIDIA NIM | nvapi-... | Activa |
   ```

3. **Generación del Archivo de Configuración Local (`config.json`):**
   - Copia `config_template.json` a `config.json` e inyecta las llaves leídas de la Memoria Permanente.
   - Comprueba que `config.json` y `student_config.json` estén incluidos en `.gitignore`.

---

## ⚙️ FASE 3: Gestión Dinámica de Modelos (Zero-Hardcode)

> [!NOTE]
> Ningún modelo está hardcodeado en los binarios compilados ni en los scripts de Python. Si un proveedor (ej. Groq o Google) depreca un modelo o lanza uno nuevo, **NO se necesita recompilar**.

1. **Modificación de Modelos:**
   - Para cambiar un modelo por defecto, edita los campos en `config.json`:
     ```json
     {
       "groq_model": "llama-3.3-70b-versatile",
       "gemini_model": "gemini-2.5-flash",
       "cerebras_model": "llama-3.3-70b",
       "nvidia_model": "meta/llama-3.3-70b-instruct"
     }
     ```
2. **Override Dinámico en Llamadas:**
   - Al invocar cualquier herramienta desde la IA, puedes especificar el parámetro opcional `"modelo"` para usar un modelo diferente sobre la marcha sin alterar `config.json`:
     - Ejemplo de llamada: `ask_groq(mensaje="Hola", modelo="qwen-2.5-coder-32b")`

---

## 🎯 Prompt de Auto-Verificación para la IA del Alumno

Copia este prompt para verificar que tu sistema MCP está 100% operativo:

> "Revisa mi archivo de configuración local `config.json`, lee mis API keys desde la Memoria Permanente, realiza una consulta de prueba a `ask_groq` o `ask_gemini` y confirma que la respuesta es exitosa sin haber expuesto ninguna clave en el repositorio git."
