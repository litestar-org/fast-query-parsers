.. image:: https://raw.githubusercontent.com/litestar-org/branding/473f54621e55cde9acbb6fcab7fc03036173eb3d/assets/Branding%20-%20SVG%20-%20Transparent/Logo%20-%20Banner%20-%20Inline%20-%20Light.svg#gh-light-mode-only
   :align: center

.. .. image:: https://raw.githubusercontent.com/litestar-org/branding/473f54621e55cde9acbb6fcab7fc03036173eb3d/assets/Branding%20-%20SVG%20-%20Transparent/Logo%20-%20Banner%20-%20Inline%20-%20Dark.svg#gh-dark-mode-only
..    :align: center
..    :class: dark_logo

+-----------+-----+------------------------------------------------------------------------------------------------------------------+
| Project   |     | Status                                                                                                           |
+===========+=====+==================================================================================================================+
| CI/CD     |     | |Publish| |CI|                                                                                                   |
+-----------+-----+------------------------------------------------------------------------------------------------------------------+
| Package   |     | |PyPI - Version| |PyPI - Support Python Versions| |PyPI - Downloads|                                             |
+-----------+-----+------------------------------------------------------------------------------------------------------------------+
| Community |     | |Reddit| |Discord| |Matrix| |Medium| |Twitter| |Blog|                                                            |
+-----------+-----+------------------------------------------------------------------------------------------------------------------+
| Meta      |     | |Litestar Project| |License - MIT| |Litestar Sponsors| |linting - Ruff| |code style - Black|                     |
+-----------+-----+------------------------------------------------------------------------------------------------------------------+

==================
Fast Query Parsers
==================

This library includes ultra-fast Rust based query string and ``urlencoded`` parsers.
These parsers can be used in any project that needs to parse query strings or ``urlencoded`` data.

Who's using this library?
-------------------------

These parsers were initially built to used by `Litestar <https://github.com/litestar-org/litestar>`_,
a high-performance Python web framework. However, these parsers can be used in any project that
needs to parse query strings or ``urlencoded`` data.

Benchmarks
----------

.. code-block:: console

   stdlib parse_qs parsing url-encoded values into dict:
   Mean +- std dev: 8.99 us +- 0.09 us
   .....................
   parse_url_encoded_dict parse url-encoded values into dict:
   Mean +- std dev: 3.77 us +- 0.08 us

Contributing
------------

Contributions are welcome!

Please see the
`contribution guide <https://litestar-org.github.com/fast-query-parsers/contributing.html>`_
for more details.


.. |Publish| image:: https://github.com/litestar-org/fast-query-parsers/actions/workflows/publish.yaml/badge.svg
   :target: https://github.com/litestar-org/fast-query-parsers/actions/workflows/publish.yaml
.. |CI| image:: https://github.com/litestar-org/fast-query-parsers/actions/workflows/ci.yaml/badge.svg
   :target: https://github.com/litestar-org/fast-query-parsers/actions/workflows/ci.yaml
.. |PyPI - Version| image:: https://img.shields.io/pypi/v/fast-query-parsers?labelColor=202235&color=edb641&logo=python&logoColor=edb641
   :target: https://badge.fury.io/py/litestar
.. |PyPI - Support Python Versions| image:: https://img.shields.io/pypi/pyversions/fast-query-parsers?labelColor=202235&color=edb641&logo=python&logoColor=edb641
.. |PyPI - Downloads| image:: https://img.shields.io/pypi/dm/fast-query-parsers?logo=python&label=fast-query-parsers%20downloads&labelColor=202235&color=edb641&logoColor=edb641
.. |Reddit| image:: https://img.shields.io/reddit/subreddit-subscribers/litestarapi?label=r%2FLitestar&logo=reddit&labelColor=202235&color=edb641&logoColor=edb641
   :target: https://reddit.com/r/litestarapi
.. |Discord| image:: https://img.shields.io/discord/919193495116337154?labelColor=202235&color=edb641&label=chat%20on%20discord&logo=discord&logoColor=edb641
   :target: https://discord.gg/X3FJqy8d2j
.. |Matrix| image:: https://img.shields.io/badge/chat%20on%20Matrix-bridged-202235?labelColor=202235&color=edb641&logo=matrix&logoColor=edb641
   :target: https://matrix.to/#/#litestar:matrix.org
.. |Medium| image:: https://img.shields.io/badge/Medium-202235?labelColor=202235&color=edb641&logo=medium&logoColor=edb641
   :target: https://blog.litestar.dev
.. |Twitter| image:: https://img.shields.io/twitter/follow/LitestarAPI?labelColor=202235&color=edb641&logo=twitter&logoColor=edb641&style=flat
   :target: https://twitter.com/LitestarAPI
.. |Blog| image:: https://img.shields.io/badge/Blog-litestar.dev-202235?logo=blogger&labelColor=202235&color=edb641&logoColor=edb641
   :target: https://blog.litestar.dev
.. |Litestar Project| image:: https://img.shields.io/badge/Litestar%20Org-%E2%AD%90%20Fast%20Query%20Parsers-202235.svg?logo=python&labelColor=202235&color=edb641&logoColor=edb641
   :target: https://github.com/litestar-org/fast-query-parsers
.. |License - MIT| image:: https://img.shields.io/badge/license-MIT-202235.svg?logo=python&labelColor=202235&color=edb641&logoColor=edb641
   :target: https://spdx.org/licenses/
.. |Litestar Sponsors| image:: https://img.shields.io/badge/Sponsor-%E2%9D%A4-%23edb641.svg?&logo=github&logoColor=edb641&labelColor=202235
   :target: https://github.com/sponsors/litestar-org
.. |linting - Ruff| image:: https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/charliermarsh/ruff/main/assets/badge/v2.json&labelColor=202235
   :target: https://github.com/astral-sh/ruff
.. |code style - Black| image:: https://img.shields.io/badge/code%20style-black-000000.svg?logo=python&labelColor=202235&logoColor=edb641
   :target: https://github.com/psf/black
