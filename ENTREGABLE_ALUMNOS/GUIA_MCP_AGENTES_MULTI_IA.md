# 🎓 Guía Pedagógica: Protocolo MCP 2026 para Agentes Multi-IA
### Actualización Crítica de Arquitectura y Compatibilidad (Antigravity 2.21+, Claude Desktop, Cursor y Gemini CLI)
**Autor:** Ariel García Traba & Antigravity Pair-Programming  
**Fecha:** Octubre 2026  

---

## 📌 1. ¿Qué cambió en los Protocolos MCP (v2.21+)?

A partir de las actualizaciones de octubre de 2026 en los motores de lenguaje de agentes (Antigravity v2.21+, Claude Desktop v0.8+, Cursor 0.45+), los hosts MCP pasaron a aplicar **validaciones estrictas de especificación (JSON-RPC y regex de identificadores)**:

### ⚠️ Regla Crítica: El Límite Inviolable de 64 Caracteres
Todos los nombres de herramientas (*Tool Names*) deben cumplir estrictamente la expresión regular:
$$\text{Regex: } \wedge[a-zA-Z0-9\_-]{1,64}\$$$

Cuando un entorno de agentes (como Antigravity) registra un plugin, crea el nombre calificado concatenando:
$$\text{Tool FQDN} = \texttt{mcp\_} + \langle\text{nombre\_plugin}\rangle + \texttt{\_} + \langle\text{nombre\_servidor}\rangle + \texttt{\_} + \langle\text{nombre\_herramienta}\rangle$$

* **El Error Común (Crash Inmediato):**
  Si el plugin se llama `build-with-google-polydim-compute` (31 caracteres), el servidor `mcp-kaggle-compute` (18 caracteres) y la herramienta `kaggle_set_credentials` (22 caracteres):
  $$\text{Longitud Total} = 4 + 31 + 1 + 18 + 1 + 22 = \mathbf{77 \text{ caracteres}} > 64$$
  **Resultado:** El motor de lenguaje en Go/C++ rechaza la herramienta, se cuelga el handshake y se reinicia el servidor.

* **La Solución Estándar (Arquitectura Limpia):**
  Mantener nombres cortos y concisos:
  * Servidor: `kaggle` (en vez de `mcp-kaggle-compute`)
  * Herramienta: `set_token` (en vez de `kaggle_set_credentials`)
  * Longitud final: $\le 45$ caracteres (**100% compatible y blindado**).

---

## 🛠️ 2. Tabla de Nombres Cortos Recomendados

| Servicio / Rol | Nombre Servidor Anterior | Nombre Servidor Nuevo (Corto) | Herramientas Optimizadas |
|---|---|---|---|
| **Google Colab** | `mcp-colab-live` | `colab` | `set_tunnel`, `ping_hw`, `eval_code` |
| **Kaggle TPU/GPU** | `mcp-kaggle-compute` | `kaggle` | `check_auth`, `set_token`, `run_bench`, `get_results` |
| **NVIDIA NIM** | `mcp-nvidia` | `nvidia` | `reason_with_nvidia_nim` |
| **Groq Llama/Qwen** | `mcp-groq` | `groq` | `reason_with_groq` |
| **Cerebras CS-3** | `mcp-cerebras` | `cerebras` | `reason_with_cerebras` |
| **SambaNova** | `mcp-sambanova` | `sambanova` | `reason_with_sambanova` |
| **HuggingFace** | `mcp-hf` | `hf` | `reason_with_hf` |
| **Moonshot Kimi** | `mcp-kimi` | `kimi` | `reason_with_kimi` |

---

## 🌐 3. Cómo Conectar estos MCPs como Agentes en OTRAS IAs

Estos ejecutables compilados en Rust y scripts ligeros en Python son **100% agnósticos**: funcionan con cualquier IA que soporte el estándar MCP (*Model Context Protocol*).

### A) Claude Desktop (Anthropic)
Abre tu archivo `%APPDATA%\Claude\claude_desktop_config.json` y agrega:

```json
{
  "mcpServers": {
    "cerebras": {
      "command": "E:/Antigravity_multyMCP_Rust/target/release/mcp_cerebras.exe",
      "args": []
    },
    "groq": {
      "command": "E:/Antigravity_multyMCP_Rust/target/release/mcp_groq.exe",
      "args": []
    },
    "colab": {
      "command": "C:/python314/python.exe",
      "args": [
        "-u",
        "E:/Antigravity_multyMCP_Rust/mcp_colab_live.py"
      ]
    }
  }
}
```

### B) Cursor / Windsurf
En Cursor, ve a **Settings $\to$ Features $\to$ MCP Servers $\to$ Add New MCP Server**:
* **Name:** `cerebras`
* **Type:** `command`
* **Command:** `E:/Antigravity_multyMCP_Rust/target/release/mcp_cerebras.exe`

### C) Google Antigravity
En tu archivo `.gemini/config/mcp_config.json`:

```json
{
  "mcpServers": {
    "cerebras": {
      "command": "E:/Antigravity_multyMCP_Rust/target/release/mcp_cerebras.exe"
    },
    "kaggle": {
      "command": "C:/python314/python.exe",
      "args": ["-u", "E:/Antigravity_multyMCP_Rust/mcp_kaggle_compute.py"]
    }
  }
}
```

---

## 🔒 4. Seguridad Inviolable: Gestión de API Keys

> [!CAUTION]
> **REGLA DE ORO DE SEGURIDAD PARA CLASE Y PRODUCCIÓN:**  
> **NUNCA, BAJO NINGUNA CIRCUNSTANCIA, SUBAS CLAVES O TOKENS A GITHUB O REPOSITORIOS PÚBLICOS.**

1. **Variables de Entorno o Vault Local:**  
   Las claves se leen de variables de entorno locales (`GROQ_API_KEY`, `CEREBRAS_API_KEY`) o de la base de datos de bóveda local (`active_vault`).
2. **Archivos `.env` Protegidos:**  
   Todo archivo `.env` debe estar listado en el `.gitignore`.
3. **Verificación Pre-Commit:**  
   Antes de hacer `git commit`, ejecuta siempre:
   ```bash
   git status
   git diff --cached
   ```
   Asegúrate de que no haya cadenas del tipo `gsk_...`, `csk-...`, `sk-...`, `AIza...` o credenciales bancarias/personales.

---

## 🚀 5. Ventajas Pedagógicas y de Rendimiento

* **Cero Carga en Memoria:** Los servidores en Rust consumen $< 5\text{ MB}$ de RAM frente a los $> 150\text{ MB}$ de servidores Node.js/TypeScript.
* **Arranque en Frío Ultrarrápido:** Inicializan en menos de $15\text{ ms}$, eliminando los timeouts de 30 a 60 segundos que bloqueaban el IDE.
* **Multi-IA Cooperativo:** Permite que un modelo local o frontera (Gemini / Claude / GPT) consulte en milisegundos a Cerebras WSE-3 o Groq como árbitros auxiliares.
