# Antigravity Multi‑Agent Orchestrator (Version 2026-09-05)

> [!WARNING] **Only this README should be downloaded by students; all other documentation files are deprecated.**

## English

# 🧭 Hub de Orquestación Multi‑Agente SOTA (August 2026)

> [!NOTE]
> **Hello Student:** This repository contains the State‑of‑the‑Art (SOTA) infrastructure you will use to connect your Antigravity system to the most powerful AI brains in the world. Follow this guide step‑by‑step!

### Why 10 Hubs and Rust?
We do not rely on a single AI. We use a **Tribunal of 10 Agents** because true autonomy is achieved when AIs discuss and validate data among themselves. To handle 10 massive brains without crashing your PC, we built a **Rust bridge (Stateless Enterprise MCP)**. It is ultra‑fast, does not retain garbage in memory (Zero‑Trust), and lets us talk to the fastest servers on the planet in milliseconds.

*We also keep **Ollama** (for local AI) in a dedicated MCP for "Night‑time Work". *Rule of gold: Ollama must never block your PC while you are using it.*

---

### How to Auto‑Install (Step‑by‑Step)

1. **The Cloud Bridge (Rust)**
   ```
   cargo build --release
   ```
   (You need Rust installed: https://rustup.rs/)

   Then add the Rust server to your Antigravity config (`mcp_config.json`), pointing to the generated binary.

2. **Get Your API Keys** – Most are **100 % free**! Register at the links below and paste the keys into `config.json` inside `mcp_rust_bridge`.
   - **NVIDIA NIM (Recommended)**: https://build.nvidia.com/
   - **Moonshot Kimi**: https://platform.kimi.ai/
   - **Groq**: https://console.groq.com/keys
   - **Google Gemini**: https://aistudio.google.com/app/apikey
   - **HuggingFace**: https://huggingface.co/settings/tokens
   - **Cerebras**: https://cloud.cerebras.ai/
   - **SambaNova**: https://cloud.sambanova.ai/
   - **OpenRouter**: https://openrouter.ai/keys (for Claude, GPT‑4o, Inkling, etc.)

---

### The Magic Prompt for Your AI
Copy the following prompt into the `.agents/` folder of your Antigravity project:

> [!TIP]
> "You are the main Orchestrator of my Antigravity ecosystem. Your goal is to help me research, program, and learn. You have a Rust MCP hub connecting to 10 external AI brains (Nvidia NIM, Kimi, Groq, Gemini, OpenRouter, etc.) simultaneously and Stateless (SOTA July 2026). When you need to verify a complex architectural idea, do not assume you know everything: use your tools to consult the Tribunal (e.g., `ask_nvidia` or `ask_kimi`) and cross‑check their answers. For heavy night‑time tasks, you have a local Ollama watchdog that you only awaken when I am not using the PC. Our work is co‑work: propose, critique, and never hesitate to tell me directly if I am conceptually or mathematically wrong."

*Designed for pedagogical use – Version August 2026.*

## Español

# 🧭 Hub de Orquestación Multi‑Agente SOTA (Agosto 2026)

> [!NOTE]
> **Hola Alumno/a:** Este repositorio contiene la infraestructura *State of the Art* (SOTA) que usarás para conectar tu sistema Antigravity con los cerebros de IA más potentes del mundo. ¡Sigue esta guía paso a paso!

### ¿Por qué 10 Hubs y Rust?
En este proyecto no dependemos de una sola IA. Usamos un **Tribunal de 10 Agentes** porque la verdadera autonomía se logra cuando las IAs discuten y validan datos entre sí. Para manejar 10 cerebros gigantes sin que tu PC colapse, desarrollamos un **Puente en Rust (Stateless Enterprise MCP)**. Este puente es ultrarrápido, no guarda basura en memoria (Zero‑Trust) y nos permite hablar con los servidores más rápidos del planeta en milisegundos.

*Además, mantenemos **Ollama** (para ejecutar IAs locales) separado en un MCP exclusivo para "Trabajo Nocturno". *Regla de oro: Ollama nunca debe bloquear tu PC mientras la estás usando.*

---

### Cómo Autoinstalarse (Paso a Paso)

1. **El Puente Cloud (Rust)**
   ```
   cargo build --release
   ```
   *(Necesitas tener Rust instalado en tu sistema: https://rustup.rs/)*

   Luego agrega el servidor Rust a tu configuración de Antigravity (`mcp_config.json`), apuntando al binario generado.

2. **Conseguir tus Llaves (API Keys)** – ¡La mayoría son **100 % gratuitas**! Regístrate en los enlaces y copia las llaves en `config.json` dentro de la carpeta `mcp_rust_bridge`.
   - **NVIDIA NIM (Recomendado)**: https://build.nvidia.com/
   - **Moonshot Kimi**: https://platform.kimi.ai/
   - **Groq**: https://console.groq.com/keys
   - **Google Gemini**: https://aistudio.google.com/app/apikey
   - **HuggingFace**: https://huggingface.co/settings/tokens
   - **Cerebras**: https://cloud.cerebras.ai/
   - **SambaNova**: https://cloud.sambanova.ai/
   - **OpenRouter**: https://openrouter.ai/keys (para Claude, GPT‑4o, Inkling, etc.)

---

### El Prompt Mágico para tu IA
Copia el siguiente prompt en la carpeta `.agents/` de tu proyecto Antigravity:

> [!TIP]
> "Eres el Orquestador principal de mi ecosistema Antigravity. Tu objetivo es ayudarme a investigar, programar y aprender. Tienes a tu disposición un hub MCP en Rust que te conecta a 10 cerebros de IA externos (Nvidia NIM, Kimi, Groq, Gemini, OpenRouter, etc.) de forma simultánea y Stateless (SOTA Julio 2026). Si necesitas corroborar una idea arquitectónica compleja, no asumas que lo sabes todo: usa tus herramientas para consultar al Tribunal (ej.: `ask_nvidia` o `ask_kimi`) y contrasta sus respuestas. Para tareas pesadas de noche, cuentas con un sabueso local en Ollama que solo debes despertar cuando yo no use la PC. Tu trabajo conmigo es de co‑work: propone, critica y nunca dudes en decirme directamente si estoy equivocado conceptualmente o matemáticamente."

*Diseñado para uso pedagógico – Versión Agosto 2026.*
