from __future__ import annotations

import asyncio
import json
import os

try:
    import aiohttp

    _HAS_AIOHTTP = True
except ImportError:  # pragma: no cover - fall back to urllib for zero-dep envs
    _HAS_AIOHTTP = False


class _HttpConduit:
    def __init__(self, api_key: str, model: str, endpoint: str, headers: dict) -> None:
        self._api_key = api_key
        self._model = model
        self._endpoint = endpoint
        self._headers = headers

    async def complete_text(self, prompt: str):
        return await self.complete_messages([{"role": "user", "content": [{"type": "text", "text": prompt}]}])

    async def stream_text(self, prompt: str):
        """Non-streaming fallback — yields the full response as one chunk.

        Override in a subclass for true SSE streaming.
        """
        response = await self.complete_text(prompt)
        yield response.text


class OpenAiConduit(_HttpConduit):
    @classmethod
    def from_env(cls) -> "OpenAiConduit":
        return cls(
            os.environ["OPENAI_API_KEY"],
            os.environ["OPENAI_MODEL"],
            # Uses the OpenAI Responses API (not Chat Completions).
            # Schema: input=[...], response has output=[{content:[{text:...}]}]
            "https://api.openai.com/v1/responses",
            {"Authorization": f"Bearer {os.environ['OPENAI_API_KEY']}"},
        )

    async def complete_messages(self, messages: list[dict]):
        payload = {
            "model": self._model,
            "input": messages,
            "max_output_tokens": 1024,
        }
        return await _complete_http(self._endpoint, payload, self._headers)


class AnthropicConduit(_HttpConduit):
    @classmethod
    def from_env(cls) -> "AnthropicConduit":
        return cls(
            os.environ["ANTHROPIC_API_KEY"],
            os.environ["ANTHROPIC_MODEL"],
            "https://api.anthropic.com/v1/messages",
            {
                "x-api-key": os.environ["ANTHROPIC_API_KEY"],
                "anthropic-version": "2023-06-01",
            },
        )

    async def complete_messages(self, messages: list[dict]):
        payload = {
            "model": self._model,
            "messages": messages,
            "max_tokens": 1024,
        }
        return await _complete_http(self._endpoint, payload, self._headers)


class CompletionResponse:
    def __init__(self, text: str) -> None:
        self.text = text


def _extract_text(body: dict) -> str:
    """Extract text from either OpenAI Responses API or Anthropic Messages API."""
    if "output" in body:
        return "".join(
            item.get("text", "")
            for block in body.get("output", [])
            for item in block.get("content", [])
            if isinstance(item, dict)
        )
    return "".join(
        item.get("text", "")
        for item in body.get("content", [])
        if isinstance(item, dict)
    )


async def _complete_http(endpoint: str, payload: dict, headers: dict) -> CompletionResponse:
    if _HAS_AIOHTTP:
        return await _complete_http_aiohttp(endpoint, payload, headers)
    return await _complete_http_urllib(endpoint, payload, headers)


async def _complete_http_aiohttp(endpoint: str, payload: dict, headers: dict) -> CompletionResponse:
    """True async HTTP using aiohttp — no thread-pool exhaustion under load."""
    async with aiohttp.ClientSession() as session:
        async with session.post(
            endpoint,
            json=payload,
            headers={"Content-Type": "application/json", **headers},
            timeout=aiohttp.ClientTimeout(total=30),
        ) as response:
            response.raise_for_status()
            body = await response.json()
    return CompletionResponse(_extract_text(body))


async def _complete_http_urllib(endpoint: str, payload: dict, headers: dict) -> CompletionResponse:
    """Fallback using urllib in a thread — used when aiohttp is not installed."""
    import urllib.request

    def _request() -> CompletionResponse:
        request = urllib.request.Request(
            endpoint,
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json", **headers},
            method="POST",
        )
        with urllib.request.urlopen(request, timeout=30) as resp:
            body = json.loads(resp.read().decode("utf-8"))
        return CompletionResponse(_extract_text(body))

    return await asyncio.to_thread(_request)
