#!/bin/bash

call() {
  model_name="$1"
  url="https://generativelanguage.googleapis.com/v1beta/models/${model_name}:generateContent"
  echo "$url"

  curl "${url}" \
    -H "x-goog-api-key: $GEMINI_API_KEY" \
    -H 'Content-Type: application/json' \
    -X POST \
    -d '{
      "contents": [
        {
          "parts": [
            {
              "text": "Explain how AI works in a few words"
            }
          ]
        }
      ]
    }'
}

# call "gemini-3-flash-preview"
call "gemini-2.5-flash"
