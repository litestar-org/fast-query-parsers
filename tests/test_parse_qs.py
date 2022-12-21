from urllib.parse import urlencode

from fast_query_parser import parse_qs


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
    }
