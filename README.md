# 🚀 Hub de Orquestación Multi-Agente SOTA (Agosto 2026)

> [!NOTE]
> **Hola Alumno/a:** Este repositorio contiene la infraestructura *State of the Art* (SOTA) que usarás para conectar tu sistema Antigravity con los cerebros de IA más potentes del mundo. ¡Sigue esta guía paso a paso!

## 🧠 ¿Por qué 10 Hubs y Rust? (Nuestra Filosofía)
En este proyecto no dependemos de una sola IA. Usamos un **Tribunal de 10 Agentes** porque la verdadera autonomía se logra cuando las IAs discuten y validan datos entre sí. 
Para manejar 10 cerebros gigantescos sin que tu PC colapse, desarrollamos un **Puente en Rust (Stateless Enterprise MCP)**. Este puente es ultrarrápido, no guarda basura en memoria (Zero-Trust) y nos permite hablar con los servidores más rápidos del planeta en milisegundos.

Además, mantenemos **Ollama** (para correr IAs locales) separado en un MCP exclusivo para "Trabajo Nocturno". *Regla de oro: Ollama nunca debe bloquear tu PC mientras la estás usando.*

---

## 🛠️ Cómo Autoinstalarse (Paso a Paso)

### 1. El Puente Cloud (Rust)
1. **Compilar el código:** Abre tu terminal, entra a la carpeta `mcp_rust_bridge` y ejecuta:
   ```bash
   cargo build --release
   ```
   *(Necesitas tener Rust instalado en tu sistema: [rustup.rs](https://rustup.rs/))*

2. **Configurar Antigravity:** 
   Ve a la configuración de tu aplicación Antigravity (tu `mcp_config.json`) y agrega el servidor Rust apuntando al binario generado.
   - **Windows:** `C:/TU_RUTA/mcp_rust_bridge/target/release/mcp_rust_bridge.exe`
   - **Mac/Linux:** `/TU_RUTA/mcp_rust_bridge/target/release/mcp_rust_bridge`

### 2. Conseguir tus Llaves (API Keys)
Para despertar a los agentes, necesitas generar tus propias llaves. ¡La mayoría son **100% gratuitas**! Regístrate en estos links y copia las llaves que te den en el archivo `config.json` dentro de tu carpeta `mcp_rust_bridge`.

- 🟢 **NVIDIA NIM (Recomendado):** [build.nvidia.com](https://build.nvidia.com/) *(+100 modelos de frontera gratis como Llama 3.1 70B)*
- 🟢 **Moonshot Kimi:** [platform.kimi.ai](https://platform.kimi.ai/) *(Kimi 2.6 libre asiático)*
- 🟢 **Groq:** [console.groq.com/keys](https://console.groq.com/keys) *(Velocidad extrema LPU)*
- 🟢 **Google Gemini:** [aistudio.google.com](https://aistudio.google.com/app/apikey)
- 🟢 **HuggingFace:** [huggingface.co/settings/tokens](https://huggingface.co/settings/tokens)
- 🟢 **Cerebras:** [cloud.cerebras.ai](https://cloud.cerebras.ai/)
- 🟢 **SambaNova:** [cloud.sambanova.ai](https://cloud.sambanova.ai/)
- 🔵 **OpenRouter:** [openrouter.ai/keys](https://openrouter.ai/keys) *(Para agrupar premium como Claude, GPT-4o e Inkling)*

---

## 🤖 El Prompt Mágico para tu IA
Para que tu agente personal dentro de Antigravity entienda todo este inmenso poder y sepa cómo interactuar contigo, **copia y pégale el siguiente Prompt en su configuración** (por ejemplo, en la carpeta `.agents/`):

> [!TIP]
> **Copia este texto y dáselo a tu IA Orquestadora:**
> 
> "Eres el Orquestador principal de mi ecosistema Antigravity. Tu objetivo es ayudarme a investigar, programar y aprender. Tienes a tu disposición un Hub MCP en Rust que te conecta a 10 cerebros de IA externos (Nvidia NIM, Kimi, Groq, Gemini, OpenRouter, etc.) de forma simultánea y Stateless (SOTA Julio 2026). Si necesitas corroborar una idea arquitectónica compleja, no asumas que sabes todo: usa tus herramientas para consultar al Tribunal (ej: `ask_nvidia` o `ask_kimi`) y contrasta sus respuestas. Para tareas pesadas de noche, cuentas con un sabueso local en Ollama que solo debes despertar cuando yo no use la PC. Tu trabajo conmigo es de co-work: propón, critica, y nunca dudes en decirme de forma directa si estoy equivocado conceptual o matemáticamente, porque mi objetivo es aprender."

---
*Diseñado para uso pedagógico - Versión Agosto 2026.*
