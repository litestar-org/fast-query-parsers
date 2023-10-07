=====
Usage
=====

Installation
------------

.. code-block:: console

   pip install fast-query-parsers

Usage
-----

The library exposes two function ``parse_query_string`` and ``parse_url_encoded_dict``.

``parse_query_string``
~~~~~~~~~~~~~~~~~~~~~~

This function is used to parse a query string into a list of key/value tuples.

.. code-block:: python

   from fast_query_parsers import parse_query_string

   result = parse_query_string(b"value=1&value=2&type=dollar&country=US", "&")
   # [("value", "1"), ("value", "2"), ("type", "dollar"), ("country", "US")]

Benchmarks
^^^^^^^^^^

Query string parsing is more than x5 times faster than the standard library:

.. code-block:: console

   stdlib parse_qsl parsing query string: Mean +- std dev: 2.86 us +- 0.03 us
   .....................
   parse_query_string parsing query string: Mean +- std dev: 916 ns +- 13 ns
   .....................
   stdlib parse_qsl parsing urlencoded query string: Mean +- std dev: 8.30 us +- 0.10 us
   .....................
   parse_query_string urlencoded query string: Mean +- std dev: 1.50 us +- 0.03 us

``parse_url_encoded_dict``
~~~~~~~~~~~~~~~~~~~~~~~~~~

This function is used to parse a url-encoded form data dictionary and parse it into the python equivalent of JSON types.

.. code-block:: python

   from urllib.parse import urlencode
   from fast_query_parsers import parse_url_encoded_dict

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
       ]
   ).encode()

   result = parse_url_encoded_dict(encoded, parse_numbers=True)
   # result == { ... }
