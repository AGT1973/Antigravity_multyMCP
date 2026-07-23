# Antigravity_multyMCP (v2.0 Agosto 2026)

Un puente **Model Context Protocol (MCP)** ultrarrápido y unificado, desarrollado en Rust para [Antigravity](https://github.com/AGT1973/Antigravity), junto con servidores especializados en Python.

Este repositorio contiene:
1. **Rust Unified Bridge (`mcp_rust_bridge`)**: Absorbe y unifica las operaciones de múltiples IAs Cloud en paralelo, reduciendo drásticamente la latencia y memoria consumida frente a wrappers en Python.
2. **MCP Ollama Nocturno (`mcp_ollama_nocturno.py`)**: Un servidor independiente en Python exclusivo para modelos locales (modo nocturno / sabueso).
3. **Scripts de Test**: Herramientas integradas para validar endpoints antes de compilar y probar latencias.

## Características Principales (Agosto 2026)

- **Arquitectura Zero-Block**: El bloqueo o lentitud de Ollama ya no afecta al puente principal, al estar separado en su propio proceso.
- **Multicloud Unificado**: Soporte instantáneo para Groq, Gemini, HuggingFace, Cerebras y SambaNova.
- **Triple Slot OpenRouter**: Permite tener configurados simultáneamente 3 modelos Premium distintos vía OpenRouter (ej: Claude 3.5 Sonnet, ThinkingMachines Inkling y GPT-4o), cada uno como una herramienta independiente.
- **Operaciones de Sistema Embebidas**: Herramientas integradas en Rust para leer y escribir archivos (TXT, MD, JSON, CSV), búsquedas y operaciones Git.
- **100% Portabilidad**: El ejecutable en Rust usa un archivo `config.json` externo. Modifica claves y prende/apaga (`true`/`false`) IAs "al vuelo" sin recompilar.

## Instalación ("Plug & Play")

### 1. El Puente Cloud (Rust)
1. Descarga el ejecutable desde la sección Releases o compílalo tú mismo (`cargo build --release`).
2. Edita `config.json` para añadir tus *API Keys* y configurar los 3 slots de OpenRouter.
3. En Antigravity, añade al `mcp_config.json`:
```json
"mcp-7-bridges-rust": {
  "command": "C:/RUTA/mcp_rust_bridge.exe",
  "args": []
}
```

### 2. El Agente Nocturno (Ollama en Python)
Si vas a dejar tareas pesadas de investigación corriendo a la noche sin consumir saldo:
1. Instala dependencias: `pip install mcp httpx`
2. En Antigravity, añade al `mcp_config.json`:
```json
"mcp-ollama-nocturno": {
  "command": "C:/python314/python.exe",
  "args": ["C:/RUTA/mcp_ollama_nocturno.py"]
}
```

## Compilación desde código fuente y Pruebas

Antes de subir cambios de código, puedes validar que todos los puentes respondan haciendo:

```bash
python test_bridges.py
```

Para compilar el puente en Rust:

```bash
cd mcp_rust_bridge
cargo build --release
```
El binario optimizado quedará en `target/release/mcp_rust_bridge.exe`.

---
*Diseñado para la tesis de Espacios Nativos de Alta Dimensión (LatentMAS) - Versión Agosto 2026.*
