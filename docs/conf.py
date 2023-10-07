from __future__ import annotations

project = "Fast Query Parsers"
copyright = "2023, Litestar Organization"
author = "Litestar Organization"
release = "1.0.0"

extensions = [
    "sphinx.ext.intersphinx",
    "sphinx.ext.autosectionlabel",
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
    "sphinx_design",
    "sphinx_copybutton",
    "sphinxcontrib.mermaid",
    "myst_parser",
]

exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
}

napoleon_google_docstring = True
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = True
napoleon_use_admonition_for_notes = True
napoleon_use_admonition_for_references = False
napoleon_attr_annotations = True

autoclass_content = "class"
autodoc_class_signature = "separated"
autodoc_default_options = {
    "special-members": True,
    "show-inheritance": True,
    "members": True,
    "exclude-members": "__init_subclass__,__weakref__,__subclasshook__",
}
autodoc_member_order = "bysource"
autodoc_typehints_format = "short"

autosectionlabel_prefix_document = True

suppress_warnings = [
    "autosectionlabel.*",
    "ref.python",  # TODO: remove when https://github.com/sphinx-doc/sphinx/issues/4961 is fixed
]

html_theme = "shibuya"
html_static_path = ["_static"]
html_show_sourcelink = False
html_title = "Fast Query Parsers"

PY_CLASS = "py:class"
PY_RE = r"py:.*"

nitpicky = True
nitpick_ignore = []
nitpick_ignore_regex = []

# -- Style configuration -----------------------------------------------------
html_theme = "shibuya"
html_static_path = ["_static"]
html_css_files = ["css/custom.css"]
html_show_sourcelink = True
html_title = "Docs"
html_favicon = "_static/logo.png"
html_logo = "_static/logo.png"
html_context = {
    "source_type": "github",
    "source_user": "litestar-org",
    "source_repo": "fast-query-parsers",
}

brand_colors = {
    "--brand-primary": {"rgb": "237, 182, 65", "hex": "#edb641"},
    "--brand-secondary": {"rgb": "32, 34, 53", "hex": "#202235"},
    "--brand-tertiary": {"rgb": "161, 173, 161", "hex": "#A1ADA1"},
    "--brand-green": {"rgb": "0, 245, 151", "hex": "#00f597"},
    "--brand-alert": {"rgb": "243, 96, 96", "hex": "#f36060"},
    "--brand-dark": {"rgb": "0, 0, 0", "hex": "#000000"},
    "--brand-light": {"rgb": "235, 221, 221", "hex": "#ebdddd"},
}

html_theme_options = {
    "logo_target": "/",
    "announcement": "This documentation is currently under development.",
    "github_url": "https://github.com/litestar-org/fast-query-parsers",
    "nav_links": [
        {"title": "Home", "url": "https://fqp.litestar.dev"},
        {"title": "Docs", "url": "https://docs.fqp.litestar.dev"},
        {"title": "Code", "url": "https://github.com/litestar-org/fast-query-parsers"},
    ],
    "light_css_variables": {
        # RGB
        "--sy-rc-theme": brand_colors["--brand-primary"]["rgb"],
        "--sy-rc-text": brand_colors["--brand-primary"]["rgb"],
        "--sy-rc-invert": brand_colors["--brand-primary"]["rgb"],
        # "--sy-rc-bg": brand_colors["--brand-secondary"]["rgb"],
        # Hex
        "--sy-c-link": brand_colors["--brand-secondary"]["hex"],
        # "--sy-c-foot-bg": "#191919",
        "--sy-c-foot-divider": brand_colors["--brand-primary"]["hex"],
        "--sy-c-foot-text": brand_colors["--brand-dark"]["hex"],
        "--sy-c-bold": brand_colors["--brand-primary"]["hex"],
        "--sy-c-heading": brand_colors["--brand-primary"]["hex"],
        "--sy-c-text-weak": brand_colors["--brand-primary"]["hex"],
        "--sy-c-text": brand_colors["--brand-dark"]["hex"],
        "--sy-c-bg-weak": brand_colors["--brand-dark"]["rgb"],
    },
    "dark_css_variables": {
        # RGB
        "--sy-rc-theme": brand_colors["--brand-primary"]["rgb"],
        "--sy-rc-text": brand_colors["--brand-primary"]["rgb"],
        "--sy-rc-invert": brand_colors["--brand-primary"]["rgb"],
        "--sy-rc-bg": brand_colors["--brand-dark"]["rgb"],
        # Hex
        "--sy-c-link": brand_colors["--brand-primary"]["hex"],
        "--sy-c-foot-bg": brand_colors["--brand-dark"]["hex"],
        "--sy-c-foot-divider": brand_colors["--brand-primary"]["hex"],
        "--sy-c-foot-text": brand_colors["--brand-light"]["hex"],
        "--sy-c-bold": brand_colors["--brand-primary"]["hex"],
        "--sy-c-heading": brand_colors["--brand-primary"]["hex"],
        "--sy-c-text-weak": brand_colors["--brand-primary"]["hex"],
        "--sy-c-text": brand_colors["--brand-light"]["hex"],
        "--sy-c-bg-weak": brand_colors["--brand-dark"]["hex"],
        "--sy-c-bg": brand_colors["--brand-primary"]["hex"],
    },
}
