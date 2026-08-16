#!/usr/bin/env python3
"""
MCP Colab Live Bridge v1.0
===========================
Servidor MCP para comunicarse en tiempo real con una GPU/TPU de Google Colab
mediante un túnel Cloudflare o ngrok.
"""

import httpx
from mcp.server.fastmcp import FastMCP

server = FastMCP("Antigravity-Colab-Live")

# URL del túnel activo
COLAB_TUNNEL_URL = "http://localhost:8000"
TIMEOUT = 60.0

@server.tool(name="colab_set_tunnel")
def colab_set_tunnel(url: str) -> str:
    """Configura la URL pública del túnel Cloudflare o ngrok de Google Colab."""
    global COLAB_TUNNEL_URL
    COLAB_TUNNEL_URL = url.rstrip("/")
    return f"✅ URL del túnel de Colab actualizada a: {COLAB_TUNNEL_URL}"

@server.tool(name="colab_ping_hardware")
async def colab_ping_hardware() -> str:
    """Consulta la GPU/TPU activa y la memoria disponible en el runtime de Colab."""
    async with httpx.AsyncClient(timeout=TIMEOUT) as client:
        try:
            resp = await client.get(f"{COLAB_TUNNEL_URL}/hardware")
            resp.raise_for_status()
            return f"🟢 Conexión con Colab Exitosa:\n{resp.text}"
        except Exception as e:
            return f"❌ No se pudo conectar con Colab en {COLAB_TUNNEL_URL}: {e}"

@server.tool(name="colab_eval_code")
async def colab_eval_code(python_code: str) -> str:
    """Ejecuta código de tensores / álgebra geométrica en la GPU/TPU de Colab en tiempo real."""
    async with httpx.AsyncClient(timeout=TIMEOUT) as client:
        try:
            resp = await client.post(
                f"{COLAB_TUNNEL_URL}/eval",
                json={"code": python_code}
            )
            resp.raise_for_status()
            return f"📊 Resultado de Ejecución en Colab:\n{resp.json().get('output', '')}"
        except Exception as e:
            return f"❌ Error de ejecución en Colab: {e}"

if __name__ == "__main__":
    server.run(transport="stdio")
