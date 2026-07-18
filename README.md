# Antigravity_multyMCP

Un puente **Model Context Protocol (MCP)** ultrarrápido y unificado, desarrollado en Rust para [Antigravity](https://github.com/AGT1973/Antigravity).

Este servidor nativo absorbe y unifica las operaciones de múltiples puentes Python (`procesador-local`, `procesador-multicloud`, `procesador-openrouter`), reduciendo el consumo de memoria de ~250MB a **menos de 5MB** y proporcionando un ruteo a latencia nativa ("Zero-Waste").

## Características Principales

- **Unificación Total**: Soporte transparente para 7 IAs (Groq, Gemini, HuggingFace, Cerebras, SambaNova, OpenRouter y Ollama local).
- **Operaciones de Sistema Embebidas**: Herramientas integradas para leer y escribir archivos (TXT, MD, JSON, CSV), administrar carpetas, realizar búsquedas y ejecutar operaciones Git sin necesidad de Python.
- **Toggle Dinámico**: Un sistema de banderas booleanas (`enable_openrouter`, `enable_groq`, etc.) permite prender o apagar conjuntos de herramientas MCP "al vuelo". Si se desactiva un proveedor, las herramientas MCP asociadas simplemente no se anuncian a Antigravity.
- **100% Portabilidad**: El ejecutable busca dinámicamente un archivo de configuración (`config.json`) en la misma carpeta donde se encuentra alojado, haciéndolo ideal para entornos universitarios y de laboratorio.

## Instalación ("Plug & Play")

1. Descarga el ejecutable desde la sección [Releases](#).
2. Descarga la plantilla `config_template.json` y renómbrala a `config.json`.
3. Edita `config.json` para añadir tus *API Keys* y configurar en `true` los servicios que deseas exponer.
4. En Antigravity, edita el `mcp_config.json` y añade este puente:

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
