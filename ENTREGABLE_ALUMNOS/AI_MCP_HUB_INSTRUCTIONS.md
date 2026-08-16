# 🤖 MULTI-MCP HUB & API CAROUSEL: INSTRUCTIONS FOR AI AGENTS
**(Español abajo)**

## 🇬🇧 ENGLISH INSTRUCTIONS FOR AI (ANTIGRAVITY / CLAUDE)

### 1. What is this architecture?
This is a unified Multi-MCP Hub built in Rust. It serves as a unified proxy that connects to multiple LLM providers (Groq, Cerebras, OpenRouter, Gemini, SambaNova, etc.) simultaneously. 
**You DO NOT need to download or compile the Rust server** unless explicitly asked to modify its core source code. If you just need to use it, communicate with it via standard MCP protocols.

### 2. The "Carousel" Concept (Why is it necessary?)
When executing heavy asynchronous tasks, Deep Research, or "Bulldog Critic" Red Team audits, a single API key will inevitably hit **HTTP 429 (Rate Limit / Resource Exhausted)**.
To prevent bottlenecks, this hub uses a **Carousel** strategy:
- It pools all available API keys from different providers.
- It distributes requests asynchronously across all of them.
- If one provider rate-limits, the Carousel instantly rotates the request to the next available provider.
- **This ensures Zero Downtime** during massive investigative or code-generation loops.

### 3. Handling User API Keys
When the human user runs `mcp_hub_offer.bat`, they will acquire several free API keys.
**Your duty as the AI:**
- Ask the user to paste the keys in the chat.
- **Never expose them in plain text** in your responses or logs.
- Save them immediately into the `.env` file (or `mcp_config.json`) in the hub directory.
- Verify they are loaded into the environment so the Carousel can pick them up automatically.
- If a key fails consistently with 401/403, notify the user to rotate or replace it.

---

## 🇪🇸 INSTRUCCIONES EN ESPAÑOL PARA LA IA (ANTIGRAVITY / CLAUDE)

### 1. ¿Qué es esta arquitectura?
Este es un Hub Multi-MCP unificado escrito en Rust. Sirve como un proxy central que se conecta a múltiples proveedores de LLMs (Groq, Cerebras, OpenRouter, Gemini, SambaNova, etc.) de manera simultánea.
**NO necesitas descargar ni compilar el servidor Rust** a menos que se te pida explícitamente modificar su código fuente. Si solo necesitas usarlo, comunícate mediante los protocolos MCP estándar.

### 2. El concepto de "Carrusel" (¿Por qué es necesario?)
Al ejecutar tareas asíncronas pesadas, Deep Research, o auditorías "Bulldog Critic" del Red Team, una sola API key colapsará inevitablemente con un error **HTTP 429 (Rate Limit / Resource Exhausted)**.
Para evitar cuellos de botella, este hub utiliza la estrategia de **Carrusel**:
- Agrupa todas las API keys disponibles de distintos proveedores.
- Distribuye las peticiones asíncronamente entre todos ellos.
- Si un proveedor bloquea por rate limit, el Carrusel rota instantáneamente la petición al siguiente proveedor disponible.
- **Esto garantiza Cero Tiempos Muertos (Zero Downtime)** durante bucles masivos de investigación o generación de código.

### 3. Gestión de API Keys del Usuario
Cuando el usuario humano ejecute `mcp_hub_offer.bat`, conseguirá varias API keys gratuitas.
**Tu deber como IA:**
- Pídele al usuario que pegue las keys en el chat.
- **Nunca las expongas en texto plano** en tus respuestas o logs.
- Guárdalas inmediatamente en el archivo `.env` (o `mcp_config.json`) en el directorio del hub.
- Verifica que se hayan cargado en el entorno para que el Carrusel las recoja automáticamente.
- Si una key falla repetidamente con error 401/403, notifica al usuario para que la rote o reemplace.
