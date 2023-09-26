"""
Dummy file for code completion
"""


class StandaloneRustAppError(RuntimeError):
    def __init__(self, *args, **kwargs):
        super().__init__('This file cannot be run on its own!')


if True:
    raise StandaloneRustAppError
else:
    pass


class Functions:
    def new(self):
        def wrapper(*args):
            print(*args)
        return wrapper


func = Functions

get_pkg_ver = func.new()

hide_console_window = func.new()

pip_install = func.new()

get_test_text = func.new()