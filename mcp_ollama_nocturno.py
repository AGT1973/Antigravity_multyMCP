#!/usr/bin/env python3
"""
MCP Ollama Nocturno v1.0 - Agosto 2026
=======================================
Servidor MCP SEPARADO e independiente para Ollama local.
Diseñado para trabajo nocturno o cuando no se necesita la PC.

VENTAJA: Al estar separado del bridge principal (Rust),
si Ollama no está corriendo, SOLO este canal falla.
El resto de los hubs (Groq, Gemini, OpenRouter, etc.) 
siguen operando normalmente sin verse afectados.

Uso: Se agrega en mcp_config.json como "mcp-ollama-nocturno"
     y se activa/desactiva manualmente según necesidad.

pip install mcp httpx
"""

import asyncio
import httpx
from mcp.server.fastmcp import FastMCP

server = FastMCP("Antigravity-Ollama-Nocturno")

OLLAMA_URL = "http://localhost:11434"
TIMEOUT    = 300.0  # Los modelos locales son lentos

# Modelo por defecto para investigación y razonamiento
DEFAULT_MODEL = "deepseek-r1:14b"


async def _ask_ollama(prompt: str, model: str, sistema: str = "") -> str:
    """Función base que llama a Ollama."""
    messages = []
    if sistema:
        messages.append({"role": "system", "content": sistema})
    messages.append({"role": "user", "content": prompt})

    async with httpx.AsyncClient(timeout=TIMEOUT) as client:
        try:
            resp = await client.post(
                f"{OLLAMA_URL}/api/chat",
                json={"model": model, "messages": messages, "stream": False}
            )
            resp.raise_for_status()
            data = resp.json()
            return data["message"]["content"]
        except httpx.ConnectError:
            return "❌ Ollama no está corriendo. Ejecuta: 'ollama serve' en una terminal."
        except Exception as e:
            return f"❌ Error Ollama: {e}"


@server.tool(name="ask_ollama")
async def ask_ollama(
    mensaje: str,
    modelo: str = DEFAULT_MODEL,
    sistema: str = ""
) -> str:
    """
    Ollama · Modelos locales para modo nocturno.
    Modelos recomendados: deepseek-r1:14b, llama3.1:8b, qwen2.5:14b
    """
    return await _ask_ollama(mensaje, modelo, sistema)


@server.tool(name="ollama_resumir")
async def ollama_resumir(texto: str, modelo: str = DEFAULT_MODEL) -> str:
    """Ollama · Resume un texto largo usando el modelo local."""
    prompt = f"Resume el siguiente texto de forma concisa y clara:\n\n{texto}"
    return await _ask_ollama(prompt, modelo)


@server.tool(name="ollama_investigar")
async def ollama_investigar(tema: str, modelo: str = DEFAULT_MODEL) -> str:
    """Ollama · Razona sobre un tema y genera un análisis detallado (modo sabueso)."""
    prompt = (
        f"Analiza en profundidad el siguiente tema. Organiza tu respuesta con: "
        f"1) Conceptos clave, 2) Estado del arte, 3) Problemas abiertos, "
        f"4) Conexiones con otras áreas.\n\nTema: {tema}"
    )
    return await _ask_ollama(prompt, modelo, "Eres un investigador científico experto.")


@server.tool(name="ollama_modelos")
async def ollama_modelos() -> str:
    """Lista los modelos descargados en Ollama local."""
    async with httpx.AsyncClient(timeout=10) as client:
        try:
            resp = await client.get(f"{OLLAMA_URL}/api/tags")
            resp.raise_for_status()
            modelos = [m["name"] for m in resp.json().get("models", [])]
            if not modelos:
                return "No hay modelos descargados. Usa: ollama pull deepseek-r1:14b"
            return "Modelos disponibles:\n" + "\n".join(f"  • {m}" for m in modelos)
        except Exception:
            return "❌ Ollama no está corriendo. Ejecuta: 'ollama serve'"


if __name__ == "__main__":
    server.run(transport="stdio")
