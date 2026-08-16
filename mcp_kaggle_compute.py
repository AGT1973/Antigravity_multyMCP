#!/usr/bin/env python3
"""
MCP Kaggle Compute v1.0
========================
Servidor MCP para enviar, monitorear y recolectar benchmarks en GPU y TPU de Kaggle.
"""

import os
import json
import subprocess
from mcp.server.fastmcp import FastMCP

server = FastMCP("Antigravity-Kaggle-Compute")

KAGGLE_DIR = os.path.expanduser("~/.kaggle")
os.makedirs(KAGGLE_DIR, exist_ok=True)

@server.tool(name="kaggle_check_auth")
def kaggle_check_auth() -> str:
    """Verifica si las credenciales de Kaggle están configuradas."""
    token_path = os.path.join(KAGGLE_DIR, "access_token")
    json_path = os.path.join(KAGGLE_DIR, "kaggle.json")
    if os.path.exists(token_path) or os.path.exists(json_path):
        return "[OK] Credenciales encontradas en .kaggle"
    return "[ERROR] No se encontro token de Kaggle."

@server.tool(name="kaggle_set_credentials")
def kaggle_set_credentials(token: str) -> str:
    """Configura el token de acceso oficial de Kaggle (KGAT_...)."""
    token_path = os.path.join(KAGGLE_DIR, "access_token")
    with open(token_path, "w", encoding="utf-8") as f:
        f.write(token.strip())
    return "[OK] Archivo access_token creado exitosamente."

@server.tool(name="kaggle_run_benchmark")
def kaggle_run_benchmark(username: str, accelerator: str = "gpu") -> str:
    """
    Empuja y ejecuta el benchmark de POLYDIM en Kaggle.
    username: Tu nombre de usuario en Kaggle (para el kernel id)
    accelerator: 'gpu' (2x NVIDIA T4) o 'tpu' (TPU v3-8)
    """
    bench_script = r"E:\POLYDIM_EINSOF\benchmark_polydim_hardware.py"
    if not os.path.exists(bench_script):
        return f"[ERROR] No se encontro el script {bench_script}"

    work_dir = r"E:\POLYDIM_EINSOF\kaggle_kernel"
    os.makedirs(work_dir, exist_ok=True)

    with open(bench_script, "r", encoding="utf-8") as f:
        code = f.read()
    with open(os.path.join(work_dir, "benchmark.py"), "w", encoding="utf-8") as f:
        f.write(code)

    metadata = {
        "id": f"{username}/polydim-hardware-benchmark-{accelerator.lower()}",
        "title": f"POLYDIM Hardware Benchmark {accelerator.upper()}",
        "code_file": "benchmark.py",
        "language": "python",
        "kernel_type": "script",
        "is_private": "true",
        "enable_gpu": "true" if accelerator.lower() == "gpu" else "false",
        "enable_tpu": "true" if accelerator.lower() == "tpu" else "false",
        "enable_internet": "true"
    }

    meta_file = os.path.join(work_dir, "kernel-metadata.json")
    with open(meta_file, "w", encoding="utf-8") as f:
        json.dump(metadata, f, indent=2)

    try:
        env = os.environ.copy()
        env["PYTHONIOENCODING"] = "utf-8"
        res = subprocess.run([r"C:\python314\python.exe", "-m", "kaggle", "kernels", "push", "-p", work_dir], capture_output=True, text=True, check=True, env=env)
        return f"[OK] Benchmark empujado a Kaggle ({accelerator.upper()}):\n{res.stdout}"
    except Exception as e:
        return f"[ERROR] Error al enviar a Kaggle: {e}"

@server.tool(name="kaggle_get_results")
def kaggle_get_results(kernel_name: str) -> str:
    """Descarga los logs y resultados generados por el benchmark en Kaggle."""
    out_dir = r"E:\POLYDIM_EINSOF\eval_logs\kaggle_results"
    os.makedirs(out_dir, exist_ok=True)
    try:
        env = os.environ.copy()
        env["PYTHONIOENCODING"] = "utf-8"
        res = subprocess.run(["kaggle", "kernels", "output", kernel_name, "-p", out_dir], capture_output=True, text=True, check=True, env=env)
        return f"[OK] Resultados descargados en {out_dir}:\n{res.stdout}"
    except Exception as e:
        return f"[ERROR] Error al descargar resultados: {e}"

if __name__ == "__main__":
    server.run(transport="stdio")
