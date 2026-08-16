@echo off
echo ==============================================================================
echo             POLYDIM MCP HUB - GESTOR DE API KEYS (ALUMNOS)
echo ==============================================================================
echo.
echo Para que el Enjambre de Inteligencia Artificial (Antigravity/Claude) funcione
echo a maxima velocidad (Modo Carrusel) sin bloqueos por Rate Limit, necesitas
echo obtener API Keys gratuitas de multiples proveedores.
echo.
echo Presiona cualquier tecla para abrir las paginas de registro en tu navegador...
pause >nul

echo Abriendo OpenRouter...
start https://openrouter.ai/keys

echo Abriendo Groq...
start https://console.groq.com/keys

echo Abriendo Cerebras...
start https://inference.cerebras.ai/

echo Abriendo SambaNova...
start https://cloud.sambanova.ai/

echo Abriendo HuggingFace...
start https://huggingface.co/settings/tokens

echo Abriendo Moonshot (Kimi)...
start https://platform.moonshot.cn/console/api-keys

echo Abriendo Google Gemini...
start https://aistudio.google.com/app/apikey

echo Abriendo Nvidia NIM...
start https://build.nvidia.com/explore/discover

echo Abriendo Together AI...
start https://api.together.xyz/settings/api-keys

echo Abriendo Mistral AI...
start https://console.mistral.ai/api-keys/

echo ==============================================================================
echo Instrucciones:
echo 1. Crea una cuenta en cada uno de estos proveedores.
echo 2. Genera una API Key en cada plataforma.
echo 3. Entregaselas a Antigravity (el agente) pegandolas en el chat.
echo 4. El agente se encargara de guardarlas y rotarlas automaticamente en el Carrusel.
echo ==============================================================================
pause
