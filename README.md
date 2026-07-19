## Antigravity hub for multiple Agent-AI (Windows, Linux, and Mac). 

## Tema Antigravity para Windows, Linux y Mac. Código fuente y compilado.

An ultra-fast, unified Model Context Protocol (MCP) bridge, developed in Rust/Python for Antigravity.

Un puente ultrarrápido y unificado del Protocolo de Contexto de Modelo (MCP), desarrollado en Rust/Python para Antigravity.

This native server absorbs and unifies the operations of multiple Python bridges (processor-local, processor-multicloud, processor-openrouter), reducing memory consumption from ~250MB to less than 5MB and providing native latency routing ("Zero-Waste").

Este servidor nativo absorbe y unifica las operaciones de múltiples puentes Python (procesador-local, procesador-multicloud, procesador-openrouter), reduciendo el consumo de memoria de ~250 MB a menos de 5 MB y proporcionando enrutamiento de baja latencia nativo ("Cero desperdicio").

Total Unification: Transparent support for 20 tested AI hubs (Groq, Gemini, HuggingFace, Cerebras, SambaNova, OpenRouter, and local Ollama).

Unificación total: Soporte transparente para 20 hubs de IA probados (Groq, Gemini, HuggingFace, Cerebras, SambaNova, OpenRouter y Ollama local).

## Main Features / Características principales

- **Total Unification**: Transparent support for 7 hubs with 20 tested AIs (Groq, Gemini, HuggingFace, Cerebras, SambaNova, OpenRouter, and local Ollama).
- **Unificación total**: Soporte transparente para 7 hubs con 20 IA probadas (Groq, Gemini, HuggingFace, Cerebras, SambaNova, OpenRouter y Ollama local).
- **Embedded System Operations**: Integrated tools for reading and writing files (TXT, MD, JSON, CSV), managing folders, performing searches, and executing Git operations without requiring Python.
- **Operaciones del sistema embebido**: Herramientas integradas para leer y escribir archivos (TXT, MD, JSON, CSV), gestionar carpetas, realizar búsquedas y ejecutar operaciones Git sin necesidad de Python.
- **Dynamic Toggle**: A system of boolean flags (`enable_openrouter`, `enable_groq`, etc.) allows you to turn MCP toolsets on or off "on the fly." If a provider is disabled, the associated MCP tools are simply not advertised to Antigravity.
- **Activación dinámica**: Un sistema de indicadores booleanos (`enable_openrouter`, `enable_groq`, etc.) permite activar o desactivar las herramientas MCP sobre la marcha. Si un proveedor está desactivado, las herramientas MCP asociadas simplemente no se anuncian a Antigravity.
- **100% Portability**: The executable dynamically searches for a configuration file (`config.json`) in the same folder where it is hosted, making it ideal for university and lab environments.
- **Portabilidad total**: El ejecutable busca dinámicamente un archivo de configuración (`config.json`) en la misma carpeta donde se aloja, lo que lo hace ideal para entornos universitarios y de laboratorio.

## Installation / instalación ("Plug & Play")

1. Download the executable from the [Releases](#) section.
1. Descargue el ejecutable de la sección [Versiones](#).

2. Download the `config_template.json` template and rename it to `config.json`.
2. Descargue la plantilla `config_template.json` y cámbiele el nombre a `config.json`.

3. Edit `config.json` to add your API keys and set the services you want to expose to `true`.
3. Edita `config.json` para añadir tus claves API y establece los servicios que deseas exponer en `true`.

4. In Antigravity, edit `mcp_config.json` and add this bridge
4. En Antigravity, edita `mcp_config.json` y añade este puente

```json
{
  "mcpServers": {
    "mcp-unificado-rust": {
      "command": "C:/TU/RUTA/mcp_rust_bridge.exe",
      "args": []
    }
  }
}
```

## Compilación desde código fuente

Si quieres modificar las operaciones locales (por ejemplo, para agregar soporte pesado de PDF/Excel) o simplemente auditar el código:

```bash
git clone https://github.com/AGT1973/Antigravity_multyMCP.git
cd Antigravity_multyMCP
cargo build --release
```
El binario optimizado quedará en `target/release/mcp_rust_bridge.exe`.

---
*Diseñado para la tesis de Espacios Nativos de Alta Dimensión (LatentMAS).*
