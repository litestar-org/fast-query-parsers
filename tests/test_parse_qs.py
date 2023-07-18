from urllib.parse import urlencode

from fast_query_parsers import parse_url_encoded_dict


def test_parse_urlencoded_with_parse_numbers() -> None:
    encoded = urlencode(
        [
            ("value", "10"),
            ("value", "12"),
            ("veggies", '["tomato", "potato", "aubergine"]'),
            ("nested", '{"some_key": "some_value"}'),
            ("calories", "122.53"),
            ("healthy", "true"),
            ("polluting", "false"),
            ("json", "null"),
        ],
    ).encode()
    result = parse_url_encoded_dict(encoded, True)
    assert result == {
        "value": [10, 12],
        "veggies": ["tomato", "potato", "aubergine"],
        "nested": {"some_key": "some_value"},
        "calories": 122.53,
        "healthy": True,
        "polluting": False,
        "json": None,
    }


def test_parse_urlencoded_without_parse_numbers() -> None:
    encoded = urlencode(
        [
            ("value", "10"),
            ("value", "12"),
            ("veggies", '["tomato", "potato", "aubergine"]'),
            ("nested", '{"some_key": "some_value"}'),
            ("calories", "122.53"),
            ("healthy", "true"),
            ("polluting", "false"),
            ("json", "null"),
        ],
    ).encode()
    result = parse_url_encoded_dict(encoded, False)
    assert result == {
        "value": ["10", "12"],
        "veggies": ["tomato", "potato", "aubergine"],
        "nested": {"some_key": "some_value"},
        "calories": "122.53",
        "healthy": True,
        "polluting": False,
        "json": None,
    }
