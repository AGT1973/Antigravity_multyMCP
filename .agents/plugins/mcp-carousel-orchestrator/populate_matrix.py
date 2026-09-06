import os
from mcp_carousel_server import register_key_with_profile, generate_reasonability_report

env_path = r"E:\Antigravity_multyMCP_Rust\.env"
print("Poblando matriz de carrusel desde:", env_path)

if os.path.exists(env_path):
    with open(env_path, "r", encoding="utf-8", errors="ignore") as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                key_name, val = line.split("=", 1)
                val = val.strip()
                if val:
                    hub = None
                    if "GROQ" in key_name: hub = "groq"
                    elif "CEREBRAS" in key_name: hub = "cerebras"
                    elif "SAMBANOVA" in key_name: hub = "sambanova"
                    elif "DEEPSEEK" in key_name: hub = "deepseek"
                    elif "GEMINI" in key_name or "GOOGLE" in key_name: hub = "gemini"
                    elif "KIMI" in key_name or "MOONSHOT" in key_name: hub = "kimi"
                    elif "NVIDIA" in key_name or "NVAPI" in key_name: hub = "nvidia"
                    elif "HF" in key_name or "HUGGINGFACE" in key_name: hub = "hf"
                    elif "KAGGLE" in key_name: hub = "kaggle"
                    elif "OPENROUTER" in key_name: hub = "openrouter"
                    
                    if hub:
                        register_key_with_profile(hub, val)

# Kaggle tokens
register_key_with_profile("kaggle", "KGAT_79462dbbf2e5852a00dbaa614644ba83", 72.0)
register_key_with_profile("kaggle", "KGAT_619948d41a0966cb677988951b7c066d", 72.0)

report = generate_reasonability_report()
print("\n[+] REPORTE DE RAZONABILIDAD DE CONSUMO Y CAROUSEL INICIALIZADO:")
for h, d in report["resumen_hubs"].items():
    print(f"  - Hub: {h.upper():<12} | Claves activas: {d['claves_activas']} | Costo: {d['perfil_costo_velocidad']}")
