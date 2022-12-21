from urllib.parse import urlencode

from starlite_http_utils import parse_qs


def test_parse_urlencoded() -> None:
    result = parse_qs(
        urlencode(
            [
                ("value", "10"),
                ("value", "12"),
                ("veggies", '["tomato", "potato", "aubergine"]'),
                ("nested", '{"some_key": "some_value"}'),
                ("calories", "122.53"),
                ("healthy", "true"),
                ("polluting", "false"),
                ("json", "null"),
                ("arabic_text", "اختبار اللغة العربية")
            ]
        ).encode()
    )
    assert result == {
        "value": [10, 12],
        "veggies": ["tomato", "potato", "aubergine"],
        "nested": {"some_key": "some_value"},
        "calories": 122.53,
        "healthy": True,
        "polluting": False,
        "json": None,
        "arabic_text":  "اختبار اللغة العربية",
    }
