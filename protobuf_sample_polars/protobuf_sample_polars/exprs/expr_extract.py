"""
Expressions to extract fields from a binary protobuf column
"""

from pathlib import Path

import polars as pl
from polars._typing import IntoExprColumn
from polars.plugins import register_plugin_function

LIB = Path(__file__).parent.parent


def protobuf_user_extract(expr: IntoExprColumn, path: str) -> pl.Expr:
    """
    Extract a field from a binary protobuf column of type User
    """
    return register_plugin_function(
        args=[expr],
        kwargs={"path": path},
        plugin_path=LIB,
        function_name="user_extract",
        is_elementwise=True,
    )
