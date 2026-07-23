# Proyecto: Configuración MCP & Orquestación Sincronizada (v2.0 Agosto 2026)

Este repositorio documenta y mantiene la estructura multi-agente (`configuracion_mpc`) diseñada **para ser compartida y utilizada por los alumnos** en sus proyectos. 

### Propósito y Alcance del Proyecto
1. **El Hub de 8 Agentes:** Este entorno se enfoca exclusivamente en manejar las 8 IAs principales (incluyendo la reciente integración de *Thinkingmachines/Inkling*) configuradas como **Agentes nativos de AGY**. Esto permite que el orquestador principal (Antigravity) coordine y consulte a los modelos simultáneamente vía Hub a través de un puente ultrarrápido en Rust (`mcp_rust_bridge`).
2. **Restricción Estricta de Co-Work (Ollama):** Se provee un MCP secundario (`mcp_ollama_nocturno.py`) exclusivo para trabajo nocturno o en background. **Regla de oro:** Este modelo jamás debe iniciar `ollama serve` si el usuario está utilizando la PC, ya que la latencia y consumo de disco impedirían el trabajo fluido humano-máquina.
3. **Monitorización Continua:** El ecosistema cuenta con un vigilante automático que monitoriza la estructura multi-agente y garantiza que tanto el código en Git (GitHub) como las copias de seguridad en Google Drive se mantengan permanentemente sincronizadas y actualizadas.

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
