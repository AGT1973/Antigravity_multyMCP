import os, shutil, zipfile

plugin_dir = r"C:\Users\eluithi\.gemini\config\plugins\mcp-carousel-orchestrator"
zip_path = r"E:\Antigravity_multyMCP_Rust\mcp-carousel-orchestrator.zip"

with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
    for root, dirs, files in os.walk(plugin_dir):
        for file in files:
            full_path = os.path.join(root, file)
            rel_path = os.path.relpath(full_path, plugin_dir)
            zipf.write(full_path, rel_path)

print(f"[OK] Plugin empaquetado exitosamente como ZIP portable en: {zip_path}")
