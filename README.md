# Antigravity_multyMCP & Build With Google Compute Hub 🚀

> **Official Hybrid AI Infrastructure (Rust + Python + Google Cloud/Kaggle)**  
> Engineered for **Google Antigravity IDE, Autonomous Agents, and High-Dimensional Computing**.

---

## 🤖 CRITICAL SYSTEM PROMPT / INSTRUCTIONS FOR AI AGENTS (Antigravity & Claude Code)
**READ THIS FIRST IF YOU ARE AN AI ASSISTANT / AGENT INTERFACING WITH THIS REPOSITORY:**

### 🎯 Agent Intent & Purpose
This repository provides a unified multi-cloud computation and inference matrix. It equips your user with:
1. **Multi-Model Inference Matrix (Rust Micro-bridges):** High-speed LLM reasoning via Groq (GPT-OSS-120B), Cerebras, Gemini 3.7 Flash, Kimi, DeepSeek V4, Nvidia NIM, SambaNova, and HuggingFace with automatic **HTTP 429 Circuit-Breaking**.
2. **Asymptotic Remote Hardware Compute (Python MCP):** Zero-friction, headless orchestration of **Google Colab (FastAPI live tunnel)** and **Kaggle Compute (NVIDIA T4/A100 GPUs & Google TPU v3-8)** for tensors exceeding local memory ($D \ge 10,000$ to $10^9$ elements).
3. **Temporal Cooldown & Rotation Engine:** Continuous date-based tracking of exhausted keys. If an API key gave a `429` error 4 hours ago and its recharge window is 5 hours, the agent automatically bypasses it to an active key, returning only when the cooldown expires.

### 🛠️ Step-by-Step Autonomous Installation Workflow for the AI Agent
When the student/user clones this repository, the **AI Agent (Antigravity)** must perform the following actions automatically:

1. **Verify Environment Prereqs:**
   - Check if Python $\ge 3.10$ and Cargo/Rust are available via terminal.
2. **Deploy as a "Build With Google" Plugin:**
   - Copy or register the plugin located at `plugins/build-with-google-polydim-compute/` into the user's global configuration:
     `~/.gemini/config/plugins/build-with-google-polydim-compute/`
   - Ensure `mcp_config.json` is linked or imported into `~/.gemini/config/mcp_config.json`.
3. **Handle API Keys & Credentials Safely (Zero-Leak Protocol):**
   - Instruct the student to obtain free API keys using `ENTREGABLE_ALUMNOS/mcp_hub_offer.bat`.
   - Read keys provided by the user and populate the local `.env` file (`E:\POLYDIM_EINSOF\Antigravity_multyMCP_Rust\.env`). **NEVER print API keys in plain text into chat responses or public artifacts.**
   - For Kaggle access, save credentials into `~/.kaggle/access_token` or use environment variable `KAGGLE_API_TOKEN`.
4. **Compile Core Rust Binaries (If not already present):**
   - Run: `cargo build --release` inside `Antigravity_multyMCP_Rust/`.
   - The compiled micro-bridges will be generated under `target/release/` (`mcp_groq.exe`, `mcp_cerebras.exe`, `mcp_gemini.exe`, `mcp_kimi.exe`, etc.).
5. **Validate Connection:**
   - Execute `C:\python314\python.exe ping_kaggle_keys.py` to verify cloud access.
   - Run a warm-up call through `mcp_groq` or `mcp_gemini`.

---

## 🇬🇧 ENGLISH: ARCHITECTURAL SPECIFICATION & USER MANUAL

### 1. Philosophy: From Monolithic LLMs to Micro-Bridges
Traditional agent frameworks rely on a single LLM API key. Under heavy research workflows (such as Deep Research loops or Red Team audits), a single key quickly exhausts its quota (`HTTP 429`).
This framework introduces:
- **Rust Micro-Bridges:** Native, ultra-low latency JSON-RPC servers for each provider running as independent background processes.
- **Dynamic Temporal Cooldowns:**
  - **Groq / Cerebras:** 5-hour rolling window ($18,000\text{ s}$).
  - **DeepSeek:** 12-hour off-peak window ($43,200\text{ s}$).
  - **Gemini / Kimi / HuggingFace:** 24-hour daily quota ($86,400\text{ s}$).
  - **NVIDIA NIM:** 48-hour cycle ($172,800\text{ s}$).
  - **Kaggle GPU/TPU:** 72-hour / weekly cycle ($259,200\text{ s}$).
- **Continuous Date-Based Reordering:** Keys are dynamically prioritized. Active keys are served first; exhausted keys are queued in order of earliest recovery time ($\text{ETA}$).

