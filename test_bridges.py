"""
test_bridges.py - Prueba de "Hola" a cada proveedor
====================================================
Corre antes del release para confirmar que todos los
canales responden correctamente.

Uso: python test_bridges.py
"""

import json
import asyncio
import httpx
import os
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8', errors='replace')

# Leer config desde la compilación release
CONFIG_PATH = Path(__file__).parent / "target" / "release" / "config.json"

with open(CONFIG_PATH, "r") as f:
    CONFIG = json.load(f)

OR_KEY  = CONFIG.get("openrouter_api_key", "")
GROQ    = CONFIG.get("groq_api_key", "")
GEMINI  = CONFIG.get("gemini_api_key", "")
HF      = CONFIG.get("hf_token", "")
CEREBRAS= CONFIG.get("cerebras_api_key", "")
SAMBA   = CONFIG.get("sambanova_api_key", "")
OR_M1   = CONFIG.get("openrouter_model_1", "anthropic/claude-sonnet-4-5")
OR_M2   = CONFIG.get("openrouter_model_2", "thinkingmachines/inkling")
OR_M3   = CONFIG.get("openrouter_model_3", "openai/gpt-4o")

PROMPT  = "Responde solo: 'Hola desde [tu nombre de modelo]'. Nada más."
TIMEOUT = 60.0

RESULTADOS = {}


async def test(nombre, coro):
    print(f"  -> Probando {nombre}...", end=" ", flush=True)
    try:
        res = await asyncio.wait_for(coro, timeout=TIMEOUT)
        texto = str(res)[:120].replace("\n", " ")
        print(f"[OK] {texto}")
        RESULTADOS[nombre] = "OK"
    except asyncio.TimeoutError:
        print(f"[TIMEOUT] ({TIMEOUT}s)")
        RESULTADOS[nombre] = "TIMEOUT"
    except Exception as e:
        print(f"[ERROR] {e}")
        RESULTADOS[nombre] = f"ERROR: {e}"


async def ask_openai_compat(url, key, model, prompt):
    async with httpx.AsyncClient(timeout=TIMEOUT) as c:
        r = await c.post(url,
            headers={"Authorization": f"Bearer {key}",
                     "HTTP-Referer": "https://github.com/AGT1973/Antigravity_multyMCP"},
            json={"model": model, "messages": [{"role": "user", "content": prompt}]})
        r.raise_for_status()
        return r.json()["choices"][0]["message"]["content"]


async def ask_ollama_local(model, prompt):
    async with httpx.AsyncClient(timeout=120) as c:
        r = await c.post("http://localhost:11434/api/chat",
            json={"model": model, "messages": [{"role":"user","content": prompt}], "stream": False})
        r.raise_for_status()
        return r.json()["message"]["content"]


async def main():
    print("\n" + "="*60)
    print("  TEST HOLA - Rust Unified MCP Bridge v2.0 Agosto 2026")
    print("="*60 + "\n")

    tareas = []

    if GROQ:
        tareas.append(test("Groq (llama-3.3-70b)",
            ask_openai_compat("https://api.groq.com/openai/v1/chat/completions",
                              GROQ, "llama-3.3-70b-versatile", PROMPT)))

    if GEMINI:
        tareas.append(test("Gemini (gemini-2.0-flash)",
            ask_openai_compat("https://generativelanguage.googleapis.com/v1beta/openai/chat/completions",
                              GEMINI, "gemini-2.0-flash", PROMPT)))

    if HF:
        tareas.append(test("HuggingFace (Llama-3.3-70B)",
            ask_openai_compat("https://api-inference.huggingface.co/v1/chat/completions",
                              HF, "meta-llama/Llama-3.3-70B-Instruct", PROMPT)))

    if CEREBRAS:
        tareas.append(test("Cerebras (llama-3.3-70b)",
            ask_openai_compat("https://api.cerebras.ai/v1/chat/completions",
                              CEREBRAS, "llama-3.3-70b", PROMPT)))

    if SAMBA:
        tareas.append(test("SambaNova (Llama-3.3-70B)",
            ask_openai_compat("https://api.sambanova.ai/v1/chat/completions",
                              SAMBA, "Meta-Llama-3.3-70B-Instruct", PROMPT)))

    if OR_KEY:
        or_url = "https://openrouter.ai/api/v1/chat/completions"
        tareas.append(test(f"OpenRouter Slot 1 ({OR_M1})",
            ask_openai_compat(or_url, OR_KEY, OR_M1, PROMPT)))
        tareas.append(test(f"OpenRouter Slot 2 ({OR_M2})",
            ask_openai_compat(or_url, OR_KEY, OR_M2, PROMPT)))
        tareas.append(test(f"OpenRouter Slot 3 ({OR_M3})",
            ask_openai_compat(or_url, OR_KEY, OR_M3, PROMPT)))

    # Ollama: solo si está corriendo
    tareas.append(test("Ollama (deepseek-r1:14b)",
        ask_ollama_local("deepseek-r1:14b", PROMPT)))

    await asyncio.gather(*tareas)

    # Resumen
    print("\n" + "="*60)
    print("  RESUMEN")
    print("="*60)
    ok  = [k for k, v in RESULTADOS.items() if v == "OK"]
    err = [k for k, v in RESULTADOS.items() if v != "OK"]
    print(f"  ✅ OK:    {len(ok)}")
    print(f"  ❌ Falló: {len(err)}")
    if err:
        for e in err:
            print(f"     • {e}: {RESULTADOS[e]}")
    print()
    if len(err) == 0:
        print("  [LISTO] Todo OK - Compilar y hacer push a GitHub!")
    elif all("Ollama" in e for e in err):
        print("  [NOCTURNO] Solo Ollama offline (esperado). El resto OK.")
        print("  [LISTO] Compilar y hacer push a GitHub!")
    else:
        print("  [AVISO] Hay proveedores cloud con fallas. Revisa las API keys.")


if __name__ == "__main__":
    asyncio.run(main())
