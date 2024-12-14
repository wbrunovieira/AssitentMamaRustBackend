#!/bin/bash

# Caminho para o diretório onde os arquivos de áudio estão localizados
AUDIO_DIR="/home/wbruno/projects/assistent_mama_backend/voices_marcia/bruno_gym"

# Lista de arquivos de áudio no diretório
AUDIO_FILES=("$AUDIO_DIR"/*.mp3)

# Verifica se existem arquivos de áudio no diretório
if [ ${#AUDIO_FILES[@]} -eq 0 ]; then
  echo "Nenhum arquivo de áudio encontrado no diretório: $AUDIO_DIR"
  exit 1
fi

# Loop infinito para tocar os arquivos de áudio
while true; do
  for FILE in "${AUDIO_FILES[@]}"; do
    echo "Reproduzindo: $FILE"
    mpg123 "$FILE"
    echo "Aguardando 8 minutos..."
    sleep 250 # Espera por 480 segundos (8 minutos)
  done
done