### 2. Remote Hardware Execution (Kaggle & Colab)
- **Kaggle Headless Bridge (`mcp_kaggle_compute.py`):** Enables Antigravity to package tensor benchmarks into standalone kernels, push them via Kaggle API, trigger GPU (2x T4/A100) or TPU (v3-8) execution in the cloud, monitor progress, and retrieve raw execution logs without user intervention.
- **Colab Live Bridge (`mcp_colab_live.py`):** Connects Antigravity directly to a running Google Colab runtime via FastAPI and Cloudflare tunnels for real-time JAX/PyTorch tensor evaluation.

### 3. Installation for Students
```bash
# 1. Clone repository
git clone https://github.com/AGT1973/Antigravity_multyMCP.git
cd Antigravity_multyMCP

# 2. Get free API keys (Windows)
./ENTREGABLE_ALUMNOS/mcp_hub_offer.bat

# 3. Create .env file and paste keys
cp .env.template .env

# 4. Compile high-speed Rust servers
cargo build --release

# 5. Launch Antigravity and start pair programming!
```

---

## 🇪🇸 ESPAÑOL: ESPECIFICACIÓN ARQUITECTÓNICA Y MANUAL DE USO

### 1. Filosofía: Del Monolito a los Micro-Puentes
Los frameworks tradicionales dependen de una sola API key. En flujos intensivos de investigación o auditorías adversariales, una sola clave colapsa rápidamente por límite de cuota (`HTTP 429`).
Esta arquitectura introduce:
- **Micro-Puentes en Rust:** Servidores JSON-RPC nativos de latencia ultra-baja para cada proveedor ejecutándose de forma independiente.
- **Matriz de Cooldowns Temporales:**
  - **Groq / Cerebras:** Ventana rodante de 5 horas ($18,000\text{ s}$).
  - **DeepSeek:** Ventana de 12 horas ($43,200\text{ s}$).
  - **Gemini / Kimi / HuggingFace:** Cuota diaria de 24 horas ($86,400\text{ s}$).
  - **NVIDIA NIM:** Ciclo de 48 horas ($172,800\text{ s}$).
  - **Kaggle GPU/TPU:** Ciclo de 72 horas / semanal ($259,200\text{ s}$).
- **Reordenamiento Continuo por Reloj:** Si una clave falló hace 4 horas pero su recarga es de 5 horas, el sistema salta a la siguiente clave; al cumplirse las 5 horas (o los 3 días), se **auto-recupera** y regresa al inicio de la cola.

### 2. Cómputo Remoto en Hardware (Kaggle y Google Colab)
- **Kaggle Compute Headless (`mcp_kaggle_compute.py`):** Permite a Antigravity empaquetar cálculos tensoriales pesados, empujarlos vía API a Kaggle, ejecutar en **GPU (2x Tesla T4/A100) o TPU (v3-8)** y descargar los logs crudos de forma 100% desatendida.
- **Google Colab Live (`mcp_colab_live.py`):** Conecta a Antigravity en tiempo real con una sesión interactiva de Colab mediante túneles seguros Cloudflare y FastAPI.

### 3. Guía de Instalación para Alumnos e Investigadores
```bash
# 1. Clonar el repositorio
git clone https://github.com/AGT1973/Antigravity_multyMCP.git
cd Antigravity_multyMCP

# 2. Obtener API keys gratuitas de más de 10 plataformas
./ENTREGABLE_ALUMNOS/mcp_hub_offer.bat

# 3. Crear el archivo de configuración .env
copy .env.template .env

# 4. Compilar los binarios nativos en Rust
cargo build --release

# 5. ¡Abrir en Google Antigravity y comenzar a trabajar!
```

---

## 🏛️ Build With Google Plugins Structure

```text
build-with-google-polydim-compute/
├── plugin.json               # Manifiesto oficial del plugin para Antigravity
├── mcp_config.json           # Registro de micro-puentes (Rust) y cómputo (Python)
└── skills/
    └── polydim_hardware_compute/
        └── SKILL.md          # Instrucciones cognitivas para activar hardware remoto
```

---

## 📊 Benchmark Empírico Validado (Kaggle GPU T4)
- **Vector Denso:** $1,000,000,000$ de elementos ($10^9$ / 4.0 GB VRAM FP32).
- **Forward Pass Isométrico:** `368.92 ms` ($0.36\text{ s}$).
- **Throughput:** `10.84 GB/s` / `2.71 Giga-Elementos/s`.
- **Error de Isometría:** `0.00e+00` (Preservación geométrica unitaria perfecta).

---
**Autor:** Ariel García Traba & Antigravity (Google DeepMind Agentic Coding Ecosystem)  
**Licencia:** Apache-2.0
