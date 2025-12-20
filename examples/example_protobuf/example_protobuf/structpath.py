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
LIB = Path(__file__).parent


class example_protobuf:

    class Person:
        """
        Protobuf message type: Person
        """

        @staticmethod
        def get_value(expr: IntoExprColumn, path: str) -> "Expr":
            """
            Extract a field from a binary protobuf column of type Person

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
                function_name="person_get_value",
                is_elementwise=True,
            )

__all__ = [
    "example_protobuf",
]
