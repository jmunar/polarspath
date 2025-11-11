"""
Auto-generated Python module for protobuf message access.
Provides class-based API like sample.User.get_value()
"""

from pathlib import Path
from typing import TYPE_CHECKING

from polars._typing import IntoExprColumn
from polars.plugins import register_plugin_function

if TYPE_CHECKING:
    from polars import Expr

# Get the path to the compiled library
# Polars expects either a directory or the full path to the .so file
# Using the directory should work, but if it doesn't, uncomment the line below:
# LIB = LIB_DIR / "protobuf_sample.abi3.so"
LIB = Path(__file__).parent


class sample:

    class User:
        """
        Protobuf message type: User
        """

        @staticmethod
        def get_value(expr: IntoExprColumn, path: str) -> "Expr":
            """
            Extract a field from a binary protobuf column of type User

            Args:
                expr: Polars expression or column name
                path: Field path to extract (e.g., 'name', 'pets[0].name')

            Returns:
                Polars expression
            """
            return register_plugin_function(
                args=[expr],
                kwargs={"path": path},
                plugin_path=LIB,
                function_name="user_get_value",
                is_elementwise=True,
            )
    class Group:
        """
        Protobuf message type: Group
        """

        @staticmethod
        def get_value(expr: IntoExprColumn, path: str) -> "Expr":
            """
            Extract a field from a binary protobuf column of type Group

            Args:
                expr: Polars expression or column name
                path: Field path to extract (e.g., 'name', 'pets[0].name')

            Returns:
                Polars expression
            """
            return register_plugin_function(
                args=[expr],
                kwargs={"path": path},
                plugin_path=LIB,
                function_name="group_get_value",
                is_elementwise=True,
            )

__all__ = [
    "sample",
]
